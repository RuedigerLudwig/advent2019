use crate::card_error::CardError;
use common::{
    as_long,
    math::{modulus_exp, modulus_inv, modulus_mul},
};

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

    pub fn parse(lines: &[String]) -> Result<Vec<Techniques>, CardError> {
        Ok(lines
            .iter()
            .map(|s| Techniques::parse_single(s))
            .collect::<Result<Vec<Techniques>, _>>()?)
    }

    fn parse_single(line: &str) -> Result<Self, CardError> {
        if line == "deal into new stack" {
            return Ok(Techniques::DealIntoNewStack);
        } else if line.starts_with("cut ") {
            if let Ok(number) = as_long(&line[4..]) {
                return Ok(Techniques::Cut(number));
            }
        } else if line.starts_with("deal with increment ") {
            if let Ok(number) = as_long(&line[20..]) {
                return Ok(Techniques::DealWithIncrement(number));
            }
        }
        Err(CardError::UnknownTechnique(line.to_owned()))
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
        let (mul, add) = techniques.iter().fold(Ok((1, 0)), |next, technique| {
            if let Ok((mul, add)) = next {
                let (m2, a2) = technique.get_params();
                if let Some(invers) = modulus_inv(m2, deck_size) {
                    let new_mul = modulus_mul(mul, invers, deck_size);
                    let new_add = modulus_mul(add + a2, m2, deck_size);
                    Ok((new_mul, new_add))
                } else {
                    Err(CardError::NotCoprime(m2, deck_size))
                }
            } else {
                next
            }
        })?;

        if let Some(inv_mul) = modulus_inv(mul, deck_size) {
            let fixpoint =
                modulus_inv(inv_mul - 1, deck_size).map(|tmp| modulus_mul(tmp, add, deck_size));
            Ok(CardShuffle {
                deck_size,
                fixpoint,
                mul,
                add,
            })
        } else {
            // This was checked beforehand, but we need to check anyway, so let#s just throw this error here
            Err(CardError::NotCoprime(mul, deck_size))
        }
    }

    pub fn invert(&self) -> Result<CardShuffle, CardError> {
        self.repeat(-1)
    }

    pub fn repeat(&self, times: i64) -> Result<CardShuffle, CardError> {
        if let Some(fixpoint) = self.fixpoint {
            let times = times.rem_euclid(self.deck_size - 1);
            let mul = modulus_exp(self.mul, times, self.deck_size);

            // I am sure I have a fixpoint, so the unwrap will always work safely
            let inv_mul = modulus_inv(mul, self.deck_size).unwrap();
            let add = modulus_mul(inv_mul - 1, fixpoint, self.deck_size);

            Ok(CardShuffle {
                deck_size: self.deck_size,
                fixpoint: self.fixpoint,
                mul,
                add,
            })
        } else {
            Err(CardError::NotImplemented)
        }
    }

    pub fn get_position_of_card(&self, card: i64) -> i64 {
        let number = card + self.add;
        modulus_mul(number, self.mul, self.deck_size).rem_euclid(self.deck_size)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::read_all_lines;
    use std::error::Error;

    #[test]
    fn simple_parse() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day22", "example2.txt")?;
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
    fn test_new_stack_forward() -> Result<(), Box<dyn Error>> {
        let shuffle = CardShuffle::create(&vec![Techniques::DealIntoNewStack], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_new_cut3_forward() -> Result<(), Box<dyn Error>> {
        let shuffle = CardShuffle::create(&vec![Techniques::Cut(3)], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_new_deal_with_increment3() -> Result<(), Box<dyn Error>> {
        let shuffle = CardShuffle::create(&vec![Techniques::DealWithIncrement(3)], 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_two_techniques() -> Result<(), Box<dyn Error>> {
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
    fn test_repeat() -> Result<(), Box<dyn Error>> {
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
    fn test_repeat_back() -> Result<(), Box<dyn Error>> {
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
    fn test_example1() -> Result<(), Box<dyn Error>> {
        let input = Techniques::parse(&read_all_lines("day22", "example1.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<(), Box<dyn Error>> {
        let input = Techniques::parse(&read_all_lines("day22", "example2.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_example4_forward() -> Result<(), Box<dyn Error>> {
        let input = Techniques::parse(&read_all_lines("day22", "example4.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10)?;

        let result = (0..10)
            .map(|number| shuffle.get_position_of_card(number))
            .collect::<Vec<_>>();

        let expected = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_repeat_simple() -> Result<(), Box<dyn Error>> {
        let input = Techniques::parse(&read_all_lines("day22", "input.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10_007)?;
        let shuffle2 = shuffle.repeat(1)?;

        assert_eq!(shuffle, shuffle2);

        Ok(())
    }

    #[test]
    fn test_double_inv() -> Result<(), Box<dyn Error>> {
        let input = Techniques::parse(&read_all_lines("day22", "input.txt")?)?;
        let shuffle = CardShuffle::create(&input, 10_007)?;
        let shuffle2 = shuffle.invert()?.invert()?;

        assert_eq!(shuffle, shuffle2);

        Ok(())
    }
}
