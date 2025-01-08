use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::process;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
// 2 columns, x rows each
// todo: tuple + error handling
fn make_mut_lists(input: &str) -> Vec<Vec<u32>> {
    let rows = input.trim().split("\n");
    let numbers_separator = Regex::new(r"\s+").unwrap();
    let mut lists: Vec<Vec<u32>> = vec![];
    // fill in left and right columns
    for row in rows {
        for (column_index, value) in numbers_separator.split(row).enumerate() {
            if lists.len() < column_index + 1 {
                lists.push(vec![]);
            }
            lists[column_index].push(value.parse::<u32>().unwrap());
        }
    }
    debug!("Lists are {:?}", lists);
    lists
}

fn part1(input: &str) -> u32 {
    let mut lists = make_mut_lists(input);
    // sort columns
    lists.iter_mut().for_each(|col| col.sort());
    // sum distances
    lists[0]
        .iter()
        .zip(lists[1].iter())
        .fold(0, |sum, (a, b)| sum + a.abs_diff(*(b)))
}

fn part2(input: &str) -> u32 {
    let lists = make_mut_lists(input);
    let mut counts = HashMap::new();
    for item in lists[1].iter() {
        *counts.entry(item).or_insert(0) += 1;
    }
    lists[0]
        .iter()
        .fold(0, |sum, item| sum + item * *counts.get(item).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    #[test]
    fn part1_test() {
        init_logger();
        assert_eq!(part1(TEST_INPUT), 11);
    }
    #[test]
    fn part2_test() {
        init_logger();
        assert_eq!(part2(TEST_INPUT), 31);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let client = reqwest::blocking::Client::new();
    let session_cookie = match env::var("SESSION_COOKIE") {
        Ok(val) => val,
        Err(_) => {
            println!("SESSION_COOKIE is not set");
            process::exit(1);
        }
    };
    let input = client
        .get("https://adventofcode.com/2024/day/1/input")
        .header("cookie", session_cookie)
        .send()?
        .text_with_charset("utf-8")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
