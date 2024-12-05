use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (page_order_rules_input, page_positions_input) = input.split_once("\n\n").unwrap();

    let page_order_specifications = page_order_rules_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
        .collect::<Vec<(i32, i32)>>();
    let mut page_order_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for page_order_specification in page_order_specifications {
        if let Some(pages) = page_order_rules.get_mut(&page_order_specification.0) {
            pages.insert(page_order_specification.1);
        } else {
            page_order_rules.insert(
                page_order_specification.0,
                HashSet::from([page_order_specification.1]),
            );
        }
    }

    let page_orders = page_positions_input
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    (page_order_rules, page_orders)
}

fn map_page_positions(page_order: &[i32]) -> HashMap<i32, usize> {
    page_order
        .iter()
        .enumerate()
        .map(|(position, page)| (*page, position))
        .collect()
}

fn is_valid_page_order(page_order_rules: &HashMap<i32, HashSet<i32>>, page_order: &[i32]) -> bool {
    let page_positions = map_page_positions(page_order);
    for (page, page_position) in &page_positions {
        if let Some(subsequent_pages) = page_order_rules.get(page) {
            for subsequent_page in subsequent_pages {
                if let Some(subsequent_position) = page_positions.get(subsequent_page) {
                    if subsequent_position < page_position {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn correct_incorrect_order(page_order_rules: &HashMap<i32, HashSet<i32>>, page_order: &mut [i32]) {
    page_order.sort_by(|page_1, page_2| {
        if let Some(subsequent_pages) = page_order_rules.get(page_1) {
            if subsequent_pages.contains(page_2) {
                return Ordering::Less;
            }
        }
        if let Some(subsequent_pages) = page_order_rules.get(page_2) {
            if subsequent_pages.contains(page_1) {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    })
}

fn get_middle_page_number(page_order: &[i32]) -> i32 {
    let middle_page_index = (page_order.len() as f64 / 2.0).floor() as usize;
    page_order[middle_page_index]
}

fn main() {
    let input = include_str!("../inputs/data_day_5.txt");
    let (page_order_rules, page_orders) = parse_input(input);
    let (correctly_ordered_updates, mut incorrectly_ordered_updates): (
        Vec<Vec<i32>>,
        Vec<Vec<i32>>,
    ) = page_orders
        .iter()
        .cloned()
        .partition(|page_order| is_valid_page_order(&page_order_rules, page_order));

    // Solution for puzzle 1
    let sum_of_middle_page_numbers_of_correct_orders = correctly_ordered_updates
        .iter()
        .map(|correct_page_order| get_middle_page_number(correct_page_order))
        .sum::<i32>();
    println!(
        "The sum of the middle pages of the correctly-ordered updates is {sum_of_middle_page_numbers_of_correct_orders}"
    );

    // Solution for puzzle 2
    let mut sum_of_corrected_orders = 0;
    for incorrect_order in incorrectly_ordered_updates.iter_mut() {
        correct_incorrect_order(&page_order_rules, incorrect_order);
        sum_of_corrected_orders += get_middle_page_number(incorrect_order);
    }
    println!(
        "The sum of the middle pages of the incorrectly-ordered updates after corrections is {sum_of_corrected_orders}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_page_positions() {
        let pages = vec![1, 2, 3];
        assert_eq!(
            map_page_positions(&pages),
            HashMap::from([(1, 0), (2, 1), (3, 2)])
        );
    }

    #[test]
    fn test_valid_page_order_check() {
        let page_order_rules =
            HashMap::from([(1, HashSet::from([2, 3])), (2, HashSet::from([3, 4, 5]))]);

        assert_eq!(is_valid_page_order(&page_order_rules, &[1, 2, 3]), true);
        assert_eq!(is_valid_page_order(&page_order_rules, &[3, 2, 1]), false);
    }

    #[test]
    fn test_get_middle_page_number() {
        assert_eq!(get_middle_page_number(&vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_get_correct_order() {
        let page_order_rules =
            HashMap::from([(1, HashSet::from([2, 3])), (2, HashSet::from([3, 4, 5]))]);
        let mut page_order = vec![3, 2, 1];
        correct_incorrect_order(&page_order_rules, &mut page_order);
        assert_eq!(page_order, vec![1, 2, 3])
    }
}
