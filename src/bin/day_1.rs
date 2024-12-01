use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .unzip()
}

fn calculate_distances(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut left = left.to_vec();
    left.sort();
    let mut right = right.to_vec();
    right.sort();

    left.iter().zip(right).map(|(x, y)| (x - y).abs()).collect()
}

fn calculate_similarities(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut right_occurrences = HashMap::new();
    for r in right {
        *right_occurrences.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .map(|l| l * right_occurrences.get(l).unwrap_or(&0))
        .collect()
}

fn main() {
    let input = include_str!("../inputs/data_day_1.txt");
    let (left, right) = parse_input(input);

    // Solution for puzzle 1
    let sum_of_distances = calculate_distances(&left, &right).iter().sum::<i32>();
    println!("The sum of location ID distances is {sum_of_distances}");

    // Solution for puzzle 2
    let sum_of_similarities = calculate_similarities(&left, &right).iter().sum::<i32>();
    println!("The sum of location ID similarities is {sum_of_similarities}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distances() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_distances(&left, &right), vec![2, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn test_calculate_similarities() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(
            calculate_similarities(&left, &right),
            vec![9, 4, 0, 0, 9, 9]
        );
    }
}
