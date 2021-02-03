use std::{cell::RefCell, rc::Rc};

pub struct LexPermutations<T> {
    _start: usize,
    _current: usize,
    _len: usize,
    _list: Rc<RefCell<Vec<T>>>,
    _tail: Option<Box<LexPermutations<T>>>,
}

impl<T: Clone + Sized> LexPermutations<T> {
    pub fn new(lst: &[T]) -> LexPermutations<T> {
        let list = lst.to_owned();
        LexPermutations {
            _start: 0,
            _current: 0,
            _len: list.len(),

            _list: Rc::new(RefCell::new(list)),
            _tail: None,
        }
    }
}

impl<T: Clone + Copy> Iterator for LexPermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self._current >= self._len {
            None
        } else if self._start + 1 == self._len {
            self._current += 1;
            Some(self._list.borrow().clone())
        } else {
            if let Some(mut rest) = self._tail.take() {
                if let Some(result) = rest.next() {
                    self._tail = Some(rest);
                    return Some(result);
                } else {
                    let mut borrow = (*self._list).borrow_mut();
                    for p in self._start..self._current {
                        borrow.swap(p, p + 1);
                    }

                    self._current += 1;
                    if self._current >= self._len {
                        return None;
                    }

                    for p in (self._start..self._current).rev() {
                        borrow.swap(p, p + 1);
                    }
                }
            }

            let mut rest = Box::new(LexPermutations {
                _len: self._len,
                _list: self._list.clone(),
                _current: self._start + 1,
                _start: self._start + 1,
                _tail: None,
            });
            let result = rest.next();
            self._tail = Some(rest);

            result
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_zero() {
        let input: Vec<i32> = vec![];
        let result: Vec<Vec<i32>> = LexPermutations::new(&input).collect();
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one() {
        let input = vec![1];
        let result: Vec<Vec<i32>> = LexPermutations::new(&input).collect();
        let expected = vec![[1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![1, 2];
        let result: Vec<Vec<i32>> = LexPermutations::new(&input).collect();
        let expected = vec![[1, 2], [2, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three() {
        let input = vec![1, 2, 3];
        let result: Vec<Vec<i32>> = LexPermutations::new(&input).collect();
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

    #[test]
    fn test_four() {
        let input = vec![1, 2, 3, 4];
        let result: Vec<Vec<i32>> = LexPermutations::new(&input).collect();
        let expected = vec![
            [1, 2, 3, 4],
            [1, 2, 4, 3],
            [1, 3, 2, 4],
            [1, 3, 4, 2],
            [1, 4, 2, 3],
            [1, 4, 3, 2],
            [2, 1, 3, 4],
            [2, 1, 4, 3],
            [2, 3, 1, 4],
            [2, 3, 4, 1],
            [2, 4, 1, 3],
            [2, 4, 3, 1],
            [3, 1, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 1, 4],
            [3, 2, 4, 1],
            [3, 4, 1, 2],
            [3, 4, 2, 1],
            [4, 1, 2, 3],
            [4, 1, 3, 2],
            [4, 2, 1, 3],
            [4, 2, 3, 1],
            [4, 3, 1, 2],
            [4, 3, 2, 1],
        ];
        assert_eq!(result, expected);
    }
}
