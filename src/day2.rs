use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect::<Vec<u8>>();

        if check_report(&report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn check_report(report: &[u8]) -> bool {
    let mut c = None;
    let mut idx1 = 0;
    let mut idx2 = 1;
    while idx2 < report.len() {
        match report[idx1].cmp(&report[idx2]) {
            std::cmp::Ordering::Equal => {
                return false;
            }
            diff => {
                if report[idx1].abs_diff(report[idx2]) > 3 {
                    return false;
                } else if let Some(diff_prev) = c {
                    if diff != diff_prev {
                        return false;
                    }
                } else {
                    c = Some(diff);
                }
            }
        }
        idx1 += 1;
        idx2 += 1;
    }
    true
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect::<Vec<u8>>();
        if check_report(&report) {
            safe_reports += 1;
        } else {
            for skip in 0..report.len() {
                let mut report = report.clone();
                report.remove(skip);
                if check_report(&report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            4
        );
    }
}
