use log::debug;
use regex::Regex;
use std::env;
use std::process;
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).fold(0, |sum, caps| {
        let (_, [left, right]) = caps.extract();
        sum + left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
    })
}

fn part2(input: &str) -> u32 {
    let re =
        Regex::new(r"(?<kw_do>do\(\))|(?<kw_dont>don't\(\))|(mul\((?<left>\d+),(?<right>\d+)\))")
            .unwrap();
    re.captures_iter(input)
        .fold((true, 0), |(enabled, sum), caps| {
            match (
                caps.name("kw_do"),
                caps.name("kw_dont"),
                caps.name("left"),
                caps.name("right"),
            ) {
                (Some(_), _, _, _) => (true, sum),
                (_, Some(_), _, _) => (false, sum),
                (_, _, Some(left), Some(right)) => (
                    enabled,
                    if enabled {
                        sum + left.as_str().parse::<u32>().unwrap()
                            * right.as_str().parse::<u32>().unwrap()
                    } else {
                        sum
                    },
                ),
                _ => panic!("Wrong state"),
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_test() {
        init_logger();
        assert_eq!(part1(TEST_INPUT1), 161);
    }
    #[test]
    fn part2_test() {
        init_logger();
        assert_eq!(part2(TEST_INPUT2), 48);
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
        .get("https://adventofcode.com/2024/day/3/input")
        .header("cookie", session_cookie)
        .send()?
        .text_with_charset("utf-8")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
