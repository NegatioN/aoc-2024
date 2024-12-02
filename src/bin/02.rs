advent_of_code::solution!(2);

fn greater_safe(a: i32, b: i32) -> bool {
    a == b + 1 || a == b + 2 || a == b + 3
}
fn less_safe(a: i32, b: i32) -> bool {
    a == b - 1 || a == b - 2 || a == b - 3
}
pub fn part_one(input: &str) -> Option<u32> {
   let o = input.split("\n").map(|line| {
       let nums: Vec<_> = line.split(" ").flat_map(|c| c.parse::<i32>().ok()).collect();
       let g  = nums.iter().zip(nums[1..].iter()).map(|(a, b)| greater_safe(*a, *b)).all(|x| x);
       let l: _ = nums.iter().zip(nums[1..].iter()).map(|(a, b)| less_safe(*a, *b)).all(|x| x);
       (g || l) as u32
   }).sum();
    Some(o)
}

pub fn part_two(input: &str) -> Option<u32> {
    let o = input.split("\n").map(|line| {
        let nums: Vec<_> = line.split(" ").flat_map(|c| c.parse::<i32>().ok()).collect();
        let mut c = 0;
        for i in 0..nums.len() {
            let mut n = Vec::new();
            n.extend_from_slice(&nums[0..i]);
            n.extend_from_slice(&nums[i+1..]);

            let g  = n.iter().zip(n[1..].iter()).map(|(a, b)| greater_safe(*a, *b)).all(|x| x);
            let l: _ = n.iter().zip(n[1..].iter()).map(|(a, b)| less_safe(*a, *b)).all(|x| x);
            c += (g || l) as u32
        }
        (c > 0) as u32
    }).sum();
    Some(o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
