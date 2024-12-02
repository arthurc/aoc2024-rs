const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    println!("Total distance: {}", part1::calculate_distance(INPUT));
    println!();
    println!("Part 2:");
    println!(
        "Similarity score: {}",
        part2::calulate_similarity_score(INPUT)
    );
}

fn parse_lists(input: &str) -> [Vec<i32>; 2] {
    input.lines().fold([vec![], vec![]], |mut acc, line| {
        let mut location_id_iter = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        acc[0].push(location_id_iter.next().unwrap());
        acc[1].push(location_id_iter.next().unwrap());
        acc
    })
}

mod part1 {
    pub fn calculate_distance(input: &str) -> i32 {
        let [mut left, mut right] = super::parse_lists(input);

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
        fn test_input() {
            assert_eq!(calculate_distance(crate::INPUT), 2196996);
        }

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

mod part2 {
    use std::collections::HashMap;

    pub fn calulate_similarity_score(input: &str) -> i32 {
        let [left, right] = super::parse_lists(input);

        let mut multiplier_map = HashMap::new();
        for i in right.iter() {
            *multiplier_map.entry(*i).or_insert(0) += 1;
        }

        left.iter().fold(0, |acc, n| {
            let multiplier = *multiplier_map.get(n).unwrap_or(&0);
            acc + n * multiplier
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_input() {
            assert_eq!(calulate_similarity_score(crate::INPUT), 23655822);
        }

        #[test]
        fn test_calculate_similarity_score() {
            assert_eq!(calulate_similarity_score("3 4"), 0);
            assert_eq!(calulate_similarity_score("1 1\n2 1"), 2);
        }

        #[test]
        fn test_example() {
            assert_eq!(
                calulate_similarity_score(
                    "3   4
4   3
2   5
1   3
3   9
3   3"
                ),
                31
            );
        }
    }
}
