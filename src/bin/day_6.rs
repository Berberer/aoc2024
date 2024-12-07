use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_next_coordinates(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Map {
    blocked_fields: Vec<Vec<bool>>,
}

impl Map {
    fn is_field_blocked(&self, (x, y): (i32, i32)) -> Option<bool> {
        if y >= 0
            && y < self.blocked_fields.len() as i32
            && x >= 0
            && x < self.blocked_fields[y as usize].len() as i32
        {
            Some(self.blocked_fields[y as usize][x as usize])
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Map, (i32, i32)) {
    let mut start_coordinates = None;
    let mut blocked_fields = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut line_blocked_fields = Vec::new();
        for (x, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    start_coordinates = Some((x as i32, y as i32));
                    line_blocked_fields.push(false);
                }
                '#' => line_blocked_fields.push(true),
                _ => line_blocked_fields.push(false),
            };
        }
        blocked_fields.push(line_blocked_fields);
    }

    (Map { blocked_fields }, start_coordinates.unwrap())
}

fn find_path(start_coordinates: &(i32, i32), map: &Map) -> Option<Vec<(i32, i32)>> {
    let mut path = vec![*start_coordinates];
    let mut path_directions = HashSet::from([(*start_coordinates, Direction::Up)]);
    let mut current_direction = Direction::Up;
    let mut current_coordinates = *start_coordinates;
    let mut reached_map_border = false;

    while !reached_map_border {
        let next_coordinates = current_direction.get_next_coordinates(current_coordinates);
        if path_directions.contains(&(next_coordinates, current_direction)) {
            return None;
        } else {
            path_directions.insert((next_coordinates, current_direction));
        }

        if let Some(is_blocked) = map.is_field_blocked(next_coordinates) {
            if is_blocked {
                current_direction = current_direction.rotate();
            } else {
                current_coordinates = next_coordinates;
                path.push(current_coordinates);
            }
        } else {
            reached_map_border = true;
        }
    }

    Some(path)
}

fn main() {
    let input = include_str!("../inputs/data_day_6.txt");
    let (map, guard_start_coordinates) = parse_input(input);
    let path = find_path(&guard_start_coordinates, &map).unwrap();
    let unique_coordinates: HashSet<(i32, i32)> = HashSet::from_iter(path.iter().cloned());

    // Solution for puzzle 1
    println!(
        "The guard will leave the map after visiting {} unique positions",
        unique_coordinates.len()
    );

    // Solution for puzzle 2
    let mut possible_obstacles_for_loops = HashSet::new();
    for (x, y) in path[1..].iter() {
        let mut additional_obstacle_map = map.clone();
        additional_obstacle_map.blocked_fields[*y as usize][*x as usize] = true;
        if find_path(&guard_start_coordinates, &additional_obstacle_map).is_none() {
            possible_obstacles_for_loops.insert((x, y));
        }
    }
    println!(
        "By placing new obstacles, the guard can be trapped in {} different loops",
        possible_obstacles_for_loops.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_coordinates() {
        assert_eq!(Direction::Up.get_next_coordinates((1, 1)), (1, 0));
        assert_eq!(Direction::Right.get_next_coordinates((1, 1)), (2, 1));
        assert_eq!(Direction::Down.get_next_coordinates((1, 1)), (1, 2));
        assert_eq!(Direction::Left.get_next_coordinates((1, 1)), (0, 1));
    }

    #[test]
    fn test_is_field_blocked() {
        let map = Map {
            blocked_fields: vec![vec![true, false], vec![false, false]],
        };
        assert_eq!(map.is_field_blocked((0, 0)), Some(true));
        assert_eq!(map.is_field_blocked((1, 1)), Some(false));
        assert_eq!(map.is_field_blocked((-1, 0)), None);
        assert_eq!(map.is_field_blocked((0, 2)), None);
    }

    #[test]
    fn test_find_path_without_loop() {
        let map = Map {
            blocked_fields: vec![
                vec![true, false, false],
                vec![false, false, true],
                vec![false, false, false],
            ],
        };
        let path = find_path(&(0, 2), &map);
        assert_eq!(path, Some(vec![(0, 2), (0, 1), (1, 1), (1, 2)]));
    }

    #[test]
    fn test_find_path_with_loop() {
        let map = Map {
            blocked_fields: vec![
                vec![false, true, false, false],
                vec![false, false, false, true],
                vec![true, false, false, false],
                vec![false, false, true, false],
            ],
        };
        let path = find_path(&(1, 3), &map);
        assert_eq!(path, None);
    }
}
