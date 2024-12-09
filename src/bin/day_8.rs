use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

type FrequencyMapping = HashMap<char, Vec<(i32, i32)>>;

fn parse_input(input: &str) -> (FrequencyMapping, i32, i32) {
    let mut antenna_locations = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y);

        for (x, field_type) in line.chars().enumerate() {
            max_x = max_x.max(x);

            if field_type != '.' {
                if let Entry::Vacant(e) = antenna_locations.entry(field_type) {
                    e.insert(vec![(x as i32, y as i32)]);
                } else {
                    antenna_locations
                        .get_mut(&field_type)
                        .unwrap()
                        .push((x as i32, y as i32));
                }
            }
        }
    }

    (antenna_locations, max_x as i32, max_y as i32)
}

fn find_possible_antinodes(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    max_x: i32,
    max_y: i32,
    step: i32,
) -> Vec<(i32, i32)> {
    let mut possible_antinodes = Vec::new();

    let mut border_reached = false;
    let mut i = step;
    while !border_reached {
        let x = x2 + i * (x2 - x1);
        let y = y2 + i * (y2 - y1);
        if x < 0 || y < 0 || x > max_x || y > max_y {
            border_reached = true;
        } else {
            possible_antinodes.push((x, y));
            i += step;
        }
    }

    possible_antinodes
}

fn find_antinodes_for_frequency(
    frequency_antenna_locations: &[(i32, i32)],
    max_x: i32,
    max_y: i32,
    antinode_limit: bool,
) -> HashSet<(i32, i32)> {
    let mut antinode_locations = HashSet::new();

    for (i, (x1, y1)) in frequency_antenna_locations.iter().enumerate() {
        for (j, (x2, y2)) in frequency_antenna_locations.iter().enumerate() {
            if i != j {
                let possible_antinodes =
                    find_possible_antinodes(*x1, *y1, *x2, *y2, max_x, max_y, 1);
                if antinode_limit {
                    if !possible_antinodes.is_empty() {
                        antinode_locations.insert(possible_antinodes[0]);
                    }
                } else {
                    antinode_locations.extend(possible_antinodes);
                    antinode_locations.extend(find_possible_antinodes(
                        *x1, *y1, *x2, *y2, max_x, max_y, -1,
                    ));
                }
            }
        }
    }

    antinode_locations
}

fn main() {
    let input = include_str!("../inputs/data_day_8.txt");
    let (antenna_locations, max_x, max_y) = parse_input(input);

    // Solution for puzzle 1
    let mut unique_antinode_locations = HashSet::new();
    for frequency_antenna_locations in antenna_locations.values() {
        unique_antinode_locations.extend(find_antinodes_for_frequency(
            frequency_antenna_locations,
            max_x,
            max_y,
            true,
        ));
    }
    println!(
        "The antennas cause {} antinodes",
        unique_antinode_locations.len()
    );

    // Solution for puzzle 2
    let mut unique_antinode_locations = HashSet::new();
    for frequency_antenna_locations in antenna_locations.values() {
        unique_antinode_locations.extend(find_antinodes_for_frequency(
            frequency_antenna_locations,
            max_x,
            max_y,
            false,
        ));
    }
    println!(
        "The antennas cause {} antinodes if resonant harmonics are considered",
        unique_antinode_locations.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_possible_antinodes() {
        assert_eq!(
            find_possible_antinodes(2, 2, 3, 3, 5, 5, -1),
            vec![(2, 2), (1, 1), (0, 0)]
        );
    }

    #[test]
    fn test_find_antinodes_for_frequency() {
        let antenna_locations = vec![(4, 3), (8, 4), (5, 5)];
        assert_eq!(
            find_antinodes_for_frequency(&antenna_locations, 10, 10, true),
            HashSet::from([(0, 2), (3, 1), (2, 6), (6, 7)])
        );
    }
}
