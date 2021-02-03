use std::{
    fmt::Display,
    iter::FromIterator,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::Pos;

pub trait HasOne:
    Copy + Ord + AddAssign + SubAssign + Add<Output = Self> + Sub<Output = Self>
{
    const ONE: Self;
}

impl HasOne for i32 {
    const ONE: i32 = 1;
}

impl HasOne for i64 {
    const ONE: i64 = 1;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Area<T> {
    lower_left: Pos<T>,
    upper_right: Pos<T>,
}

impl<T> Area<T>
where
    T: Ord + Copy,
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
    T: HasOne,
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
    T: HasOne + Mul<Output = T>,
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
    _T: Ord + Copy,
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
    _T: 'a + Ord + Copy,
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
    T: Copy,
{
    pub fn rows(&self, ascending: bool) -> RowIterator<'_, T> {
        RowIterator {
            area: self,
            row: if ascending {
                self.lower_left.y()
            } else {
                self.upper_right.y()
            },
            ascending,
        }
    }
}

#[derive(Debug)]
pub struct RowIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
    ascending: bool,
}

impl<'a, T> Iterator for RowIterator<'a, T>
where
    T: HasOne,
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
    pub fn cols(&self, ascending: bool) -> ColIterator<'a, T> {
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
    T: HasOne,
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
