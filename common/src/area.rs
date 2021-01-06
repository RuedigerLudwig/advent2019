use crate::Pos;

#[derive(Debug, Clone, Copy)]
pub struct Area<T>(Pos<T>, Pos<T>);

pub trait SuccPrev: Ord + Copy {
    fn succ(self) -> Self;
    fn prev(self) -> Self;
}

impl SuccPrev for i32 {
    fn succ(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
}

impl<T> Area<T>
where
    T: Ord + Copy,
{
    pub fn single(pos: Pos<T>) -> Area<T> {
        Area(pos, pos)
    }

    pub fn new(p1: Pos<T>, p2: Pos<T>) -> Area<T> {
        Area(
            Pos::new(p1.x().min(p2.x()), p1.y().max(p2.y())),
            Pos::new(p1.x().max(p2.x()), p1.y().min(p2.y())),
        )
    }

    pub fn extend(&self, pos: Pos<T>) -> Area<T> {
        Area(
            Pos::new(self.0.x().min(pos.x()), self.0.y().max(pos.y())),
            Pos::new(self.1.x().max(pos.x()), self.1.y().min(pos.y())),
        )
    }

    pub fn get_upper_left(&self) -> Pos<T> {
        self.0
    }

    pub fn get_lower_right(&self) -> Pos<T> {
        self.1
    }
}

impl<T> Area<T>
where
    T: SuccPrev,
{
    pub fn rows(&self) -> RowIterator<T> {
        RowIterator {
            area: self,
            row: self.0.y(),
        }
    }
}

pub struct RowIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
}

impl<'a, T> Iterator for RowIterator<'a, T>
where
    T: SuccPrev,
{
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.area.1.y() {
            let row = Row {
                area: self.area,
                row: self.row,
            };
            self.row = self.row.prev();
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
    pub fn cols(&self) -> ColIterator<T> {
        ColIterator {
            area: self.area,
            row: self.row,
            col: self.area.0.x(),
        }
    }
}

pub struct ColIterator<'a, T> {
    area: &'a Area<T>,
    row: T,
    col: T,
}

impl<'a, T> Iterator for ColIterator<'a, T>
where
    T: SuccPrev,
{
    type Item = Pos<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.col <= self.area.1.x() {
            let pos = Pos::new(self.col, self.row);
            self.col = self.col.succ();
            Some(pos)
        } else {
            None
        }
    }
}
