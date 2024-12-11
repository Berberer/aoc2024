use std::collections::HashSet;

fn parse_input(input_data: &str) -> Vec<Vec<u32>> {
    input_data
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_start_points(map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut start_points = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                start_points.push((x, y));
            }
        }
    }

    start_points
}

fn extend_paths(
    point: (usize, usize),
    paths: Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)>> {
    paths
        .iter()
        .map(|path| [point].iter().chain(path).cloned().collect())
        .collect()
}

fn find_paths((x, y): (usize, usize), topographic_map: &Vec<Vec<u32>>) -> Vec<Vec<(usize, usize)>> {
    let height = topographic_map[y][x];

    if height == 9 {
        vec![vec![(x, y)]]
    } else {
        let mut paths = Vec::new();

        if y > 0 && height + 1 == topographic_map[y - 1][x] {
            let paths_up = find_paths((x, y - 1), topographic_map);
            paths.extend(extend_paths((x, y), paths_up));
        }
        if y < topographic_map.len() - 1 && height + 1 == topographic_map[y + 1][x] {
            let paths_down = find_paths((x, y + 1), topographic_map);
            paths.extend(extend_paths((x, y), paths_down));
        }
        if x > 0 && height + 1 == topographic_map[y][x - 1] {
            let paths_left = find_paths((x - 1, y), topographic_map);
            paths.extend(extend_paths((x, y), paths_left));
        }
        if x < topographic_map[y].len() - 1 && height + 1 == topographic_map[y][x + 1] {
            let paths_right = find_paths((x + 1, y), topographic_map);
            paths.extend(extend_paths((x, y), paths_right));
        }

        paths
    }
}

fn get_trailhead_score(paths: &[Vec<(usize, usize)>]) -> usize {
    HashSet::<(usize, usize)>::from_iter(paths.iter().map(|path| path[path.len() - 1])).len()
}

fn main() {
    let input = include_str!("../inputs/data_day_10.txt");
    let topographic_map = parse_input(input);
    let start_points = find_start_points(&topographic_map);
    let (trailhead_score_sum, trailhead_rating_sum) = start_points.iter().fold(
        (0, 0),
        |(trailhead_score_acc, trailhead_rating_acc), point| {
            let paths_from_point = find_paths(*point, &topographic_map);
            (
                trailhead_score_acc + get_trailhead_score(&paths_from_point),
                trailhead_rating_acc + paths_from_point.len(),
            )
        },
    );

    // Solution for puzzle 1
    println!("The sum of trailhead scores is {trailhead_score_sum}");

    // Solution for puzzle 2
    println!("The sum of trailhead ratings is {trailhead_rating_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_points() {
        let topographic_map = vec![
            vec![0, 1, 2, 3],
            vec![1, 3, 5, 4],
            vec![9, 0, 6, 9],
            vec![9, 8, 7, 8],
        ];
        assert_eq!(find_start_points(&topographic_map), vec![(0, 0), (1, 2),]);
    }

    #[test]
    fn test_find_paths() {
        let topographic_map = vec![
            vec![0, 1, 2, 3],
            vec![1, 3, 5, 4],
            vec![9, 0, 6, 9],
            vec![9, 8, 7, 8],
        ];
        assert_eq!(
            HashSet::from_iter(find_paths((0, 0), &topographic_map)),
            HashSet::from([
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (3, 1),
                    (2, 1),
                    (2, 2),
                    (2, 3),
                    (3, 3),
                    (3, 2)
                ],
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (3, 1),
                    (2, 1),
                    (2, 2),
                    (2, 3),
                    (1, 3),
                    (0, 3)
                ],
            ])
        );
    }

    #[test]
    fn test_get_trailhead_score() {
        let paths = vec![
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 5),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (5, 4),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
                (1, 1),
                (1, 0),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
                (0, 2),
                (0, 3),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 5),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (5, 4),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (1, 3),
                (1, 2),
                (1, 1),
                (1, 0),
            ],
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (1, 3),
                (1, 2),
                (0, 2),
                (0, 3),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 5),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (5, 4),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
                (1, 1),
                (1, 0),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (1, 2),
                (0, 2),
                (0, 3),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 5),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4),
                (5, 4),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (1, 3),
                (1, 2),
                (1, 1),
                (1, 0),
            ],
            vec![
                (2, 0),
                (3, 0),
                (3, 1),
                (3, 2),
                (2, 2),
                (2, 3),
                (1, 3),
                (1, 2),
                (0, 2),
                (0, 3),
            ],
        ];
        assert_eq!(get_trailhead_score(&paths), 5);
    }
}
