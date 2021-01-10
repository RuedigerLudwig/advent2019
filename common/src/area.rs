use std::iter::FromIterator;

use crate::Pos;

pub trait SuccPrev: Ord + Copy {
    fn step(self, ascending: bool) -> Self;
}

impl SuccPrev for i32 {
    fn step(self, ascending: bool) -> Self {
        if ascending {
            self + 1
        } else {
            self - 1
        }
    }
}

impl SuccPrev for i64 {
    fn step(self, ascending: bool) -> Self {
        if ascending {
            self + 1
        } else {
            self - 1
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
}

impl<_T> FromIterator<Pos<_T>> for Area<_T>
where
    _T: Ord + Copy,
{
    fn from_iter<T: IntoIterator<Item = Pos<_T>>>(iter: T) -> Self {
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

impl<T> Area<T>
where
    T: SuccPrev,
{
    pub fn rows(&self, ascending: bool) -> RowIterator<T> {
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

pub struct RowIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
    ascending: bool,
}

impl<'a, T> Iterator for RowIterator<'a, T>
where
    T: SuccPrev,
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
            self.row = self.row.step(self.ascending);
            Some(row)
        } else {
            None
        }
    }
}

pub struct Row<'a, T> {
    area: &'a Area<T>,
    row: T,
}

impl<'a, T> Row<'a, T>
where
    T: SuccPrev,
{
    pub fn cols(&self, ascending: bool) -> ColIterator<T> {
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

pub struct ColIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
    col: T,
    ascending: bool,
}

impl<'a, T> Iterator for ColIterator<'a, T>
where
    T: SuccPrev,
{
    type Item = Pos<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.col <= self.area.upper_right.x())
            || (!self.ascending && self.col >= self.area.lower_left.x())
        {
            let pos = Pos::new(self.col, self.row);
            self.col = self.col.step(self.ascending);
            Some(pos)
        } else {
            None
        }
    }
}
