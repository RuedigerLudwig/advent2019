#![allow(dead_code)]

use std::fmt::Display;

use common::Turn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Path {
    Left(usize),
    Right(usize),
    Function(usize),
}

impl Path {
    pub fn new(turn: Turn, steps: usize) -> Path {
        match turn {
            Turn::Right => Path::Right(steps),
            Turn::Left => Path::Left(steps),
            _ => panic!("Illegal turn for path {:?}", turn),
        }
    }

    pub fn is_function(&self) -> bool {
        if let Path::Function(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_non_function(&self) -> bool {
        if let Path::Function(_) = self {
            false
        } else {
            true
        }
    }

    pub fn as_string(path: &[Path]) -> String {
        let mut result = String::with_capacity(25);
        let mut iter = path.iter();
        if let Some(first) = iter.next() {
            result += &first.to_string();
            for next in iter {
                result += ",";
                result += &next.to_string();
            }
        }
        result
    }

    pub fn extract_equal_parts(path: &[Path], level: usize) -> Option<(Vec<Path>, Vec<Vec<Path>>)> {
        if level == 0 || path.is_empty() {
            return None;
        }

        let max = path.len().min(6);
        for len in (1..max).rev() {
            let to_check = &path[..len];
            if to_check.iter().any(|p| p.is_function()) {
                continue;
            }
            if Path::as_string(to_check).len() > 25 {
                continue;
            }

            let pm = PathMatcher::new(path, to_check, level);
            for attempt in pm {
                if let Some(pos) = attempt.iter().position(Path::is_non_function) {
                    if let Some((functions, result)) =
                        Path::extract_equal_parts(&attempt[pos..], level - 1)
                    {
                        let mut rest_result = Vec::new();
                        rest_result.extend(&attempt[..pos]);
                        rest_result.extend(functions);
                        let mut movements = result.clone();
                        movements.push(to_check.iter().copied().collect::<Vec<_>>());
                        return Some((rest_result, movements));
                    }
                } else {
                    let mut result = Vec::new();
                    result.push(to_check.iter().copied().collect::<Vec<_>>());
                    return Some((attempt, result));
                }
            }
        }

        None
    }

    fn find_all_sub_vecs<T: PartialEq>(sub: &[T], full: &[T]) -> Vec<usize> {
        let len = sub.len();
        let mut result = Vec::new();
        if len <= full.len() {
            for i in 0..full.len() - len + 1 {
                if sub.eq(&full[i..i + len]) {
                    result.push(i);
                }
            }
        }
        result
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Left(steps) => write!(f, "L,{}", steps),
            Path::Right(steps) => write!(f, "R,{}", steps),
            Path::Function(1) => write!(f, "A"),
            Path::Function(2) => write!(f, "B"),
            Path::Function(3) => write!(f, "C"),
            Path::Function(_) => write!(f, "?"),
        }
    }
}

struct PathMatcher<'a> {
    level: usize,
    original: &'a [Path],
    sub: &'a [Path],
    possible_cuts: Vec<usize>,
    use_cuts: Option<Vec<bool>>,
}

impl<'a> PathMatcher<'a> {
    pub fn new(original: &'a [Path], sub: &'a [Path], level: usize) -> PathMatcher<'a> {
        let possible = Path::find_all_sub_vecs(sub, original);
        let mut current_cuts = vec![false].repeat(possible.len());
        current_cuts[0] = true;

        PathMatcher {
            level,
            original,
            sub,
            possible_cuts: possible,
            use_cuts: Some(current_cuts),
        }
    }

    fn inc_use_cuts(&mut self) -> bool {
        if let Some(use_cuts) = &self.use_cuts {
            let mut next_cuts = use_cuts.clone();
            for run in 1..next_cuts.len() {
                next_cuts[run] = !next_cuts[run];
                if next_cuts[run] {
                    self.use_cuts = Some(next_cuts);
                    return true;
                }
            }
            self.use_cuts = None;
        }
        return false;
    }

    fn calc_actual_cuts(&self) -> Option<Vec<usize>> {
        if let Some(use_cuts) = &self.use_cuts {
            let used = use_cuts
                .iter()
                .zip(&self.possible_cuts)
                .filter(|(curr, _)| **curr)
                .map(|(_, pos)| *pos)
                .collect::<Vec<_>>();
            for i in 1..used.len() {
                for j in 0..i {
                    if used[j] + self.sub.len() > used[i] {
                        return None;
                    }
                }
            }
            Some(used)
        } else {
            None
        }
    }

    fn get_actual_cuts(&mut self) -> Option<Vec<usize>> {
        loop {
            if let Some(to_be_used) = self.calc_actual_cuts() {
                return Some(to_be_used);
            }
            if !self.inc_use_cuts() {
                return None;
            }
        }
    }
}

impl<'a> Iterator for PathMatcher<'a> {
    type Item = Vec<Path>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(actual_cuts) = self.get_actual_cuts() {
            let mut result = Vec::new();
            let mut actual_cuts = actual_cuts.iter();
            let mut last_cut = *actual_cuts.next().unwrap() + self.sub.len();
            for cut in actual_cuts {
                result.push(Path::Function(self.level));
                result.extend(&self.original[last_cut..*cut]);
                last_cut = cut + self.sub.len();
            }
            result.push(Path::Function(self.level));
            if last_cut < self.original.len() {
                result.extend(&self.original[last_cut..]);
            }
            self.inc_use_cuts();
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Path::*;

    #[test]
    fn test_simpl_path() {
        let path = vec![Left(1), Left(1), Right(1), Left(1), Left(1), Left(1)];
        let sub = vec![Left(1), Left(1)];
        let pm = PathMatcher::new(&path, &sub, 3);
        let expected = vec![
            vec![Function(3), Right(1), Left(1), Left(1), Left(1)],
            vec![Function(3), Right(1), Function(3), Left(1)],
            vec![Function(3), Right(1), Left(1), Function(3)],
        ];
        let result = pm.collect::<Vec<_>>();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_in_vec() {
        let long = vec![0, 1, 2, 3, 4, 5];
        let expected = vec![2];
        let result = Path::find_all_sub_vecs(&[2, 3], &long);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_in_vec2() {
        let long = vec![0, 1, 2, 3, 4, 5];
        let expected = vec![4];
        let result = Path::find_all_sub_vecs(&[4, 5], &long);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_in_vec3() {
        let long = vec![0, 1];
        let expected: Vec<usize> = Vec::new();
        let result = Path::find_all_sub_vecs(&[0, 1, 2], &long);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_in_vec4() {
        let long = vec![0, 1];
        let expected = vec![0];
        let result = Path::find_all_sub_vecs(&[0, 1], &long);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_in_vec5() {
        let long = vec![0, 1, 2, 3, 4, 5];
        let expected: Vec<usize> = Vec::new();
        let result = Path::find_all_sub_vecs(&[0, 1, 3], &long);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_all_vec_in_vec() {
        let long = vec![1, 1, 1, 2, 1, 2, 1, 1];
        let expected = vec![0, 1, 6];
        let result = Path::find_all_sub_vecs(&[1, 1], &long);

        assert_eq!(expected, result);
    }
}
