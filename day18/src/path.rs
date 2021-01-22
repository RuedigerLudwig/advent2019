use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::map::ENTRANCE;

#[derive(Debug, Clone)]
struct Content {
    _steps: usize,
    _required: HashSet<char>,
}
type RefKeyContent<'a> = (&'a char, &'a Content);

impl Content {
    fn clone_with_steps(&self, steps: usize) -> Content {
        let mut clone = self.clone();
        clone._steps += steps;
        clone
    }

    fn clone_with_required(&self, blocking: char) -> Content {
        let mut clone = self.clone();
        clone._required.insert(blocking);
        clone
    }

    fn inc_steps(&mut self, steps: usize) {
        self._steps += steps;
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; {:?}",
            self._steps,
            self._required //, self.blocking
        )
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    content: HashMap<char, HashMap<char, Content>>,
    needed: HashMap<char, usize>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            content: HashMap::new(),
            needed: HashMap::new(),
        }
    }

    pub fn merge(all_paths: Vec<Path>, special_merge: bool) -> Path {
        let mut merged = Path::new();

        for path in &all_paths {
            for (from, to_map) in &path.content {
                merged
                    .content
                    .entry(*from)
                    .or_insert(HashMap::new())
                    .extend(to_map.clone())
            }
        }

        for one in 0..all_paths.len() {
            let first = all_paths[one].content.get(&ENTRANCE).unwrap();
            for two in 0..all_paths.len() {
                if one != two {
                    let correct = if special_merge && (one % 2) != (two % 2) {
                        2
                    } else {
                        0
                    };
                    let second = all_paths[two].content.get(&ENTRANCE).unwrap();
                    for (fst_char, fst_content) in first {
                        let next_map = merged.content.entry(*fst_char).or_insert(HashMap::new());
                        for (snd_char, snd_content) in second {
                            next_map.insert(
                                *snd_char,
                                snd_content.clone_with_steps(fst_content._steps - correct),
                            );
                        }
                    }
                }
            }
        }

        merged
    }

    pub fn key_count(&self) -> usize {
        self.content[&ENTRANCE].len()
    }

    pub fn add_from_entrance(&mut self, key: &char, steps: usize, required: &HashSet<char>) {
        let entry = self.content.entry(ENTRANCE).or_insert(HashMap::new());
        entry.insert(
            *key,
            Content {
                _steps: steps,
                _required: required.clone(),
            },
        );
    }

    pub fn merge_on_key(&mut self, from: &char, other: Path) {
        for (other_from, other_map) in &other.content {
            if *other_from == ENTRANCE {
                self.content.entry(ENTRANCE).and_modify(|tm| {
                    for (to, content) in other_map {
                        tm.insert(*to, content.clone_with_required(*from));
                    }
                });

                let entry = self.content.entry(*from).or_insert(HashMap::new());
                for (other_to, other_content) in other_map {
                    entry.insert(*other_to, other_content.clone());
                }
            } else {
                self.content.insert(*other_from, other_map.clone());
            }
        }
    }

    pub fn inc_steps_from_entrance(&mut self, steps: usize) {
        self.content.entry(ENTRANCE).and_modify(|path| {
            for content in path.values_mut() {
                content.inc_steps(steps)
            }
        });
    }

    pub fn get_possible_neighbors<'a>(
        &'a self,
        from: char,
        keyring: &'a HashSet<char>,
    ) -> ConnectionIterator<'a> {
        ConnectionIterator::new(self, from, keyring)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (fst, from_map) in &self.content {
            write!(f, "'{}' =>\n", fst)?;
            for (snd, content) in from_map {
                write!(f, "       '{}' => ({})\n", snd, content)?;
            }
        }
        write!(f, "\n")
    }
}

pub struct Connection {
    pub to: char,
    pub steps: usize,
}

pub struct ConnectionIterator<'a> {
    _iter: Option<Box<dyn Iterator<Item = RefKeyContent<'a>> + 'a>>,
}

impl<'a> ConnectionIterator<'a> {
    fn new(path: &'a Path, from: char, keyring: &'a HashSet<char>) -> ConnectionIterator<'a> {
        let iter: Option<Box<dyn Iterator<Item = RefKeyContent>>> =
            path.content.get(&from).map(move |map| {
                Box::new(
                    map.iter()
                        .filter(move |(key, _)| !keyring.contains(*key))
                        .filter(move |(_, content)| keyring.is_superset(&content._required)),
                ) as Box<dyn Iterator<Item = RefKeyContent>>
            });

        ConnectionIterator { _iter: iter }
    }
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = Connection;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut to_map) = self._iter {
            if let Some((to, content)) = to_map.next() {
                Some(Connection {
                    to: *to,
                    steps: content._steps,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
