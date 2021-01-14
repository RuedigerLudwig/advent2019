use std::collections::{HashMap, VecDeque};

use common::as_long;

use crate::factory_error::FactoryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction<'a> {
    _amount: i64,
    _ingredients: Vec<(&'a str, i64)>,
}

impl<'a> Reaction<'a> {
    pub fn new(amount: i64, _ingredients: Vec<(&'a str, i64)>) -> Reaction<'a> {
        Reaction {
            _amount: amount,
            _ingredients,
        }
    }

    fn get_parts(input: &'a str) -> Result<(&'a str, i64), FactoryError> {
        let parts: Vec<&str> = input.split(" ").collect();
        if parts.len() != 2 {
            Err(FactoryError::IngredientError(String::from(input)))
        } else {
            let num = as_long(parts[0])?;
            Ok((parts[1], num))
        }
    }

    fn get_ingredients(ingreditents: &'a str) -> Result<Vec<(&'a str, i64)>, FactoryError> {
        let parts = ingreditents.split(",");
        let result = parts
            .map(|input| Reaction::get_parts(input.trim()))
            .collect::<Result<_, _>>()?;
        Ok(result)
    }

    pub fn parse(line: &'a str) -> Result<(&'a str, Reaction), FactoryError> {
        let parts: Vec<&str> = line.split("=>").collect();
        if parts.len() != 2 {
            Err(FactoryError::ReactionError(String::from(line)))
        } else {
            let (name, amount) = Reaction::get_parts(parts[1].trim())?;
            let ingredients = Reaction::get_ingredients(parts[0].trim())?;

            Ok((name, Reaction::new(amount, ingredients)))
        }
    }
}

pub struct Factory<'a> {
    reactions: HashMap<&'a str, Reaction<'a>>,
}

impl<'a> Factory<'a> {
    pub fn new(lines: &'a [String]) -> Result<Factory<'a>, FactoryError> {
        let reactions = lines
            .iter()
            .map(|l| Reaction::parse(l))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Factory { reactions })
    }

    pub fn get_amount_for(&self, amount: i64, ingredient: &str) -> Option<Vec<(&'a str, i64)>> {
        let result = self.reactions.get(ingredient).map(|reaction| {
            let mut result = reaction._ingredients.clone();

            let packets = 1 + (amount - 1) / reaction._amount;
            if packets != 1 {
                result = result
                    .into_iter()
                    .map(|(key, value)| (key, value * packets))
                    .collect();
            }
            result
        });

        result
    }

    pub fn ore_per_fuel(&self, desired: i64) -> Result<i64, FactoryError> {
        let mut found = HashMap::new();
        let mut to_do = VecDeque::new();
        to_do.push_back(("FUEL", desired));

        while let Some((outcome, amount)) = to_do.pop_front() {
            if let Some(ingredients) = self.get_amount_for(amount, &outcome) {
                for (name, needed) in ingredients {
                    let react = found.entry(name).or_insert(HashMap::new());
                    let sum: i64 = react.values().sum();
                    let prev_needed = react.insert(outcome, needed).unwrap_or(0i64);
                    to_do.push_back((name, sum + needed - prev_needed));
                }
            }
        }

        if let Some(ore) = found.get("ORE") {
            let sum: i64 = ore.values().sum();
            Ok(sum)
        } else {
            Err(FactoryError::MessageError(String::from(
                "Did not produce ore",
            )))
        }
    }

    pub fn fuel_for_ore(&self, ore: i64) -> Result<i64, FactoryError> {
        let one_fuel = self.ore_per_fuel(1)?;
        let mut prev_ore = one_fuel;
        let mut prev_fuel = 1i64;

        let fuel = loop {
            let fuel = prev_fuel + (ore - prev_ore) / one_fuel;
            if fuel <= prev_fuel {
                break fuel;
            }

            prev_ore = self.ore_per_fuel(fuel)?;
            prev_fuel = fuel;
        };
        Ok(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_all_lines;

    #[test]
    fn parse_simple() -> Result<(), FactoryError> {
        let input = String::from("10 ORE => 10 A");
        let expected = Reaction {
            _amount: 10,
            _ingredients: vec![("ORE", 10)],
        };
        let (_, result) = Reaction::parse(&input)?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_amount() -> Result<(), FactoryError> {
        let input = read_all_lines("day14", "example1.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.get_amount_for(1, "B").unwrap();
        let expected = vec![("ORE", 1)];
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_amount2() -> Result<(), FactoryError> {
        let input = read_all_lines("day14", "example1.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.get_amount_for(11, "A").unwrap();
        let expected = vec![("ORE", 20)];
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_ore_for_fuel2() -> Result<(), FactoryError> {
        let input = read_all_lines("day14", "example2.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.ore_per_fuel(1)?;
        let expected = 165;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_ore_for_fuel3() -> Result<(), FactoryError> {
        let input = read_all_lines("day14", "example3.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.ore_per_fuel(1)?;
        let expected = 13312;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_max_fuel() -> Result<(), FactoryError> {
        let input = read_all_lines("day14", "example3.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.fuel_for_ore(1_000_000_000_000_i64)?;
        let expected = 82892753;
        assert_eq!(expected, result);

        Ok(())
    }
}
