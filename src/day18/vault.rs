use super::{content::Content, error::VaultError, explorer::Explorer, map::Map, path::Path};

fn pop_minimum(list: &mut Vec<Content>) -> Option<Content> {
    let min = list.iter().min()?;
    let index = list
        .iter()
        .position(|content| content == min)
        .unwrap_or_default();
    Some(list.swap_remove(index))
}

fn do_search(paths: Vec<Path>) -> Result<usize, VaultError> {
    let key_count = paths.iter().map(|path| path.key_count()).sum();
    let mut check_list = vec![Default::default()];

    while let Some(item) = pop_minimum(&mut check_list) {
        if item.count_keys() == key_count {
            return Ok(item.get_steps());
        }
        Content::merge_clean(&mut check_list, item.get_neighbors(&paths));
    }

    Err(VaultError::NoPath)
}

pub fn find_all_keys_part1(map: &Map) -> Result<usize, VaultError> {
    let path = Explorer::new(map).explore_part1()?;
    do_search(vec![path])
}

pub fn find_all_keys_part2(map: &Map) -> Result<usize, VaultError> {
    let paths = Explorer::new(map).explore_part2()?;
    do_search(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file::read_data;

    #[test]
    fn test_find_all_keys() -> Result<(), VaultError> {
        let input = read_data("day18", "example1.txt")?;
        let map = Map::new(&input)?;
        let expected = 8;
        let result = find_all_keys_part1(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_2() -> Result<(), VaultError> {
        let input = read_data("day18", "example2.txt")?;
        let map = Map::new(&input)?;
        let expected = 86;
        let result = find_all_keys_part1(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_3() -> Result<(), VaultError> {
        let input = read_data("day18", "example3.txt")?;
        let map = Map::new(&input)?;
        let expected = 132;
        let result = find_all_keys_part1(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_find_all_keys_4() -> Result<(), VaultError> {
        let input = read_data("day18", "example4.txt")?;
        let map = Map::new(&input)?;
        let expected = 136;
        let result = find_all_keys_part1(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_5() -> Result<(), VaultError> {
        let input = read_data("day18", "example5.txt")?;
        let map = Map::new(&input)?;
        let expected = 81;
        let result = find_all_keys_part1(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_7() -> Result<(), VaultError> {
        let input = read_data("day18", "example7.txt")?;
        let map = Map::new(&input)?;
        let expected = 8;
        let result = find_all_keys_part2(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_8() -> Result<(), VaultError> {
        let input = read_data("day18", "example8.txt")?;
        let map = Map::new(&input)?;
        let expected = 24;
        let result = find_all_keys_part2(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_find_all_keys_9() -> Result<(), VaultError> {
        let input = read_data("day18", "example9.txt")?;
        let map = Map::new(&input)?;
        let expected = 72;
        let result = find_all_keys_part2(&map)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
