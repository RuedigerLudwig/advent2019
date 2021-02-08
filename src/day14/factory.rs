use super::error::FactoryError;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction<'a> {
    amount: i64,
    ingredients: Vec<(&'a str, i64)>,
}

impl<'a> Reaction<'a> {
    fn new(amount: i64, ingredients: Vec<(&'a str, i64)>) -> Reaction<'a> {
        Reaction {
            amount,
            ingredients,
        }
    }

    fn get_parts(input: &'a str) -> Result<(&'a str, i64), FactoryError> {
        let parts = input.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(FactoryError::IngredientError(input.to_owned()))
        } else {
            let num = parts[0].parse()?;
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

    pub fn parse(line: &'a str) -> Result<(&'a str, Reaction<'_>), FactoryError> {
        let parts = line.split("=>").collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(FactoryError::ReactionError(line.to_owned()))
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
    pub fn new(input: &'a str) -> Result<Factory<'a>, FactoryError> {
        let reactions = input
            .lines()
            .map(|l| Reaction::parse(l))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Factory { reactions })
    }

    pub fn get_amount_for(&self, amount: i64, ingredient: &str) -> Option<Vec<(&'a str, i64)>> {
        self.reactions.get(ingredient).map(|reaction| {
            let packets = 1 + (amount - 1) / reaction.amount;
            if packets != 1 {
                reaction
                    .ingredients
                    .iter()
                    .map(|(key, value)| (*key, value * packets))
                    .collect::<Vec<_>>()
            } else {
                reaction.ingredients.clone()
            }
        })
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

        found
            .get("ORE")
            .map(|ore| ore.values().sum())
            .ok_or(FactoryError::NoOre)
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
    use crate::common::file::read_data;

    #[test]
    fn parse_simple() -> Result<(), FactoryError> {
        let input = String::from("10 ORE => 10 A");
        let expected = Reaction {
            amount: 10,
            ingredients: vec![("ORE", 10)],
        };
        let (_, result) = Reaction::parse(&input)?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_amount() -> Result<(), FactoryError> {
        let input = read_data("day14", "example1.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory
            .get_amount_for(1, "B")
            .expect("Should not fail because of input");
        let expected = vec![("ORE", 1)];
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_amount2() -> Result<(), FactoryError> {
        let input = read_data("day14", "example1.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory
            .get_amount_for(11, "A")
            .expect("Should not fail because of input");
        let expected = vec![("ORE", 20)];
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_ore_for_fuel2() -> Result<(), FactoryError> {
        let input = read_data("day14", "example2.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.ore_per_fuel(1)?;
        let expected = 165;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_ore_for_fuel3() -> Result<(), FactoryError> {
        let input = read_data("day14", "example3.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.ore_per_fuel(1)?;
        let expected = 13312;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_max_fuel() -> Result<(), FactoryError> {
        let input = read_data("day14", "example3.txt")?;
        let factory = Factory::new(&input)?;
        let result = factory.fuel_for_ore(1_000_000_000_000_i64)?;
        let expected = 82892753;
        assert_eq!(expected, result);

        Ok(())
    }
}
