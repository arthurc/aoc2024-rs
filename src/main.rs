const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    println!("Total distance: {}", part1::calculate_distance(INPUT));
}

mod part1 {
    pub fn calculate_distance(input: &str) -> i32 {
        let [mut left, mut right] = input.lines().fold([vec![], vec![]], |mut acc, line| {
            let mut location_id_iter = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
            acc[0].push(location_id_iter.next().unwrap());
            acc[1].push(location_id_iter.next().unwrap());
            acc
        });

        left.sort();
        right.sort();

        left.iter()
            .zip(right.iter())
            .fold(0, |acc, (l, r)| acc + (l - r).abs())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate_distance() {
            assert_eq!(calculate_distance("3 4"), 1);
            assert_eq!(calculate_distance("3 4\n1 2"), 2);
        }

        #[test]
        fn test_example() {
            assert_eq!(
                calculate_distance(
                    "3   4
4   3
2   5
1   3
3   9
3   3"
                ),
                11
            );
        }
    }
}
