use std::collections::HashMap;
advent_of_code::solution!(1);

fn parse_nums(input: &str) -> (Vec<u32>, Vec<u32>) {
    let a: Vec<(u32, u32)> = input.split("\n").flat_map(|l| {
        let a: Vec<_> = l.split(" ").collect();
        Some((a.first()?.parse::<u32>().unwrap(), a.last()?.parse::<u32>().unwrap()))
    }).collect();
    (a.iter().map(|(a, b)| a.clone()).collect(), a.iter().map(|(a, b)| b.clone()).collect())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_nums(input);
    left.sort();
    right.sort();
    Some(left.into_iter().zip(right).map(|(a, b)|  a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_nums(input);
    let mut m: HashMap<u32, u32> = HashMap::new();
    for &a in right.iter() {
        *m.entry(a).or_insert(0) += 1;
    }
    Some(left.iter().map(|&a| a * m.get(&a).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
