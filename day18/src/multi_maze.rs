use std::collections::HashSet;

use crate::{map::ENTRANCE, path::Path};

#[derive(Debug)]
pub struct MultiContent {
    pub steps: usize,
    nodes: Vec<char>,
    keyring: HashSet<char>,
}

impl MultiContent {
    pub fn new(num_robots: usize) -> MultiContent {
        MultiContent {
            steps: 0,
            nodes: vec![ENTRANCE].repeat(num_robots),
            keyring: HashSet::new(),
        }
    }

    fn replace_node(&self, pos: usize, other: char) -> Vec<char> {
        let mut new_nodes = self.nodes.clone();
        new_nodes[pos] = other;
        new_nodes
    }

    pub fn get_neighbors(&self, paths: &Vec<Path>) -> Vec<MultiContent> {
        let mut result = Vec::new();
        for num in 0..self.nodes.len() {
            let path = &paths[num];
            let node = self.nodes[num];

            result.extend(
                path.get_possible_neighbors(node, &self.keyring)
                    .filter_map(|other| {
                        let next_steps = self.steps + other.steps;
                        let mut next_keyring = self.keyring.clone();
                        next_keyring.insert(other.to);
                        Some(MultiContent {
                            steps: next_steps,
                            nodes: self.replace_node(num, other.to),
                            keyring: next_keyring,
                        })
                    }),
            );
        }
        result
    }

    pub fn clean_up(old_list: &mut Vec<MultiContent>, mut new_list: Vec<MultiContent>) {
        let mut keep_new = vec![true].repeat(new_list.len());

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

impl PartialEq for MultiContent {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.keyring == other.keyring
    }
}

impl Eq for MultiContent {}

impl PartialOrd for MultiContent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MultiContent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .then(self.keyring.len().cmp(&other.keyring.len()).reverse())
    }
}
