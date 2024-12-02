const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    println!(
        "Total number of safe reports: {}",
        part1::calculate_number_of_safe_reports(INPUT)
    );
}

type Report = Vec<i32>;
trait ReportExt {
    fn is_safe(&self) -> bool;
}
impl ReportExt for Report {
    fn is_safe(&self) -> bool {
        self.windows(2)
            .map(|w| w[1] - w[0])
            .try_fold(0i32, |acc, l| {
                match l.abs() {
                    x if x < 1 || x > 3 => Err(()),
                    x => Ok(x),
                }?;

                match l.signum() * acc.signum() {
                    0 => Ok(l),
                    n if n > 0 => Ok(l),
                    _ => Err(()),
                }
            })
            .is_ok()
    }
}

fn parse_reports(input: &str) -> impl Iterator<Item = Report> + '_ {
    input.lines().map(|s| {
        s.trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    })
}

mod part1 {
    use super::*;

    pub fn calculate_number_of_safe_reports(input: &str) -> usize {
        super::parse_reports(input).filter(|r| r.is_safe()).count()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_input() {
            assert_eq!(calculate_number_of_safe_reports(crate::INPUT), 236);
        }

        #[test]
        fn test_example() {
            assert_eq!(
                calculate_number_of_safe_reports(
                    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
                ),
                2
            );
        }
    }
}
