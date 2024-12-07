#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(test_value, numbers)| {
            (
                test_value.parse().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn get_possible_operator_combinations(
    required_operator_amount: usize,
    allowed_operators: &[Operator],
) -> Vec<Vec<Operator>> {
    if required_operator_amount == 1 {
        allowed_operators
            .iter()
            .map(|operator| vec![*operator])
            .collect()
    } else {
        let combinations =
            get_possible_operator_combinations(required_operator_amount - 1, allowed_operators);
        let mut extended_combinations = Vec::new();

        for allowed_operator in allowed_operators {
            for combination in &combinations {
                let mut extended_combination = vec![*allowed_operator];
                extended_combination.extend(combination.clone());
                extended_combinations.push(extended_combination);
            }
        }

        extended_combinations
    }
}

fn execute_calculation(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];

    for i in 1..numbers.len() {
        result = match operators[i - 1] {
            Operator::Plus => result + numbers[i],
            Operator::Multiply => result * numbers[i],
            Operator::Concatenate => format!("{result}{}", numbers[i]).parse().unwrap(),
        };
    }

    result
}

fn find_valid_operator_combinations_for_equation(
    test_value: i64,
    numbers: &[i64],
    allowed_operators: &[Operator],
) -> Vec<Vec<Operator>> {
    get_possible_operator_combinations(numbers.len() - 1, allowed_operators)
        .iter()
        .filter(|operator_combination| {
            execute_calculation(numbers, operator_combination) == test_value
        })
        .cloned()
        .collect()
}

fn get_sum_of_test_values_of_solvable_test_equations(
    test_equations: &Vec<(i64, Vec<i64>)>,
    allowed_operators: &[Operator],
) -> i64 {
    let mut sum_of_solvable_test_equations = 0;

    for (test_value, numbers) in test_equations {
        let valid_operator_combinations =
            find_valid_operator_combinations_for_equation(*test_value, numbers, allowed_operators);
        if !valid_operator_combinations.is_empty() {
            sum_of_solvable_test_equations += test_value;
        }
    }

    sum_of_solvable_test_equations
}

fn main() {
    let input = include_str!("../inputs/data_day_7.txt");
    let test_equations = parse_input(input);

    // Solution for puzzle 1
    let sum_of_solvable_test_equations_with_plus_and_multiply =
        get_sum_of_test_values_of_solvable_test_equations(
            &test_equations,
            &[Operator::Plus, Operator::Multiply],
        );
    println!(
        "The sum of the test values of solvable equations with Addition and Multiplication is {sum_of_solvable_test_equations_with_plus_and_multiply}"
    );

    // Solution for puzzle 2
    let sum_of_solvable_test_equations_with_all_operators =
        get_sum_of_test_values_of_solvable_test_equations(
            &test_equations,
            &[Operator::Plus, Operator::Multiply, Operator::Concatenate],
        );
    println!(
        "The sum of the test values of solvable equations with Addition, Multiplication, and Concatenation is {sum_of_solvable_test_equations_with_all_operators}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_operator_combinations() {
        assert_eq!(
            get_possible_operator_combinations(1, &[Operator::Plus, Operator::Multiply]),
            vec![vec![Operator::Plus], vec![Operator::Multiply]]
        );

        assert_eq!(
            get_possible_operator_combinations(3, &[Operator::Plus, Operator::Multiply]),
            vec![
                vec![Operator::Plus, Operator::Plus, Operator::Plus],
                vec![Operator::Plus, Operator::Plus, Operator::Multiply],
                vec![Operator::Plus, Operator::Multiply, Operator::Plus],
                vec![Operator::Plus, Operator::Multiply, Operator::Multiply],
                vec![Operator::Multiply, Operator::Plus, Operator::Plus],
                vec![Operator::Multiply, Operator::Plus, Operator::Multiply],
                vec![Operator::Multiply, Operator::Multiply, Operator::Plus],
                vec![Operator::Multiply, Operator::Multiply, Operator::Multiply],
            ]
        );
    }

    #[test]
    fn test_execute_calculation() {
        assert_eq!(
            execute_calculation(
                &[1, 2, 3, 4],
                &[Operator::Plus, Operator::Multiply, Operator::Concatenate]
            ),
            94
        );
    }

    #[test]
    fn test_find_valid_operator_combinations_for_equation() {
        let valid_operator_combinations = find_valid_operator_combinations_for_equation(
            3267,
            &[81, 40, 27],
            &[Operator::Plus, Operator::Multiply],
        );
        assert_eq!(
            valid_operator_combinations,
            vec![
                vec![Operator::Plus, Operator::Multiply],
                vec![Operator::Multiply, Operator::Plus]
            ]
        );
    }
}
