use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((d+),(d+)\)").unwrap();
    println!("{:?}", input);
    for c in re.captures_iter(input) {
       println!("{:?}", &c[0]);
        println!("{:?}", &c[1]);
    }
    Some(re.captures_iter(input).flat_map(|m| {
        println!("{:?}", &m[0]);
        println!("{:?}", &m[1]);
        Some((&m[0].parse::<u32>().ok()? * &m[1].parse::<u32>().ok()?))
    }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
