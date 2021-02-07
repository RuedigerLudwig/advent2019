use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Picture<'a> {
    pixels: &'a str,
    width: usize,
    height: usize,
    layers: usize,
}

impl<'a> Picture<'a> {
    pub fn new(pixels: &'a str, width: usize, height: usize) -> Picture<'a> {
        Picture {
            pixels,
            width,
            height,
            layers: pixels.chars().count() / (width * height),
        }
    }
}

impl Picture<'_> {
    pub fn count_number_per_layer(&self) -> Vec<HashMap<char, usize>> {
        let mut result = Vec::new();
        let mut chars = self.pixels.chars();

        for _ in 0..self.layers {
            let mut map = HashMap::new();
            for _ in 0..(self.height * self.width) {
                let pixel = chars
                    .next()
                    .expect("We made sure that we have the right amount of chars");
                let count = map.entry(pixel).or_insert(0);
                *count = *count + 1;
            }
            result.push(map);
        }

        result
    }

    pub fn get_magic_number(&self) -> Option<usize> {
        let info = self.count_number_per_layer();
        if let Some(layer) = info.iter().min_by_key(|&pixel_info| pixel_info[&'0']) {
            let one = layer.get(&'1');
            let two = layer.get(&'2');
            one.zip(two).map(|(&a, &b)| a * b)
        } else {
            None
        }
    }

    fn display(&self) -> String {
        let mut result = (0..self.height)
            .map(|_| ["?"].repeat(self.width))
            .collect::<Vec<_>>();

        let mut chars = self.pixels.chars();
        for _ in 0..self.layers {
            for row in 0..self.height {
                for col in 0..self.width {
                    let pixel = chars
                        .next()
                        .expect("We made sure we have enough chars stored");
                    if result[row][col] == "?" {
                        match pixel {
                            '0' => result[row][col] = " ",
                            '1' => result[row][col] = "#",
                            _ => (),
                        }
                    }
                }
            }
        }

        result
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for Picture<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use common::hashmap;

    use super::*;

    #[test]
    fn test_count() {
        let input = "111222122201";
        let expected = vec![hashmap!('1'=>3, '2'=>3), hashmap!('0'=> 1, '1'=>2, '2'=>3)];
        let picture = Picture::new(input, 3, 2);
        let result = picture.count_number_per_layer();
        assert_eq!(result, expected);
    }
}
