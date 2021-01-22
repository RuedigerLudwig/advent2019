use std::collections::{HashMap, HashSet};

use crate::{map::ENTRANCE, path::Path};

#[derive(Debug, Eq, Default)]
pub struct Content {
    _steps: usize,
    _nodes: HashMap<usize, char>,
    _keyring: HashSet<char>,
}

impl Content {
    pub fn get_steps(&self) -> usize {
        return self._steps;
    }

    fn replace_node(&self, pos: usize, other: char) -> HashMap<usize, char> {
        let mut new_nodes = self._nodes.clone();
        new_nodes.insert(pos, other);
        new_nodes
    }

    pub fn get_neighbors(&self, paths: &Vec<Path>) -> Vec<Content> {
        (0..paths.len())
            .map(|index| {
                let path = &paths[index];
                let node = self._nodes.get(&index).unwrap_or(&ENTRANCE);

                path.get_possible_neighbors(*node, &self._keyring)
                    .filter_map(move |other| {
                        let mut next_keyring = self._keyring.clone();
                        next_keyring.insert(other.to);
                        let next_steps = self._steps + other.steps;
                        Some(Content {
                            _steps: next_steps,
                            _nodes: self.replace_node(index, other.to),
                            _keyring: next_keyring,
                        })
                    })
            })
            .flatten()
            .collect()
    }

    pub fn merge_clean(old_list: &mut Vec<Content>, mut new_list: Vec<Content>) {
        let mut keep_new = vec![true; new_list.len()];

        old_list.retain(|content| {
            let mut keep_old = true;
            for (keep_new, new_item) in keep_new.iter_mut().zip(&new_list) {
                if content._nodes == new_item._nodes {
                    if content._steps >= new_item._steps
                        && content._keyring.is_subset(&new_item._keyring)
                    {
                        keep_old = false;
                        break;
                    } else if content._steps <= new_item._steps
                        && content._keyring.is_superset(&new_item._keyring)
                    {
                        *keep_new = false;
                    }
                }
            }
            keep_old
        });

        old_list.extend(
            new_list
                .drain(..)
                .zip(keep_new)
                .filter_map(|(content, use_it)| if use_it { Some(content) } else { None }),
        );
    }

    pub fn count_keys(&self) -> usize {
        self._keyring.len()
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self._steps == other._steps && self._keyring == other._keyring
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Content {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self._steps
            .cmp(&other._steps)
            .then(self._keyring.len().cmp(&other._keyring.len()).reverse())
    }
}
