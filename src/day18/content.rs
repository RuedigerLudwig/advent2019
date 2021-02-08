use super::{map::ENTRANCE, path::Path};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Default)]
pub struct Content {
    steps: usize,
    nodes: HashMap<usize, char>,
    keyring: HashSet<char>,
}

impl Content {
    pub fn get_steps(&self) -> usize {
        return self.steps;
    }

    fn replace_node(&self, pos: usize, other: char) -> HashMap<usize, char> {
        let mut new_nodes = self.nodes.clone();
        new_nodes.insert(pos, other);
        new_nodes
    }

    pub fn get_neighbors(&self, paths: &Vec<Path>) -> Vec<Content> {
        (0..paths.len())
            .map(|index| {
                let path = &paths[index];
                let node = self.nodes.get(&index).unwrap_or(&ENTRANCE);

                path.get_possible_neighbors(*node, &self.keyring)
                    .filter_map(move |other| {
                        let mut next_keyring = self.keyring.clone();
                        next_keyring.insert(other.to);
                        let next_steps = self.steps + other.steps;
                        Some(Content {
                            steps: next_steps,
                            nodes: self.replace_node(index, other.to),
                            keyring: next_keyring,
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
                if content.nodes == new_item.nodes {
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
        self.steps == other.steps && self.keyring == other.keyring
    }
}

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
