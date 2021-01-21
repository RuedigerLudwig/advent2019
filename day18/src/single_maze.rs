use std::collections::HashSet;

use crate::{map::ENTRANCE, path::Path};

#[derive(Debug)]
pub struct Content {
    pub steps: usize,
    node: char,
    keyring: HashSet<char>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            steps: 0,
            node: ENTRANCE,
            keyring: HashSet::new(),
        }
    }

    pub fn get_neighbors(&self, path: &Path) -> Vec<Content> {
        path.get_possible_neighbors(self.node, &self.keyring)
            .filter_map(|other| {
                let next_steps = self.steps + other.steps;
                let mut next_keyring = self.keyring.clone();
                next_keyring.insert(other.to);
                Some(Content {
                    steps: next_steps,
                    node: other.to,
                    keyring: next_keyring,
                })
            })
            .collect()
    }

    pub fn clean_up(old_list: &mut Vec<Content>, mut new_list: Vec<Content>) {
        let mut keep_new = vec![true].repeat(new_list.len());

        old_list.retain(|content| {
            let mut keep_old = true;
            for (keep_new, new_item) in keep_new.iter_mut().zip(&new_list) {
                if content.node == new_item.node {
                    if content.steps >= new_item.steps
                        && content.keyring.is_subset(&new_item.keyring)
                    {
                        keep_old = false;
                        break;
                    } else if content.steps <= new_item.steps
                        && content.keyring.is_superset(&new_item.keyring)
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
        self.keyring.len()
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.node == other.node && self.keyring == other.keyring
    }
}

impl Eq for Content {}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Content {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .then(self.keyring.len().cmp(&other.keyring.len()).reverse())
    }
}
