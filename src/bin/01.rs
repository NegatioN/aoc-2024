use std::collections::HashMap;
use rayon::prelude::*;
advent_of_code::solution!(1);

fn parse_nums(input: &str) -> (Vec<u32>, Vec<u32>) {
    let sep: &str = "\n";
    input.as_parallel_string().split(sep).flat_map(|l| {
        let a: Vec<_> = l.split(" ").collect();
        Some((a.first()?.parse::<u32>().ok()?, a.last()?.parse::<u32>().ok()?))
    }).unzip()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_nums(input);
    left.par_sort();
    right.par_sort();
    Some(left.into_par_iter().zip(right).map(|(a, b)|  a.abs_diff(b) as u64).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_nums(input);
    let mut rm: HashMap<u32, u32> = HashMap::new();
    for (a, b) in left.iter().zip(right) {
        *rm.entry(b).or_insert(0) += 1;
    }
    Some(left.par_iter().map(|&a| (a * rm.get(&a).unwrap_or(&0)) as u64).sum())
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
