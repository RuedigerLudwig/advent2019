use std::{fmt::Display, str::FromStr};

use crate::error::FftError;

#[derive(Debug)]
pub struct Transmission {
    values: Vec<u8>,
    offset: usize,
}

impl Transmission {
    fn calc_next_offset(&self) -> usize {
        let mut result: usize = 0;
        for i in 0..7 {
            result = result * 10 + (self.values[i] as usize);
        }
        result
    }

    fn get_pattern(&self, input_pos: usize, output_pos: usize) -> (usize, usize) {
        let in_pos = self.offset + input_pos + 1;
        let out_pos = self.offset + output_pos + 1;

        let pattern = (in_pos / out_pos) % 4;
        let len = out_pos - (in_pos % out_pos);

        (pattern, len)
    }

    fn run_phases(&self, steps: usize) -> Transmission {
        let len = self.values.len();
        let mut values = self.values.clone();

        let thorough_until = (len + self.offset) / 2;
        let quick_starts = if len > thorough_until {
            len - thorough_until
        } else {
            0
        };

        for _ in 0..steps {
            for output_pos in 0..quick_starts {
                let mut sum = 0i64;
                let mut input_pos = output_pos;
                while input_pos < len {
                    let (pattern, pattern_len) = self.get_pattern(input_pos, output_pos);
                    let end_pos = len.min(input_pos + pattern_len);

                    match pattern {
                        1 => {
                            sum += values[input_pos..end_pos]
                                .iter()
                                .map(|&n| n as i64)
                                .sum::<i64>()
                        }
                        3 => {
                            sum -= values[input_pos..end_pos]
                                .iter()
                                .map(|&n| n as i64)
                                .sum::<i64>()
                        }
                        _ => (),
                    }
                    input_pos = end_pos;
                }
                values[output_pos] = (sum % 10).abs() as u8;
            }

            for output_pos in (quick_starts..len - 1).rev() {
                values[output_pos] = (values[output_pos] + values[output_pos + 1]) % 10;
            }
        }
        Transmission {
            values,
            offset: self.offset,
        }
    }

    fn blow_up(&self, repeat: usize, offset: usize) -> Transmission {
        let len = self.values.len();
        let offset_repeat = offset / len;
        let mut values = self.values[(offset % len)..].to_vec();
        values.extend(self.values.repeat(repeat - offset_repeat - 1));

        return Transmission { values, offset };
    }

    pub fn run_small(&self) -> Transmission {
        self.run_phases(100)
    }

    pub fn run_big(&self) -> Transmission {
        let skip = self.calc_next_offset();
        let big = self.blow_up(10_000, skip);
        big.run_phases(100)
    }
}

impl FromStr for Transmission {
    type Err = FftError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        for digit in input.trim().chars() {
            if let Some(number) = digit.to_digit(10) {
                values.push(number as u8);
            } else {
                return Err(FftError::NotADigit(digit));
            }
        }
        Ok(Transmission { values, offset: 0 })
    }
}

impl Display for Transmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.values.iter().map(|&i| i.to_string()).take(8).collect();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), FftError> {
        let input = "12345678";
        let fft: Transmission = input.parse()?;
        let result = fft.to_string();
        let expected = "12345678";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_step() -> Result<(), FftError> {
        let input = "12345678";
        let fft: Transmission = input.parse()?;
        let result = fft.run_phases(1).to_string();
        let expected = "48226158";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_step_4() -> Result<(), FftError> {
        let input = "80871224585914546619083218645595";
        let fft: Transmission = input.parse()?;
        let result = &fft.run_phases(100).to_string()[..8];
        let expected = "24176176";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_large_100() -> Result<(), FftError> {
        let input = "12345678";
        let fft: Transmission = input.parse()?;
        let result = fft.run_phases(4).to_string();
        let expected = "01029498";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_get_offset() -> Result<(), FftError> {
        let input = "03036732577212944063491565474664";
        let fft: Transmission = input.parse()?;
        let expected = 303673;
        let result = fft.calc_next_offset();

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_repeat_pattern() -> Result<(), FftError> {
        let input = "12345678";
        let fft: Transmission = input.parse()?;
        let expected = (1, 4);
        let result = fft.get_pattern(3, 3);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_repeat_pattern2() -> Result<(), FftError> {
        let input = "12345678";
        let fft: Transmission = input.parse()?;
        let expected = (0, 2);
        let result = fft.get_pattern(2, 4);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_repeat_pattern3() -> Result<(), FftError> {
        let input = "12345678";
        let mut fft: Transmission = input.parse()?;
        fft.offset = 1;
        let expected = (0, 2);
        let result = fft.get_pattern(1, 3);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_big() -> Result<(), FftError> {
        let input = "03036732577212944063491565474664";
        let fft: Transmission = input.parse()?;
        let result = fft.run_big();
        let expected = "84462026";

        assert_eq!(result.to_string(), expected);

        Ok(())
    }
}
