advent_of_code::solution!(1);

struct Number {
    first: u32,
    last: u32
}

impl Number {
    fn combine(first: u32, last: u32) -> Option<u32>{
        format!("{}{}", first, last).parse().ok()
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let r = input
        .split("\n")
        .flat_map(|l| {
            let x = l.chars().find_map(|c| c.to_digit(10));
            let y = l.chars().rev().find_map(|c| c.to_digit(10));
           Some(Number{first: x?, last: y?})
        })
        .flat_map(|n| Number::combine(n.first, n.last))
        .sum();
    Some(r)
}

fn find_first_number(numbers: &Vec<&str>, line: &str) -> Option<u32> {
    for i in 0..line.len() {
        for (j, n) in numbers.iter().enumerate() {
            let end = if i + n.len() < line.len() {  i + n.len()} else { line.len() };
            if &line[i..end] == *n {
                return Some(j as u32 + 1);
            }
        }
        let asciinum = *&line.chars().nth(i)?.to_digit(10);
        match asciinum {Some(num) => return Some(num),
        None => continue}
    }
    todo!("This shouldn't be reachable in AoC");
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = Vec::from(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]);
    let revnums: Vec<String> = numbers.iter().map(|n| n.chars().rev().collect()).collect();
    let a: u32= input.split("\n").flat_map(|l|
        Some(Number{first: find_first_number(&numbers, l)?,
        last: find_first_number(&revnums.iter().map(|s| s.as_str()).collect(), &l.chars().rev().collect::<String>())?})
    ).flat_map(|n| Number::combine(n.first, n.last)).sum();
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
