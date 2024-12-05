#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn get_all() -> Vec<Direction> {
        vec![
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
            Self::UpLeft,
        ]
    }

    fn get_next_coordinates(&self, current_coordinates: (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (current_coordinates.0, current_coordinates.1 - 1),
            Self::UpRight => (current_coordinates.0 + 1, current_coordinates.1 - 1),
            Self::Right => (current_coordinates.0 + 1, current_coordinates.1),
            Self::DownRight => (current_coordinates.0 + 1, current_coordinates.1 + 1),
            Self::Down => (current_coordinates.0, current_coordinates.1 + 1),
            Self::DownLeft => (current_coordinates.0 - 1, current_coordinates.1 + 1),
            Self::Left => (current_coordinates.0 - 1, current_coordinates.1),
            Self::UpLeft => (current_coordinates.0 - 1, current_coordinates.1 - 1),
        }
    }

    fn are_cross_directions(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Self::UpLeft, Self::UpRight)
                | (Self::UpLeft, Self::DownLeft)
                | (Self::UpRight, Self::UpLeft)
                | (Self::UpRight, Self::DownRight)
                | (Self::DownLeft, Self::UpLeft)
                | (Self::DownLeft, Self::DownRight)
                | (Self::DownRight, Self::UpRight)
                | (Self::DownRight, Self::DownLeft)
        )
    }
}

#[derive(Debug)]
struct WordMatch {
    coordinates: Vec<(usize, usize)>,
    direction: Direction,
}

impl WordMatch {
    fn crosses_match(&self, other: &WordMatch) -> bool {
        let center_index = (self.coordinates.len() as f64 / 2.0).floor() as usize;

        self.coordinates[center_index] == other.coordinates[center_index]
            && self.direction.are_cross_directions(&other.direction)
    }
}

struct WordSearchPuzzle {
    letters: Vec<Vec<char>>,
}

impl WordSearchPuzzle {
    fn are_valid_coordinates(&self, coordinates: &(i32, i32)) -> bool {
        coordinates.0 >= 0
            && coordinates.1 >= 0
            && coordinates.0 < self.letters.len() as i32
            && coordinates.1 < self.letters[0].len() as i32
    }

    fn find_letters(&self, search_letter: u8) -> Vec<(usize, usize)> {
        let mut coordinates = Vec::new();

        for (y, line) in self.letters.iter().enumerate() {
            for (x, letter) in line.iter().enumerate() {
                if *letter as u8 == search_letter {
                    coordinates.push((x, y));
                }
            }
        }

        coordinates
    }

    fn get_letters_in_direction(
        &self,
        start_coordinates: &(usize, usize),
        direction: Direction,
    ) -> Vec<((usize, usize), char)> {
        let mut letters_in_direction = vec![(
            *start_coordinates,
            self.letters[start_coordinates.1][start_coordinates.0],
        )];
        let mut next_coordinates = direction
            .get_next_coordinates((start_coordinates.0 as i32, start_coordinates.1 as i32));

        while self.are_valid_coordinates(&next_coordinates) {
            let coordinates_as_indices = (next_coordinates.0 as usize, next_coordinates.1 as usize);
            letters_in_direction.push((
                coordinates_as_indices,
                self.letters[coordinates_as_indices.1][coordinates_as_indices.0],
            ));
            next_coordinates = direction.get_next_coordinates(next_coordinates);
        }

        letters_in_direction
    }
}

