use super::error::MapError;
use crate::common::{area::Area as RawArea, helper::zip6, pos::Pos as RawPos};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub type Pos = RawPos<i32>;
pub type Area = RawArea<i32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PortalType {
    Inner,
    Outer,
    Entrance,
    Exit,
}

impl PortalType {
    pub fn is_complement(&self, other: &PortalType) -> bool {
        match (*self, *other) {
            (PortalType::Inner, PortalType::Outer) | (PortalType::Outer, PortalType::Inner) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PortalData {
    pub id: String,
    pub portal_type: PortalType,
    pub position: Pos,
}

impl PortalData {
    pub fn new(
        ch1: char,
        ch2: char,
        position: Pos,
        is_outer: bool,
    ) -> Result<PortalData, MapError> {
        let mut id = ch1.to_string();
        id.push(ch2);

        match (ch1, ch2, is_outer) {
            ('A', 'A', true) => Ok(PortalData {
                id,
                portal_type: PortalType::Entrance,
                position,
            }),
            ('A', 'A', false) => Err(MapError::EntranceMustBeOuter),
            ('Z', 'Z', true) => Ok(PortalData {
                id,
                portal_type: PortalType::Exit,
                position,
            }),
            ('Z', 'Z', false) => Err(MapError::ExitMustBeOuter),
            (_, _, true) => Ok(PortalData {
                id,
                portal_type: PortalType::Outer,
                position,
            }),
            (_, _, false) => Ok(PortalData {
                id,
                portal_type: PortalType::Inner,
                position,
            }),
        }
    }

    pub fn is_outer(&self) -> bool {
        !matches!(self.portal_type, PortalType::Inner)
    }

    pub fn is_entrance(&self) -> bool {
        matches!(self.portal_type, PortalType::Entrance)
    }

    pub fn is_exit(&self) -> bool {
        matches!(self.portal_type, PortalType::Exit)
    }

    pub fn is_portal(&self) -> bool {
        matches!(self.portal_type, PortalType::Inner | PortalType::Outer)
    }
}

impl Display for PortalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.portal_type {
            PortalType::Inner => write!(f, "v{}", self.id),
            PortalType::Outer => write!(f, "^{}", self.id),
            PortalType::Entrance => write!(f, "vAA"),
            PortalType::Exit => write!(f, "^ZZ"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
    Portal,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Wall
    }
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<Pos, Tile>,
    portals: Vec<PortalData>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Map, MapError> {
        let (mut map, letters, outer, inner) = Map::parse_raw(input)?;

        let mut portals = Map::extract_portals(&mut map, &letters, &outer, true)?;
        portals.extend(Map::extract_portals(&mut map, &letters, &inner, false)?);

        if portals.len() * 2 != letters.len() {
            return Err(MapError::EqualAmountPortals);
        }
        let mut found_entrance = false;
        let mut found_exit = false;
        let mut inner = HashSet::new();
        let mut outer = HashSet::new();
        for p1 in &portals {
            match p1.portal_type {
                PortalType::Inner => {
                    inner.insert(p1.id.to_owned());
                }
                PortalType::Outer => {
                    outer.insert(p1.id.to_owned());
                }
                PortalType::Entrance => {
                    if found_entrance {
                        return Err(MapError::ExactlyOneEntranceExit);
                    } else {
                        found_entrance = true;
                    }
                }
                PortalType::Exit => {
                    if found_exit {
                        return Err(MapError::ExactlyOneEntranceExit);
                    } else {
                        found_exit = true;
                    }
                }
            }
        }
        if !found_entrance || !found_exit {
            return Err(MapError::ExactlyOneEntranceExit);
        }
        if inner != outer {
            return Err(MapError::PortalsNotEvenlyDistributed);
        }

        Ok(Map { map, portals })
    }

    fn parse_raw(
        input: &str,
    ) -> Result<(HashMap<Pos, Tile>, HashMap<Pos, char>, Area, Area), MapError> {
        let mut inner_col_start = None;
        let mut inner_row_start = None;
        let mut inner_col_end = None;
        let mut inner_row_end = None;
        let mut outer_col_end = None;
        let mut outer_row_end = None;

        let mut map = HashMap::new();
        let mut letters = HashMap::new();

        for (row, line) in (0..).zip(input.lines()) {
            for (col, ch) in (0..).zip(line.chars()) {
                match ch {
                    ' ' => {
                        if row >= 2 {
                            if col >= 2 {
                                if outer_col_end.is_none() {
                                    outer_col_end = Some(col - 1);
                                } else if inner_col_start.is_none()
                                    && outer_col_end.map_or(false, |c| col < c)
                                {
                                    inner_col_start = Some(col - 1);
                                    inner_row_start = Some(row - 1);
                                }
                            }

                            if outer_row_end.is_none()
                                && inner_row_end.is_some()
                                && col >= 2
                                && outer_col_end.map_or(false, |c| col <= c)
                            {
                                outer_row_end = Some(row - 1)
                            }
                        }
                    }

                    '#' => {
                        if inner_col_end.is_none() && inner_col_start.is_some() {
                            inner_col_end = Some(col)
                        }

                        if inner_row_end.is_none()
                            && inner_row_start.is_some()
                            && inner_col_start.map_or(false, |c| col > c)
                            && inner_col_end.map_or(false, |c| col < c)
                        {
                            inner_row_end = Some(row)
                        }
                    }
                    '.' => {
                        map.insert(Pos::new(col, row), Tile::Floor);
                    }
                    'A'..='Z' => {
                        letters.insert(Pos::new(col, row), ch);
                    }
                    _ => return Err(MapError::UnknownTile(ch)),
                }
            }
            if outer_col_end.is_none() && row >= 2 {
                outer_col_end = Some(line.len() as i32 - 1);
            }
        }

        zip6(
            inner_row_start,
            inner_row_end,
            outer_row_end,
            inner_col_start,
            inner_col_end,
            outer_col_end,
        )
        .map(
            |(
                inner_row_start,
                inner_row_end,
                outer_row_end,
                inner_col_start,
                inner_col_end,
                outer_col_end,
            )| {
                let outer = Area::new(Pos::new(2, 2), Pos::new(outer_col_end, outer_row_end));
                let inner = Area::new(
                    Pos::new(inner_col_start, inner_row_start),
                    Pos::new(inner_col_end, inner_row_end),
                );
                (map, letters, outer, inner)
            },
        )
        .ok_or(MapError::MapNotCorrectSquare)
    }

    fn extract_portals(
        map: &mut HashMap<Pos, Tile>,
        letters: &HashMap<Pos, char>,
        border: &Area,
        is_outer: bool,
    ) -> Result<Vec<PortalData>, MapError> {
        let extract_one_portal = |check_pos: &Pos, dir: &Pos| {
            if let Some(ch1) = letters.get(&(check_pos + dir)) {
                if let Some(ch2) = letters.get(&(check_pos + dir * 2)) {
                    return PortalData::new(*ch1, *ch2, *check_pos, is_outer);
                }
            } else if let Some(ch2) = letters.get(&(check_pos - dir)) {
                if let Some(ch1) = letters.get(&(check_pos - dir * 2)) {
                    return PortalData::new(*ch1, *ch2, *check_pos, is_outer);
                }
            }
            Err(MapError::PortalLettersNotAligned)
        };

        let mut result = vec![];
        let upper_row = border.get_upper_right().y();
        let lower_row = border.get_lower_left().y();
        let up_one = Pos::new(0, 1);
        for col in border.get_lower_left().x()..border.get_upper_right().x() {
            let check_pos1 = Pos::new(col, upper_row);
            if let Some(Tile::Floor) = map.get(&check_pos1) {
                result.push(extract_one_portal(&check_pos1, &up_one)?);
                map.insert(check_pos1, Tile::Portal);
            }
            let check_pos2 = Pos::new(col, lower_row);
            if let Some(Tile::Floor) = map.get(&check_pos2) {
                result.push(extract_one_portal(&check_pos2, &up_one)?);
                map.insert(check_pos2, Tile::Portal);
            }
        }

        let left_col = border.get_lower_left().x();
        let right_col = border.get_upper_right().x();
        let right_one = Pos::new(1, 0);
        for row in border.get_lower_left().y()..border.get_upper_right().y() {
            let check_pos = Pos::new(left_col, row);
            if let Some(Tile::Floor) = map.get(&check_pos) {
                result.push(extract_one_portal(&check_pos, &right_one)?);
                map.insert(check_pos, Tile::Portal);
            }
            let check_pos = Pos::new(right_col, row);
            if let Some(Tile::Floor) = map.get(&check_pos) {
                result.push(extract_one_portal(&check_pos, &right_one)?);
                map.insert(check_pos, Tile::Portal);
            }
        }

        Ok(result)
    }

    pub fn get_tile(&self, pos: &Pos) -> &Tile {
        self.map.get(pos).unwrap_or(&Tile::Wall)
    }

    pub fn get_portal(&self, pos: &Pos) -> Result<&PortalData, MapError> {
        self.portals
            .iter()
            .find(|pd| pd.position == *pos)
            .ok_or(MapError::UnknownPortal)
    }

    pub fn get_entrance(&self) -> &PortalData {
        self.portals
            .iter()
            .find(|portal_data| portal_data.is_entrance())
            .expect("We made sure to have exactly one portal previously")
    }

    pub fn get_exit(&self) -> &PortalData {
        self.portals
            .iter()
            .find(|portal_data| portal_data.is_exit())
            .expect("We made sure to have exactly one exit previously")
    }

    pub fn get_portal_complement(&self, portal_data: &PortalData) -> &PortalData {
        self.portals
            .iter()
            .find(|other_portal| {
                other_portal.id == *portal_data.id
                    && other_portal
                        .portal_type
                        .is_complement(&portal_data.portal_type)
            })
            .expect("We made sure to have a complement for each portal previously")
    }
}
