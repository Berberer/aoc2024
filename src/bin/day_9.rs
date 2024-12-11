#[derive(Clone, Copy, Debug, PartialEq)]
struct FileBlock {
    id: u64,
    original_size: u32,
    can_be_moved: bool,
}

impl FileBlock {
    fn new(id: u64, original_size: u32) -> Self {
        FileBlock {
            id,
            original_size,
            can_be_moved: id != 0,
        }
    }

    fn set_as_moved(&mut self) {
        self.can_be_moved = false;
    }
}

fn parse_input(input: &str) -> Vec<Option<FileBlock>> {
    let mut disk_map = Vec::new();

    let mut id = 0;
    let mut is_file = true;
    for c in input.trim().chars() {
        let size = c.to_digit(10).unwrap();
        for _ in 0..size {
            if is_file {
                disk_map.push(Some(FileBlock::new(id, size)));
            } else {
                disk_map.push(None);
            }
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }

    disk_map
}

fn find_free_blocks(disk_map: &[Option<FileBlock>]) -> Vec<(usize, usize)> {
    let mut free_blocks = Vec::new();
    let mut last_free_block_index = None;
    for (index, block) in disk_map.iter().enumerate() {
        if block.is_some() {
            if let Some(free_block_index) = last_free_block_index {
                free_blocks.push((free_block_index, index - 1));
                last_free_block_index = None;
            }
        } else if last_free_block_index.is_none() {
            last_free_block_index = Some(index);
        }
    }

    if let Some(free_block_index) = last_free_block_index {
        free_blocks.push((free_block_index, disk_map.len() - 1));
    }

    free_blocks
}

fn find_last_file_block(disk_map: &[Option<FileBlock>]) -> Option<(usize, usize)> {
    disk_map
        .iter()
        .enumerate()
        .rfind(|(_, block)| block.map(|b| b.can_be_moved).unwrap_or(false))
        .map(|(end_index, block)| {
            disk_map[..=end_index]
                .iter()
                .enumerate()
                .rfind(|(_, b)| b.map(|b| b.id != block.unwrap().id).unwrap_or(true))
                .map(|(start_index, _)| (start_index + 1, end_index))
                .unwrap_or((end_index, end_index))
        })
}

fn find_file_move(
    disk_map: &[Option<FileBlock>],
    can_split_files: bool,
) -> Option<(usize, usize, usize)> {
    let free_blocks = find_free_blocks(disk_map);
    let last_file_block = find_last_file_block(disk_map);
    if let Some((last_file_block_start, last_file_block_end)) = last_file_block {
        for (free_block_start, free_block_end) in free_blocks {
            if free_block_start < last_file_block_start {
                if can_split_files {
                    return Some((free_block_start, last_file_block_end, 1));
                } else if (free_block_end - free_block_start)
                    >= (last_file_block_end - last_file_block_start)
                {
                    return Some((
                        free_block_start,
                        last_file_block_start,
                        last_file_block_end - last_file_block_start + 1,
                    ));
                }
            }
        }
    }

    None
}

fn set_block_as_moved(id: u64, disk_map: &mut [Option<FileBlock>]) {
    disk_map
        .iter_mut()
        .filter(|block| block.map(|b| b.id == id).unwrap_or(false))
        .for_each(|block| block.as_mut().unwrap().set_as_moved())
}

fn rearrange_files(
    disk_map: &[Option<FileBlock>],
    can_split_files: bool,
) -> Vec<Option<FileBlock>> {
    let mut rearranged_disk_map = disk_map.to_owned();
    let mut has_movable_file = true;

    while has_movable_file {
        if let Some((free_block_index, file_block_index, file_block_size)) =
            find_file_move(&rearranged_disk_map, can_split_files)
        {
            for i in 0..file_block_size {
                rearranged_disk_map[free_block_index + i] =
                    rearranged_disk_map[file_block_index + i];
                rearranged_disk_map[free_block_index + i]
                    .as_mut()
                    .unwrap()
                    .set_as_moved();
                rearranged_disk_map[file_block_index + i] = None;
            }
        } else {
            let file_block = rearranged_disk_map
                .iter()
                .rfind(|block| block.map(|b| b.can_be_moved).unwrap_or(false));
            if let Some(Some(FileBlock { id, .. })) = file_block {
                set_block_as_moved(*id, &mut rearranged_disk_map);
            } else {
                has_movable_file = false;
            }
        }
    }

    rearranged_disk_map
}

fn calculate_checksum(disk_map: &[Option<FileBlock>]) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .map(|(index, file_block)| match file_block {
            Some(FileBlock { id, .. }) => index as u64 * id,
            None => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("../inputs/data_day_9.txt");
    let disk_map = parse_input(input);

    // Solution for puzzle 1
    let rearranged_disk_map = rearrange_files(&disk_map, true);
    let checksum = calculate_checksum(&rearranged_disk_map);
    println!(
        "Rearranging with splitting files ends with a filesystem that has the checksum {checksum}"
    );

    // Solution for puzzle 1
    let rearranged_disk_map = rearrange_files(&disk_map, false);
    let checksum = calculate_checksum(&rearranged_disk_map);
    println!("Rearranging without splitting files ends with a filesystem that has the checksum {checksum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_free_blocks() {
        let disk_map = vec![
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            None,
            None,
            Some(FileBlock::new(1, 2)),
            Some(FileBlock::new(1, 2)),
            None,
        ];
        assert_eq!(find_free_blocks(&disk_map), vec![(3, 4), (7, 7)]);
    }

    #[test]
    fn test_find_last_file_block() {
        let mut disk_map = vec![
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(1, 1)),
            None,
            Some(FileBlock::new(2, 2)),
            Some(FileBlock::new(2, 2)),
        ];
        assert_eq!(find_last_file_block(&disk_map), Some((5, 6)));

        set_block_as_moved(2, &mut disk_map);
        assert_eq!(find_last_file_block(&disk_map), Some((3, 3)));
    }

    #[test]
    fn test_find_file_move_with_split() {
        let disk_map = vec![
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            None,
            None,
            Some(FileBlock::new(1, 2)),
            Some(FileBlock::new(1, 2)),
            None,
        ];
        assert_eq!(find_file_move(&disk_map, true), Some((3, 6, 1)));
    }

    #[test]
    fn test_find_file_move_without_split() {
        let disk_map = vec![
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            Some(FileBlock::new(0, 3)),
            None,
            None,
            Some(FileBlock::new(1, 2)),
            Some(FileBlock::new(1, 2)),
            None,
        ];
        assert_eq!(find_file_move(&disk_map, false), Some((3, 5, 2)));
    }

    #[test]
    fn test_rearrange_file_blocks_with_split() {
        let disk_map = vec![
            Some(FileBlock::new(0, 2)),
            Some(FileBlock::new(0, 2)),
            None,
            Some(FileBlock::new(1, 1)),
            None,
            Some(FileBlock::new(2, 2)),
            Some(FileBlock::new(2, 2)),
        ];
        assert_eq!(
            rearrange_files(&disk_map, true),
            vec![
                Some(FileBlock::new(0, 2)),
                Some(FileBlock::new(0, 2)),
                Some(FileBlock {
                    id: 2,
                    original_size: 2,
                    can_be_moved: false
                }),
                Some(FileBlock {
                    id: 1,
                    original_size: 1,
                    can_be_moved: false
                }),
                Some(FileBlock {
                    id: 2,
                    original_size: 2,
                    can_be_moved: false
                }),
                None,
                None
            ]
        );
    }

    #[test]
    fn test_rearrange_file_blocks_without_split() {
        let disk_map = vec![
            Some(FileBlock::new(0, 2)),
            Some(FileBlock::new(0, 2)),
            None,
            None,
            None,
            Some(FileBlock::new(1, 1)),
            Some(FileBlock::new(2, 2)),
            Some(FileBlock::new(2, 2)),
        ];
        assert_eq!(
            rearrange_files(&disk_map, true),
            vec![
                Some(FileBlock::new(0, 2)),
                Some(FileBlock::new(0, 2)),
                Some(FileBlock {
                    id: 2,
                    original_size: 2,
                    can_be_moved: false
                }),
                Some(FileBlock {
                    id: 2,
                    original_size: 2,
                    can_be_moved: false
                }),
                Some(FileBlock {
                    id: 1,
                    original_size: 1,
                    can_be_moved: false
                }),
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn test_calculate_checksum() {
        let disk_map = vec![
            Some(FileBlock::new(0, 2)),
            Some(FileBlock::new(0, 2)),
            Some(FileBlock::new(1, 1)),
            Some(FileBlock::new(3, 1)),
            None,
        ];
        assert_eq!(calculate_checksum(&disk_map), 11);
    }
}
