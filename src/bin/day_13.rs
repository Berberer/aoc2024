#[derive(Debug, Clone)]
struct ClawMachine {
    button_a_movement: (i64, i64),
    button_b_movement: (i64, i64),
    prize_position: (i64, i64),
}

impl ClawMachine {
    fn from_input_block(input: &str) -> Self {
        let mut lines = input.lines();
        let button_a_movement = lines
            .next()
            .map(|l| l.split_once(':').unwrap().1)
            .map(|l| parse_movement(l, '+'))
            .unwrap();
        let button_b_movement = lines
            .next()
            .map(|l| l.split_once(':').unwrap().1)
            .map(|l| parse_movement(l, '+'))
            .unwrap();
        let prize_position = lines
            .next()
            .map(|l| l.split_once(':').unwrap().1)
            .map(|l| parse_movement(l, '='))
            .unwrap();

        Self {
            button_a_movement,
            button_b_movement,
            prize_position,
        }
    }
}

fn parse_movement(s: &str, separator: char) -> (i64, i64) {
    s.split_once(',')
        .map(|(l, r)| {
            (
                l.split_once(separator).unwrap().1.parse().unwrap(),
                r.split_once(separator).unwrap().1.parse().unwrap(),
            )
        })
        .unwrap()
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(ClawMachine::from_input_block)
        .collect()
}

fn find_button_press_combination(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
    let (x_a, y_a) = claw_machine.button_a_movement;
    let (x_b, y_b) = claw_machine.button_b_movement;
    let (x_p, y_p) = claw_machine.prize_position;

    let b = (y_p * x_a - x_p * y_a) / (-x_b * y_a + y_b * x_a);
    let a = (x_p - b * x_b) / x_a;

    if a * x_a + b * x_b == x_p && a * y_a + b * y_b == y_p {
        Some((a, b))
    } else {
        None
    }
}

fn calculate_token_cost(a: i64, b: i64) -> i64 {
    a * 3 + b
}

fn main() {
    let input = include_str!("../inputs/data_day_13.txt");
    let claw_machines = parse_input(input);

    // Solution for puzzle 1
    let min_token_number = claw_machines
        .iter()
        .flat_map(find_button_press_combination)
        .map(|(a, b)| calculate_token_cost(a, b))
        .sum::<i64>();
    println!("To win all possible prizes, at least {min_token_number} has to be spent");

    // Solution for puzzle 2
    let claw_machines = claw_machines
        .iter()
        .cloned()
        .map(
            |ClawMachine {
                 button_a_movement,
                 button_b_movement,
                 prize_position: (x_p, y_p),
             }| ClawMachine {
                button_a_movement,
                button_b_movement,
                prize_position: (x_p + 10000000000000, y_p + 10000000000000),
            },
        )
        .collect::<Vec<ClawMachine>>();
    let min_token_number = claw_machines
        .iter()
        .flat_map(find_button_press_combination)
        .map(|(a, b)| calculate_token_cost(a, b))
        .sum::<i64>();
    println!("To win all possible prizes with corrected prize coordinates, at least {min_token_number} has to be spent");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_button_press_combination() {
        let claw_machine = ClawMachine {
            button_a_movement: (94, 34),
            button_b_movement: (22, 67),
            prize_position: (8400, 5400),
        };
        assert_eq!(find_button_press_combination(&claw_machine), Some((80, 40)));

        let claw_machine = ClawMachine {
            button_a_movement: (26, 66),
            button_b_movement: (67, 21),
            prize_position: (12748, 12176),
        };
        assert_eq!(find_button_press_combination(&claw_machine), None);

        let claw_machine = ClawMachine {
            button_a_movement: (17, 86),
            button_b_movement: (84, 37),
            prize_position: (7870, 6450),
        };
        assert_eq!(find_button_press_combination(&claw_machine), Some((38, 86)));

        let claw_machine = ClawMachine {
            button_a_movement: (69, 23),
            button_b_movement: (27, 71),
            prize_position: (18641, 10279),
        };
        assert_eq!(find_button_press_combination(&claw_machine), None);
    }
}
