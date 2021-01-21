use crate::{
    explorer::Explorer, map::Map, multi_maze::MultiContent, single_maze::Content,
    vault_error::VaultError,
};

fn extract_minimum(list: &mut Vec<Content>) -> Option<Content> {
    if let Some(min) = list.iter().min() {
        let index = list
            .iter()
            .position(|content| content == min)
            .unwrap_or_default();
        Some(list.swap_remove(index))
    } else {
        None
    }
}

pub fn find_all_keys(map: &Map) -> Result<usize, VaultError> {
    let path = Explorer::new(map).explore()?;
    let key_count = path.key_count();
    let mut check_list = Vec::new();
    let mut next_item = Some(Content::new());
    while let Some(item) = next_item.take() {
        if item.count_keys() == key_count {
            return Ok(item.steps);
        }

        let neighbors = item.get_neighbors(&path);
        Content::clean_up(&mut check_list, neighbors);
        next_item = extract_minimum(&mut check_list);
    }

    Err(VaultError::NoPath)
}

fn extract_minimum2(list: &mut Vec<MultiContent>) -> Option<MultiContent> {
    if let Some(min) = list.iter().min() {
        let index = list
            .iter()
            .position(|content| content == min)
            .unwrap_or_default();
        Some(list.swap_remove(index))
    } else {
        None
    }
}

pub fn find_all_keys2(map: &Map) -> Result<usize, VaultError> {
    let paths = Explorer::new(map).explore2()?;
    let key_count = paths.iter().map(|path| path.key_count()).sum();
    let mut check_list = Vec::new();
    let mut next_item = Some(MultiContent::new(4));
    while let Some(item) = next_item.take() {
        if item.count_keys() == key_count {
            return Ok(item.steps);
        }

        let neighbors = item.get_neighbors(&paths);
        MultiContent::clean_up(&mut check_list, neighbors);
        next_item = extract_minimum2(&mut check_list);
    }

    Err(VaultError::NoPath)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_all_lines;
    use std::error::Error;

    #[test]
    fn test_find_all_keys() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example1.txt")?;
        let map = Map::new(&input)?;
        let expected = 8;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_2() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example2.txt")?;
        let map = Map::new(&input)?;
        let expected = 86;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_3() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example3.txt")?;
        let map = Map::new(&input)?;
        let expected = 132;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_find_all_keys_4() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example4.txt")?;
        let map = Map::new(&input)?;
        let expected = 136;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_5() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example5.txt")?;
        let map = Map::new(&input)?;
        let expected = 81;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_7() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example7.txt")?;
        let map = Map::new(&input)?;
        let expected = 8;
        let result = find_all_keys2(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_8() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example8.txt")?;
        let map = Map::new(&input)?;
        let expected = 24;
        let result = find_all_keys2(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_9() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "input.txt")?;
        let map = Map::new(&input)?;
        let expected = 72;
        let result = find_all_keys(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
