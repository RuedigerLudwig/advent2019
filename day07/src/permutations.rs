pub struct Permutations<T> {
    _lst: Vec<T>,
    _current: usize,
    _rest: Option<Box<Permutations<T>>>,
}

impl<T: Clone + Copy> Permutations<T> {
    pub fn new(lst: &Vec<T>) -> Permutations<T> {
        Permutations {
            _lst: lst.clone(),
            _current: 0,
            _rest: None,
        }
    }
}

impl<T: Clone + Copy> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut rest) = self._rest.take() {
            if let Some(next_rest) = &rest.next() {
                let mut result = vec![self._lst[self._current]];
                result.extend(next_rest);
                self._rest = Some(rest);
                return Some(result);
            }
            self._current += 1;
        }

        if self._current >= self._lst.len() {
            None
        } else if self._lst.len() == 1 {
            self._current += 1;
            Some(self._lst.clone())
        } else {
            let mut rest = self._lst.clone();
            rest.remove(self._current);
            let mut rest = Box::new(Permutations {
                _lst: rest,
                _current: 0,
                _rest: None,
            });
            if let Some(next_rest) = &rest.next() {
                let mut result = vec![self._lst[self._current]];
                result.extend(next_rest);
                self._rest = Some(rest);
                Some(result)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one() {
        let input = vec![1];
        let result: Vec<Vec<i32>> = Permutations::new(&input).collect();
        let expected = vec![[1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![1, 2];
        let result: Vec<Vec<i32>> = Permutations::new(&input).collect();
        let expected = vec![[1, 2], [2, 1]];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_three() {
        let input = vec![1, 2, 3];
        let result: Vec<Vec<i32>> = Permutations::new(&input).collect();
        let expected = vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        assert_eq!(result, expected);
    }
}
