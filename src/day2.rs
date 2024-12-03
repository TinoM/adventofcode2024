use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
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

fn check_report_opt(report: &[u8]) -> (bool,usize) {
    let mut c = None;
    let mut idx1 = 0;
    let mut idx2 = 1;
    while idx2 < report.len() {
        match report[idx1].cmp(&report[idx2]) {
            std::cmp::Ordering::Equal => {
                return (false,idx1);
            }
            diff => {
                if report[idx1].abs_diff(report[idx2]) > 3 {
                    return (false,idx1);
                } else if let Some(diff_prev) = c {
                    if diff != diff_prev {
                        return (false,idx1);
                    }
                } else {
                    c = Some(diff);
                }
            }
        }
        idx1 += 1;
        idx2 += 1;
    }
    (true,0)
}

#[aoc(day2, part2)]
fn part2_old(input: &str) -> usize {
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

#[aoc(day2, part2,optimize)]
pub fn part2(input: &str) -> usize {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect::<Vec<u8>>();
        let check =check_report_opt(&report);
        if  check.0 {
            // eprintln!("safe report: {:?}",report);
            safe_reports += 1;
        } else {
            let mut report2 = report.clone();
            report2.remove(check.1);
            // eprintln!("unsafe report: {:?},trying {:?}",report,report2);
            if check_report(&report2) {
                safe_reports += 1;
                // eprintln!("safed report: {:?}",report2);
                continue;
            }
            if check.1 < report.len()-1 {
                let mut report2 = report.clone();
                report2.remove(check.1+1);
                // eprintln!("unsafe report: {:?},trying {:?}",report,report2);
                if check_report(&report2) {
                    safe_reports += 1;
                    // eprintln!("safed report: {:?}",report2);
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

    #[test]
    fn test_part2_old() {
        assert_eq!(
            part2_old("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            4
        );
    }

    #[test]
    fn test_part2_speed() {
        assert_eq!(
            part2("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            4
        );
    }
}
