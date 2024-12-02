use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect::<Vec<u8>>();

        if check_report(&report, false) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn check_report(report: &[u8], dampener_active: bool) -> bool {
    let mut c = None;
    eprintln!("{report:?}");

    let mut idx1 = 0;
    let mut idx2 = 1;
    let mut dampening_used = !dampener_active;
    while idx2 < report.len() {
        eprintln!("Cmp: {} {}", report[idx1], report[idx2]);
        match report[idx1].cmp(&report[idx2]) {
            std::cmp::Ordering::Equal => {
                if !dampening_used {
                    eprintln!("Dampening used ==");
                    dampening_used = true;
                } else {
                    return false;
                }
            }
            diff => {
                if report[idx1].abs_diff(report[idx2]) > 3 {
                    if !dampening_used {
                        eprintln!("Dampening used diff > 3");
                        dampening_used = true;
                    } else {
                        return false;
                    }
                } else if let Some(diff_prev) = c {
                    if diff != diff_prev {
                        if !dampening_used {
                            eprintln!("Dampening used switch");
                            dampening_used = true;
                        } else {
                            return false;
                        }
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
        if check_report(&report, true) {
            eprintln!("{report:?} is safe");
            safe_reports += 1;
        } else {
            eprintln!("{report:?} is unsafe");
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
