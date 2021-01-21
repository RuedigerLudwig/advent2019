use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::map::ENTRANCE;

#[derive(Debug, Clone)]
struct Content {
    steps: usize,
    required: HashSet<char>,
    blocking: HashSet<char>,
}
type RefKeyContent<'a> = (&'a char, &'a Content);

impl Content {
    fn new(steps: usize, required: HashSet<char>) -> Content {
        Content {
            steps,
            required,
            blocking: HashSet::new(),
        }
    }

    fn copy_inc_steps(&self, steps: usize) -> Content {
        let mut clone = self.clone();
        clone.steps += steps;
        clone
    }

    fn copy_add_blocking(&self, blocking: char) -> Content {
        let mut clone = self.clone();
        clone.blocking.insert(blocking);
        clone
    }

    fn inc_steps(&mut self, steps: usize) {
        self.steps += steps;
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; {:?}; -{:?}",
            self.steps, self.required, self.blocking
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
                            if let Some(_) = next_map.insert(
                                *snd_char,
                                snd_content.copy_inc_steps(fst_content.steps - correct),
                            ) {
                                panic!();
                            }
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
        entry.insert(*key, Content::new(steps, required.clone()));
    }

    pub fn merge_on_key(&mut self, from: &char, other: Path) {
        for (other_from, other_map) in &other.content {
            if *other_from == ENTRANCE {
                self.content.entry(ENTRANCE).and_modify(|tm| {
                    for (to, content) in other_map {
                        if let Some(_) = tm.insert(*to, content.copy_add_blocking(*from)) {
                            panic!();
                        }
                    }
                });

                let entry = self.content.entry(*from).or_insert(HashMap::new());
                for (other_to, other_content) in other_map {
                    if let Some(_) = entry.insert(*other_to, other_content.clone()) {
                        panic!();
                    }
                }
            } else {
                if let Some(_) = self.content.insert(*other_from, other_map.clone()) {
                    panic!();
                }
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

pub struct Connection<'a> {
    pub from: char,
    pub to: char,
    pub steps: usize,
    pub required: &'a HashSet<char>,
}

pub struct ConnectionIterator<'a> {
    to_map: Option<Box<dyn Iterator<Item = RefKeyContent<'a>> + 'a>>,
    from: char,
}

impl<'a> ConnectionIterator<'a> {
    fn new(path: &'a Path, from: char, keyring: &'a HashSet<char>) -> ConnectionIterator<'a> {
        let iter: Option<Box<dyn Iterator<Item = RefKeyContent>>> =
            path.content.get(&from).map(move |map| {
                Box::new(
                    map.iter()
                        .filter(move |(key, _)| !keyring.contains(*key))
                        .filter(move |(_, content)| {
                            keyring.is_superset(&content.required)
                                && keyring.is_superset(&content.blocking)
                        }),
                ) as Box<dyn Iterator<Item = RefKeyContent>>
            });

        ConnectionIterator { from, to_map: iter }
    }
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = Connection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut to_map) = self.to_map {
            if let Some((to, content)) = to_map.next() {
                Some(Connection {
                    to: *to,
                    steps: content.steps,
                    required: &content.required,
                    from: self.from,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
