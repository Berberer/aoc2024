fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace().flat_map(str::parse).collect())
        .collect()
}

fn get_level_diffs(report: &[i32]) -> Vec<i32> {
    report
        .windows(2)
        .map(|level_pair| level_pair[1] - level_pair[0])
        .collect()
}

fn check_save_level_differences(report: &[i32], save_diff_predicate: fn(&i32) -> bool) -> bool {
    get_level_diffs(report).iter().all(save_diff_predicate)
}

fn is_save_report(report: &[i32]) -> bool {
    let all_increasing = check_save_level_differences(report, |diff| *diff > 0 && *diff < 4);
    let all_decreasing = check_save_level_differences(report, |diff| *diff < 0 && *diff > -4);

    all_increasing || all_decreasing
}

fn generate_possible_reports_with_one_missing_level(report: &[i32]) -> Vec<Vec<i32>> {
    (0..report.len())
        .map(|tolerance_index| {
            (0..report.len())
                .filter(|index| *index != tolerance_index)
                .map(|index| report[index])
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_save_report_with_tolerance(report: &[i32]) -> bool {
    generate_possible_reports_with_one_missing_level(report)
        .iter()
        .any(|report_with_one_missing_level| is_save_report(report_with_one_missing_level))
}

fn main() {
    let input = include_str!("../inputs/data_day_2.txt");
    let reports = parse_input(input);

    // Solution for puzzle 1
    let save_reports_count = reports
        .iter()
        .filter(|report| is_save_report(report))
        .count();
    println!("The number of save reports is {save_reports_count}");

    // Solution for puzzle 2
    let save_reports_with_tolerance_count = reports
        .iter()
        .filter(|report| is_save_report_with_tolerance(report))
        .count();
    println!("The number of save reports with tolerance is {save_reports_with_tolerance_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_save_report() {
        assert_eq!(is_save_report(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_save_report(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_save_report(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_save_report(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_save_report(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_save_report(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_generate_possible_reports_with_one_missing_level() {
        assert_eq!(
            generate_possible_reports_with_one_missing_level(&vec![0, 1, 2, 3, 4]),
            vec![
                vec![1, 2, 3, 4],
                vec![0, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 4],
                vec![0, 1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_is_save_report_with_tolerance() {
        assert_eq!(is_save_report_with_tolerance(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_save_report_with_tolerance(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_save_report_with_tolerance(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_save_report_with_tolerance(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(is_save_report_with_tolerance(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(is_save_report_with_tolerance(&vec![1, 3, 6, 7, 9]), true);
    }
}
