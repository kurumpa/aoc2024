use log::debug;
use std::env;
use std::process;
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// for the nice `match (ok, correction) { ... }`
const OK: bool = true;
const ERROR: bool = false;
const ONCE_CORRECTED: bool = true;
const NEVER_CORRECTED: bool = false;

fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|line| {
            let mut last_delta: Option<i32> = None;
            let mut last_value: Option<i32> = None;

            let ok = line
                .split_ascii_whitespace()
                .all(|value| match value.parse::<i32>() {
                    Ok(val) => {
                        let mut ok = true;
                        let mut current_delta: Option<i32> = None;

                        if let Some(lv) = last_value {
                            let delta = val - lv;
                            ok = ok && delta.abs() >= 1 && delta.abs() <= 3;
                            if let Some(ld) = last_delta {
                                ok = ok && delta.signum() == ld.signum();
                            }
                            current_delta = Some(delta);
                        }
                        last_delta = current_delta;
                        last_value = Some(val);
                        ok
                    }
                    Err(_) => false,
                });
            ok
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|line| {
            let cr_tst = || {
                let mut last_delta: Option<i32> = None;
                let mut last_value: Option<i32> = None;
                let mut error_corrected = NEVER_CORRECTED;

                move |value: &str| match value.parse::<i32>() {
                    Ok(val) => {
                        let mut ok = true;
                        let mut current_delta: Option<i32> = None;

                        if let Some(lv) = last_value {
                            let delta = val - lv;
                            ok = ok && delta.abs() >= 1 && delta.abs() <= 3;
                            if let Some(ld) = last_delta {
                                ok = ok && delta.signum() == ld.signum();
                            }
                            current_delta = Some(delta);
                        }

                        match (ok, error_corrected) {
                            (OK, _) => {
                                last_delta = current_delta;
                                last_value = Some(val);
                                OK
                            }
                            (ERROR, ONCE_CORRECTED) => ERROR,
                            (ERROR, NEVER_CORRECTED) => {
                                error_corrected = ONCE_CORRECTED;
                                OK
                            }
                        }
                    }
                    Err(_) => false,
                }
            };
            // TODO window instead of reverse?
            let res1 = line.split_ascii_whitespace().all(cr_tst());
            let res2 = line.split_ascii_whitespace().rev().all(cr_tst());
            res1 || res2
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n";
    const TEST_INPUT2: &str = "2 5 8 7 8";

    #[test]
    fn part1_test() {
        init_logger();
        assert_eq!(part1(TEST_INPUT), 2);
    }
    #[test]
    fn part2_test1() {
        init_logger();
        assert_eq!(part2(TEST_INPUT2), 1);
    }
    #[test]
    fn part2_test() {
        init_logger();
        assert_eq!(part2(TEST_INPUT), 4);
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
        .get("https://adventofcode.com/2024/day/2/input")
        .header("cookie", session_cookie)
        .send()?
        .text_with_charset("utf-8")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
