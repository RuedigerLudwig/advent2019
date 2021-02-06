use std::str::FromStr;

use crate::error::CardError;
use common::math::{modulus_exp, modulus_inv, modulus_mul};

#[derive(Debug, PartialEq, Eq)]
pub enum Techniques {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(i64),
}

impl Techniques {
    fn get_params(&self) -> (i64, i64) {
        match *self {
            Techniques::DealIntoNewStack => (-1, -1),
            Techniques::Cut(cut) => (1, cut),
            Techniques::DealWithIncrement(increment) => (increment, 0),
        }
    }

    pub fn parse(lines: &str) -> Result<Vec<Techniques>, CardError> {
        Ok(lines
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<Techniques>, _>>()?)
    }
}

impl FromStr for Techniques {
    type Err = CardError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use Techniques::*;
        if line == "deal into new stack" {
            Ok(DealIntoNewStack)
        } else if line.starts_with("cut ") {
            Ok(Cut(line[4..].parse()?))
        } else if line.starts_with("deal with increment ") {
            Ok(DealWithIncrement(line[20..].parse()?))
        } else {
            Err(CardError::UnknownTechnique(line.to_owned()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardShuffle {
    deck_size: i64,
    fixpoint: Option<i64>,
    mul: i64,
    add: i64,
}

impl CardShuffle {
    pub fn create(techniques: &[Techniques], deck_size: i64) -> Result<CardShuffle, CardError> {
        if deck_size <= 0 {
            return Err(CardError::IllegalDeckSize(deck_size));
        }
        let (mul, add) = techniques
            .iter()
            .try_fold((1, 0), |(mul, add), technique| {
                let (m2, a2) = technique.get_params();

                modulus_inv(m2, deck_size)
                    .map(|invers| {
                        let new_mul = modulus_mul(mul, invers, deck_size);
                        let new_add = modulus_mul(add + a2, m2, deck_size);

                        (new_mul, new_add)
                    })
                    .ok_or(CardError::NotCoprime(m2, deck_size))
            })?;

        modulus_inv(mul, deck_size)
            .map(|inv_mul| {
                let fixpoint =
                    modulus_inv(inv_mul - 1, deck_size).map(|tmp| modulus_mul(tmp, add, deck_size));

                CardShuffle {
                    deck_size,
                    fixpoint,
                    mul,
                    add,
                }
            })
            .ok_or(CardError::NotCoprime(mul, deck_size))
    }

    pub fn invert(&self) -> Result<CardShuffle, CardError> {
        self.repeat(-1)
    }

    pub fn repeat(&self, times: i64) -> Result<CardShuffle, CardError> {
        self.fixpoint
            .map(|fixpoint| {
                let times = times.rem_euclid(self.deck_size - 1);
                let mul = modulus_exp(self.mul, times, self.deck_size);

                let inv_mul = modulus_inv(mul, self.deck_size)
                    .expect("I am sure I have a fixpoint, so this will always work safely");
                let add = modulus_mul(inv_mul - 1, fixpoint, self.deck_size);

                CardShuffle {
                    deck_size: self.deck_size,
                    fixpoint: self.fixpoint,
                    mul,
                    add,
                }
            })
            .ok_or(CardError::NotImplemented)
    }

    pub fn get_position_of_card(&self, card: i64) -> i64 {
        let number = card + self.add;
        modulus_mul(number, self.mul, self.deck_size).rem_euclid(self.deck_size)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::read_data;

    #[test]
    fn simple_parse() -> Result<(), CardError> {
        let input = read_data("day22", "example2.txt")?;
        let expected = vec![
            Techniques::Cut(6),
            Techniques::DealWithIncrement(7),
            Techniques::DealIntoNewStack,
        ];
        let result = Techniques::parse(&input)?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_new_stack_forward() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(&vec![Techniques::DealIntoNewStack], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_new_cut3_forward() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(&vec![Techniques::Cut(3)], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_new_deal_with_increment3() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(&vec![Techniques::DealWithIncrement(3)], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_two_techniques() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(
            &vec![Techniques::DealWithIncrement(3), Techniques::Cut(2)],
            10,
        )?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![4, 1, 8, 5, 2, 9, 6, 3, 0, 7];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_repeat() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(
            &vec![Techniques::DealWithIncrement(7), Techniques::Cut(5)],
            13,
        )?
        .repeat(4)?;

        let result = (0..13)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![7, 10, 0, 3, 6, 9, 12, 2, 5, 8, 11, 1, 4];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_repeat_back() -> Result<(), CardError> {
        let shuffle = CardShuffle::create(
            &vec![Techniques::DealWithIncrement(7), Techniques::Cut(5)],
            13,
        )?
        .invert()?
        .repeat(5)?;

        let result = (0..13)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![9, 7, 5, 3, 1, 12, 10, 8, 6, 4, 2, 0, 11];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_example1() -> Result<(), CardError> {
        let input = Techniques::parse(&read_data("day22", "example1.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<(), CardError> {
        let input = Techniques::parse(&read_data("day22", "example2.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_example4_forward() -> Result<(), CardError> {
        let input = Techniques::parse(&read_data("day22", "example4.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_repeat_simple() -> Result<(), CardError> {
        let input = Techniques::parse(&read_data("day22", "input.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10_007)?;
        let shuffle2 = shuffle.repeat(1)?;

        assert_eq!(shuffle, shuffle2);

        Ok(())
    }

    #[test]
    fn test_double_inv() -> Result<(), CardError> {
        let input = Techniques::parse(&read_data("day22", "input.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10_007)?;
        let shuffle2 = shuffle.invert()?.invert()?;

        assert_eq!(shuffle, shuffle2);

        Ok(())
    }
}
