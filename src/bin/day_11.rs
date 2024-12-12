use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn change_stone(stone: u64) -> Vec<u64> {
    let s = format!("{stone}");

    if stone == 0 {
        vec![1]
    } else if s.len() % 2 == 0 {
        let center = s.len() / 2;
        vec![s[..center].parse().unwrap(), s[center..].parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

fn execute_blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| change_stone(*stone))
        .collect()
}

fn observe_stone_counts_of_stone(
    stones: &[u64],
    number_of_blinks: u8,
    memoization_cache: HashMap<(u64, u8), usize>,
) -> (usize, HashMap<(u64, u8), usize>) {
    let stones_after_blink = execute_blink(stones);

    if number_of_blinks == 1 {
        (stones_after_blink.len(), memoization_cache)
    } else {
        stones_after_blink
            .iter()
            .fold((0, memoization_cache), |(acc, cache), stone| {
                if cache.contains_key(&(*stone, number_of_blinks - 1)) {
                    let cached_result = *cache.get(&(*stone, number_of_blinks - 1)).unwrap();
                    (acc + cached_result, cache)
                } else {
                    let (new_stones, updated_cache) = observe_stone_counts_of_stone(
                        &[*stone],
                        number_of_blinks - 1,
                        cache.clone(),
                    );
                    (
                        acc + new_stones,
                        cache
                            .into_iter()
                            .chain(updated_cache)
                            .chain([((*stone, number_of_blinks - 1), new_stones)])
                            .collect(),
                    )
                }
            })
    }
}

fn main() {
    let input = include_str!("../inputs/data_day_11.txt");
    let initial_stones = parse_input(input);

    // Solution for puzzle 1
    let (stones, memoization_cache) =
        observe_stone_counts_of_stone(&initial_stones, 25, HashMap::new());
    println!("{stones} stones exist after 25 blinks");

    // Solution for puzzle 2

    println!(
        "{} stones exist after 75 blinks",
        observe_stone_counts_of_stone(&initial_stones, 75, memoization_cache).0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_stone() {
        assert_eq!(change_stone(0), vec![1]);
        assert_eq!(change_stone(1000), vec![10, 0]);
        assert_eq!(change_stone(2), vec![4048]);
    }

    #[test]
    fn test_execute_blink() {
        assert_eq!(
            execute_blink(&vec![0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }

    #[test]
    fn test_observe_stone_counts_of_stone() {
        let (stones, memoization_cache) =
            observe_stone_counts_of_stone(&vec![125, 17], 6, HashMap::new());
        assert_eq!(stones, 22);

        assert_eq!(
            observe_stone_counts_of_stone(&vec![125, 17], 25, memoization_cache).0,
            55312
        );
    }
}
