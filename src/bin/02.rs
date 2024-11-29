use regex::Regex;
use std::sync::LazyLock;
advent_of_code::solution!(2);

#[derive(Debug)]
struct Draw {
    blue: u32,
    red: u32,
    green: u32
}
#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>
}
impl Game {
    fn possible_given(&self, given: &Draw) -> bool {
        self.draws.iter().all(|d| d.possible_given(given))
    }
    fn lowest_possible_count(&self) -> Draw {
       Draw{
            blue: self.draws.iter().map(|d| d.blue).max().unwrap(),
            red: self.draws.iter().map(|d| d.red).max().unwrap(),
            green: self.draws.iter().map(|d| d.green).max().unwrap()
        }
    }
}
impl Draw {
    fn possible_given(&self, given: &Draw) -> bool {
        self.blue <= given.blue && self.red <= given.red && self.green <= given.green
    }
    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+)\s+(green|red|blue)").unwrap());

fn parse_game(line: &str) -> Option<Game> {
    let g: Vec<&str> = line.split(":").collect();
    let first = g[0];
    let rest = g[1];
    let draws = rest.split(";").map(|d| {
        let mut mydraw = Draw{blue: 0, red: 0, green: 0};
        for cap in RE.captures_iter(d) {
            let count: u32 = cap[1].parse().unwrap();
            let color = &cap[2];
            match color {
                "blue" => {mydraw.blue += count}
                "red" => {mydraw.red += count}
                "green" => {mydraw.green += count}
                &_ => {}
            }
        }
        mydraw
    }).collect();
    Some(Game{id: first[5..].parse().ok()?, draws})
}

pub fn part_one(input: &str) -> Option<u32> {
    let a: Vec<Game> = input.split("\n").flat_map(|l| parse_game(l)).collect();
    let given = Draw{blue: 14, red: 12, green: 13};
    let r = a.iter().filter(|g| g.possible_given(&given)).fold(0, |acc, g| acc + g.id);
    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a: Vec<Game> = input.split("\n").flat_map(|l| parse_game(l)).collect();
    let r = a.iter().map(|g| g.lowest_possible_count().power()).sum();
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
