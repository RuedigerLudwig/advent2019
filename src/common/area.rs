#![allow(dead_code)]
use std::fmt::Display;

use super::{number::Number, pos::Pos};

#[derive(Debug, Clone, Copy, Default)]
pub struct Area<T>
where
    T: Number,
{
    lower_left: Pos<T>,
    upper_right: Pos<T>,
}

impl<T> Area<T>
where
    T: Number + Ord,
{
    pub fn new(p1: Pos<T>, p2: Pos<T>) -> Area<T> {
        Area {
            lower_left: p1.min_components(&p2),
            upper_right: p1.max_components(&p2),
        }
    }

    pub fn extend(&self, pos: Pos<T>) -> Area<T> {
        if self.contains(pos) {
            return *self;
        }

        Area {
            lower_left: self.lower_left.min_components(&pos),
            upper_right: self.upper_right.max_components(&pos),
        }
    }

    pub fn get_lower_left(&self) -> Pos<T> {
        self.lower_left
    }

    pub fn get_upper_right(&self) -> Pos<T> {
        self.upper_right
    }

    pub fn contains(&self, pos: Pos<T>) -> bool {
        self.lower_left.x() >= pos.x()
            && pos.x() >= self.upper_right.x()
            && self.lower_left.y() >= pos.y()
            && pos.y() >= self.upper_right.y()
    }
}

impl<'a, T> Area<T>
where
    T: Number + Ord + 'a,
{
    pub fn from_iterator<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = &'a Pos<T>>,
    {
        let first = *iter.next()?;
        let (upper, lower) = iter.fold((first, first), |(mx, mn), p| {
            (mx.max_components(&p), mn.min_components(&p))
        });

        Some(Area::new(lower, upper))
    }
}

impl<T> Area<T>
where
    T: Number,
{
    pub fn width(&self) -> T {
        self.upper_right.x() - self.lower_left.x() + T::ONE
    }

    #[allow(dead_code)]
    pub fn height(&self) -> T {
        self.upper_right.y() - self.lower_left.y() + T::ONE
    }
}

impl<T> Area<T>
where
    T: Number,
{
    #[allow(dead_code)]
    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T> Display for Area<T>
where
    T: Number + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.lower_left, self.upper_right)
    }
}

impl<T> Area<T>
where
    T: Number,
{
    pub fn cells(&self, ascending: bool) -> CellIterator<'_, T> {
        CellIterator::new(self, ascending)
    }

    pub fn rows(&self, ascending: bool) -> RowIterator<'_, T> {
        RowIterator::new(self, ascending)
    }
}

#[derive(Debug)]
pub struct RowIterator<'a, T>
where
    T: Number,
{
    area: &'a Area<T>,
    row: T,
    ascending: bool,
}

impl<'a, T> RowIterator<'a, T>
where
    T: Number,
{
    fn new(area: &'a Area<T>, ascending: bool) -> RowIterator<'a, T> {
        RowIterator {
            area,
            row: if ascending {
                area.lower_left.y()
            } else {
                area.upper_right.y()
            },
            ascending,
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T>
where
    T: Number,
{
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.row <= self.area.upper_right.y())
            || (!self.ascending && self.row >= self.area.lower_left.y())
        {
            let row = Row {
                area: self.area,
                row: self.row,
            };
            if self.ascending {
                self.row += T::ONE;
            } else {
                self.row -= T::ONE;
            }
            Some(row)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Row<'a, T>
where
    T: Number,
{
    area: &'a Area<T>,
    row: T,
}

impl<'a, T> Row<'a, T>
where
    T: Number,
{
    pub fn cols(&self, ascending: bool) -> ColIterator<'_, T> {
        ColIterator {
            area: self.area,
            row: self.row,
            col: if ascending {
                self.area.lower_left.x()
            } else {
                self.area.upper_right.x()
            },
            ascending,
        }
    }
}

#[derive(Debug)]
pub struct ColIterator<'a, T>
where
    T: Number,
{
    area: &'a Area<T>,
    row: T,
    col: T,
    ascending: bool,
}

impl<'a, T> Iterator for ColIterator<'a, T>
where
    T: Number,
{
    type Item = Pos<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.col <= self.area.upper_right.x())
            || (!self.ascending && self.col >= self.area.lower_left.x())
        {
            let pos = Pos::new(self.col, self.row);
            if self.ascending {
                self.col += T::ONE
            } else {
                self.col -= T::ONE
            };
            Some(pos)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CellIterator<'a, T>
where
    T: Number,
{
    area: &'a Area<T>,
    row: T,
    col: T,
    ascending: bool,
}

impl<'a, T> CellIterator<'a, T>
where
    T: Number,
{
    pub fn new(area: &'a Area<T>, ascending: bool) -> CellIterator<'a, T> {
        let (col, row) = if ascending {
            (area.lower_left.x(), area.lower_left.y())
        } else {
            (area.upper_right.x(), area.upper_right.y())
        };
        CellIterator {
            area,
            row,
            col,
            ascending,
        }
    }
}

impl<'a, T> Iterator for CellIterator<'a, T>
where
    T: Number,
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.row <= self.area.upper_right.y())
            || (!self.ascending && self.row >= self.area.lower_left.y())
        {
            let pos = Pos::new(self.col, self.row);
            if self.ascending {
                self.col += T::ONE;
                if self.col > self.area.upper_right.x() {
                    self.row += T::ONE;
                    self.col = self.area.lower_left.x();
                }
            } else {
                self.col -= T::ONE;
                if self.col < self.area.lower_left.x() {
                    self.row -= T::ONE;
                    self.col = self.area.upper_right.x();
                }
            }

            Some(pos)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_iterator() {
        let area = Area::new(Pos::new(-1, -1), Pos::new(1, 1));
        let result = area.cells(true).collect::<Vec<_>>();
        let expected = vec![
            Pos::new(-1, -1),
            Pos::new(0, -1),
            Pos::new(1, -1),
            Pos::new(-1, 0),
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(-1, 1),
            Pos::new(0, 1),
            Pos::new(1, 1),
        ];
        assert_eq!(result, expected);
    }
}
