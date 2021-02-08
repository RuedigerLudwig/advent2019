use std::{cell::RefCell, rc::Rc};

pub trait PermutateExt<T>: IntoIterator<Item = T> {
    fn permutate(&self) -> Permutations<'_, T>;
}

impl<T> PermutateExt<T> for Vec<T> {
    fn permutate(&self) -> Permutations<'_, T> {
        let list = self.into_iter().collect::<Vec<_>>();
        Permutations {
            list: Rc::new(RefCell::new(list)),

            start: 0,
            current: 0,
            len: self.len(),
            maybe_tail: None,
        }
    }
}

pub struct Permutations<'a, T> {
    list: Rc<RefCell<Vec<&'a T>>>,
    start: usize,
    current: usize,
    len: usize,
    maybe_tail: Option<Box<Permutations<'a, T>>>,
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.current >= self.len {
            None
        } else if self.start + 1 == self.len {
            self.current += 1;
            Some(self.list.borrow().clone())
        } else {
            if let Some(mut tail) = self.maybe_tail.take() {
                if let Some(result) = tail.next() {
                    self.maybe_tail = Some(tail);
                    return Some(result);
                } else {
                    let mut borrow = (*self.list).borrow_mut();
                    // Swapping prev first item back to its original osition
                    for p in self.start..self.current {
                        borrow.swap(p, p + 1);
                    }

                    self.current += 1;
                    if self.current >= self.len {
                        return None;
                    }

                    // Getting next first item for next iteration
                    for p in (self.start..self.current).rev() {
                        borrow.swap(p, p + 1);
                    }
                }
            }

            let mut rest = Box::new(Permutations {
                len: self.len,
                list: self.list.clone(),
                current: self.start + 1,
                start: self.start + 1,
                maybe_tail: None,
            });
            let result = rest.next();
            self.maybe_tail = Some(rest);

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
        let result = input.permutate().collect::<Vec<_>>();
        let expected: Vec<Vec<&i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one() {
        let input = vec![1];
        let result = input.permutate().collect::<Vec<_>>();
        let expected = vec![[&1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![1, 2];
        let result = input.permutate().collect::<Vec<_>>();
        let expected = vec![[&1, &2], [&2, &1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three() {
        let input = vec![1, 2, 3];
        let result = input.permutate().collect::<Vec<_>>();
        let expected = vec![
            [&1, &2, &3],
            [&1, &3, &2],
            [&2, &1, &3],
            [&2, &3, &1],
            [&3, &1, &2],
            [&3, &2, &1],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_four() {
        let input = vec![1, 2, 3, 4];
        let result = input.permutate().collect::<Vec<_>>();
        let expected = vec![
            [&1, &2, &3, &4],
            [&1, &2, &4, &3],
            [&1, &3, &2, &4],
            [&1, &3, &4, &2],
            [&1, &4, &2, &3],
            [&1, &4, &3, &2],
            [&2, &1, &3, &4],
            [&2, &1, &4, &3],
            [&2, &3, &1, &4],
            [&2, &3, &4, &1],
            [&2, &4, &1, &3],
            [&2, &4, &3, &1],
            [&3, &1, &2, &4],
            [&3, &1, &4, &2],
            [&3, &2, &1, &4],
            [&3, &2, &4, &1],
            [&3, &4, &1, &2],
            [&3, &4, &2, &1],
            [&4, &1, &2, &3],
            [&4, &1, &3, &2],
            [&4, &2, &1, &3],
            [&4, &2, &3, &1],
            [&4, &3, &1, &2],
            [&4, &3, &2, &1],
        ];
        assert_eq!(result, expected);
    }
}
