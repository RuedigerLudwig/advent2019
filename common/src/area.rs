use crate::{math::Number, pos::Pos};
use std::{fmt::Display, iter::FromIterator};

#[derive(Debug, Clone, Copy, Default)]
pub struct Area<T> {
    lower_left: Pos<T>,
    upper_right: Pos<T>,
}

impl<T> Area<T>
where
    T: Number + Ord,
{
    pub fn single(pos: Pos<T>) -> Area<T> {
        Area {
            lower_left: pos,
            upper_right: pos,
        }
    }

    pub fn new(p1: Pos<T>, p2: Pos<T>) -> Area<T> {
        Area {
            lower_left: Pos::new(p1.x().min(p2.x()), p1.y().min(p2.y())),
            upper_right: Pos::new(p1.x().max(p2.x()), p1.y().max(p2.y())),
        }
    }

    pub fn extend(&self, pos: Pos<T>) -> Area<T> {
        Area {
            lower_left: Pos::new(
                self.lower_left.x().min(pos.x()),
                self.lower_left.y().min(pos.y()),
            ),
            upper_right: Pos::new(
                self.upper_right.x().max(pos.x()),
                self.upper_right.y().max(pos.y()),
            ),
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

impl<T> Area<T>
where
    T: Number,
{
    pub fn width(&self) -> T {
        self.upper_right.x() - self.lower_left.x() + T::ONE
    }

    pub fn height(&self) -> T {
        self.upper_right.y() - self.lower_left.y() + T::ONE
    }
}

impl<T> Area<T>
where
    T: Number,
{
    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<_T> Display for Area<_T>
where
    _T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.lower_left, self.upper_right)
    }
}

impl<_T> FromIterator<Pos<_T>> for Area<_T>
where
    _T: Number + Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Pos<_T>>,
    {
        let mut iter = iter.into_iter();
        if let Some(pos) = iter.next() {
            let mut area = Area::single(pos);
            while let Some(pos) = iter.next() {
                area = area.extend(pos);
            }
            area
        } else {
            panic!("Need to have at least one position for an area");
        }
    }
}

impl<'a, _T> FromIterator<&'a Pos<_T>> for Area<_T>
where
    _T: 'a + Number + Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a Pos<_T>>,
    {
        let mut iter = iter.into_iter();
        if let Some(&pos) = iter.next() {
            let mut area = Area::single(pos);
            while let Some(&pos) = iter.next() {
                area = area.extend(pos);
            }
            area
        } else {
            panic!("Need to have at least one position for an area");
        }
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
pub struct RowIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
    ascending: bool,
}

impl<'a, T> RowIterator<'a, T>
where
    T: Copy,
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
pub struct Row<'a, T> {
    area: &'a Area<T>,
    row: T,
}

impl<'a, T> Row<'a, T>
where
    T: Copy,
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
pub struct ColIterator<'a, T> {
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
pub struct CellIterator<'a, T> {
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
