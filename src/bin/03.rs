use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<d1>\d+),(?<d2>\d+)\)").unwrap();
    Some(re.captures_iter(input).flat_map(|m| {
        let d1 = m.name("d1").map_or("", |d| d.as_str());
        let d2 = m.name("d2").map_or("", |d| d.as_str());
        Some(&d1.parse::<u32>().ok()? * &d2.parse::<u32>().ok()?)
    }).sum::<u32>())
}

fn split_and_keep(input: &str) -> Vec<String> {
    let re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut result = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(input) {
        if mat.start() != last_end {
            result.push(input[last_end..mat.start()].to_string());
        }
        result.push(mat.as_str().to_string());
        last_end = mat.end();
    }
    if last_end != input.len() {
        result.push(input[last_end..].to_string());
    }
    result
}
pub fn part_two(input: &str) -> Option<u32> {
    let x: Vec<_> = split_and_keep(input);
    let mut i = 0;
    let mut s: u32 = 0;
    while i < x.len() {
        if x[i] == "do()" {
            i += 1;
        } else if x[i] == "don't()" {
            i += 2;
        } else {
            s += part_one(x[i].as_str())?;
            i += 1;
        }
    }
    Some(s)
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
        assert_eq!(result, Some(48));
    }
}
