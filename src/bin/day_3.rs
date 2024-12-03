use regex::Regex;

#[derive(Debug, PartialEq)]
enum Instruction {
    Multiply(i32, i32),
    Do,
    DoNot,
}

impl Instruction {
    fn new(instruction_code: &str) -> Self {
        if instruction_code.starts_with("mul") {
            let number_regex = Regex::new(r"\d+").unwrap();
            let numbers = number_regex
                .find_iter(instruction_code)
                .map(|number_literal| number_literal.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Self::Multiply(numbers[0], numbers[1])
        } else if instruction_code.starts_with("don't") {
            Self::DoNot
        } else if instruction_code.starts_with("do") {
            Self::Do
        } else {
            panic!("Unknown instruction code: {}", instruction_code);
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let multiply_instruction_regex = Regex::new(r"mul\(\d+,\d+\)|do(?:n't)?\(\)").unwrap();
    multiply_instruction_regex
        .find_iter(input)
        .map(|instruction_match| instruction_match.as_str())
        .collect()
}

fn execute_program(program: &[Instruction], handle_enabling: bool) -> i32 {
    program
        .iter()
        .fold(
            (0, true),
            |(sum, is_enabled), instruction| match instruction {
                Instruction::Multiply(x, y) if is_enabled => (sum + x * y, true),
                Instruction::Do if handle_enabling && !is_enabled => (sum, true),
                Instruction::DoNot if handle_enabling && is_enabled => (sum, false),
                _ => (sum, is_enabled),
            },
        )
        .0
}

fn main() {
    let input = include_str!("../inputs/data_day_3.txt");
    let program = parse_input(input)
        .iter()
        .map(|instruction_code| Instruction::new(instruction_code))
        .collect::<Vec<Instruction>>();

    // Solution for puzzle 1
    let sum = execute_program(&program, false);
    println!(
        "The sum of the multiplication instruction without enabling/disabling instructions is {sum}"
    );

    // Solution for puzzle 2
    let sum = execute_program(&program, true);
    println!(
        "The sum of the multiplication instruction with enabling/disabling instructions is {sum}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("Amul(1,2)Bdo()Cdon't()D"),
            vec!["mul(1,2)", "do()", "don't()"]
        );
    }

    #[test]
    fn test_new_instruction() {
        assert_eq!(Instruction::new("mul(1,2)"), Instruction::Multiply(1, 2));
        assert_eq!(Instruction::new("do()"), Instruction::Do);
        assert_eq!(Instruction::new("don't()"), Instruction::DoNot);
    }

    #[test]
    fn test_execute_program() {
        let program = vec![
            Instruction::Multiply(1, 2),
            Instruction::DoNot,
            Instruction::Multiply(3, 4),
            Instruction::Do,
            Instruction::Multiply(5, 6),
        ];

        assert_eq!(execute_program(&program, false), 44);
        assert_eq!(execute_program(&program, true), 32);
    }
}