fn parse_input(input: &str) -> WordSearchPuzzle {
    WordSearchPuzzle {
        letters: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn are_letters_in_direction_word_match(
    letters_in_direction: Vec<((usize, usize), char)>,
    direction: Direction,
    expected_letter_sequence: &[u8],
) -> Option<WordMatch> {
    let mut coordinates = Vec::new();

    for (index, expected_letter) in expected_letter_sequence.iter().enumerate() {
        if index >= letters_in_direction.len()
            || *expected_letter != letters_in_direction[index].1 as u8
        {
            return None;
        } else {
            coordinates.push(letters_in_direction[index].0);
        }
    }

    Some(WordMatch {
        coordinates,
        direction,
    })
}

fn find_word_matches(
    word_search_puzzle: &WordSearchPuzzle,
    expected_letter_sequence: &str,
) -> Vec<WordMatch> {
    let mut matches = Vec::new();
    let all_directions = Direction::get_all();
    let expected_letter_sequence = expected_letter_sequence.as_bytes();
    let possible_start_coordinates = word_search_puzzle.find_letters(expected_letter_sequence[0]);

    for start_coordinates in possible_start_coordinates {
        for direction in &all_directions {
            let letters_in_direction =
                word_search_puzzle.get_letters_in_direction(&start_coordinates, *direction);
            if let Some(word_match) = are_letters_in_direction_word_match(
                letters_in_direction,
                *direction,
                expected_letter_sequence,
            ) {
                matches.push(word_match);
            }
        }
    }

    matches
}

fn main() {
    let input = include_str!("../inputs/data_day_4.txt");
    let word_search_puzzle = parse_input(input);

    // Solution for puzzle 1
    let xmas_matches = find_word_matches(&word_search_puzzle, "XMAS");
    println!(
        "XMAS is found {} times in the word search",
        xmas_matches.len()
    );

    // Solution for puzzle 2
    let mas_matches = find_word_matches(&word_search_puzzle, "MAS");
    let mut cross_match_counter = 0;
    for (i, mas_match_1) in mas_matches.iter().enumerate() {
        for m_match_2 in mas_matches[(i + 1)..].iter() {
            if mas_match_1.crosses_match(m_match_2) {
                cross_match_counter += 1;
            }
        }
    }
    println!("MAS is found {cross_match_counter} times in an X shape in the word search");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_coordinates() {
        assert_eq!(Direction::Up.get_next_coordinates((1, 1)), (1, 0));
        assert_eq!(Direction::UpRight.get_next_coordinates((1, 1)), (2, 0));
        assert_eq!(Direction::Right.get_next_coordinates((1, 1)), (2, 1));
        assert_eq!(Direction::DownRight.get_next_coordinates((1, 1)), (2, 2));
        assert_eq!(Direction::Down.get_next_coordinates((1, 1)), (1, 2));
        assert_eq!(Direction::DownLeft.get_next_coordinates((1, 1)), (0, 2));
        assert_eq!(Direction::Left.get_next_coordinates((1, 1)), (0, 1));
        assert_eq!(Direction::UpLeft.get_next_coordinates((1, 1)), (0, 0));
    }

    #[test]
    fn test_valid_coordinates_check() {
        let word_search_puzzle = WordSearchPuzzle {
            letters: vec![vec!['a', 'b'], vec!['c', 'd']],
        };

        assert_eq!(word_search_puzzle.are_valid_coordinates(&(-1, -1)), false);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(-1, 0)), false);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(0, -1)), false);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(0, 0)), true);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(1, 0)), true);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(0, 1)), true);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(1, 1)), true);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(2, 1)), false);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(1, 2)), false);
        assert_eq!(word_search_puzzle.are_valid_coordinates(&(2, 2)), false);
    }

    #[test]
    fn test_find_letters() {
        let word_search_puzzle = WordSearchPuzzle {
            letters: vec![vec!['x', 'b'], vec!['c', 'x']],
        };

        assert_eq!(
            word_search_puzzle.find_letters('x' as u8),
            vec![(0, 0), (1, 1)]
        );
    }

    #[test]
    fn test_get_letters_in_direction() {
        let word_search_puzzle = WordSearchPuzzle {
            letters: vec![vec!['a', 'b'], vec!['c', 'd']],
        };

        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::Up),
            vec![((0, 0), 'a')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::UpRight),
            vec![((0, 0), 'a')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::Right),
            vec![((0, 0), 'a'), ((1, 0), 'b')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::DownRight),
            vec![((0, 0), 'a'), ((1, 1), 'd')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::Down),
            vec![((0, 0), 'a'), ((0, 1), 'c')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::DownLeft),
            vec![((0, 0), 'a')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::Left),
            vec![((0, 0), 'a')]
        );
        assert_eq!(
            word_search_puzzle.get_letters_in_direction(&(0, 0), Direction::UpLeft),
            vec![((0, 0), 'a')]
        );
    }

    #[test]
    fn test_cross_match_check() {
        let word_match = WordMatch {
            coordinates: vec![(0, 0), (1, 1), (2, 2)],
            direction: Direction::DownRight,
        };

        assert_eq!(
            word_match.crosses_match(&WordMatch {
                coordinates: vec![(3, 0), (2, 1), (1, 2)],
                direction: Direction::DownLeft
            }),
            false
        );

        assert_eq!(
            word_match.crosses_match(&WordMatch {
                coordinates: vec![(2, 0), (1, 1), (0, 2)],
                direction: Direction::DownLeft
            }),
            true
        );

        let word_match = WordMatch {
            coordinates: vec![(2, 0), (1, 1), (0, 2)],
            direction: Direction::DownLeft,
        };

        assert_eq!(
            word_match.crosses_match(&WordMatch {
                coordinates: vec![(0, 0), (1, 1), (2, 2)],
                direction: Direction::DownRight,
            }),
            true
        );
    }
}
