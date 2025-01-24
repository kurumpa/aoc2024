use log::debug;
use regex::Regex;
use std::env;
use std::iter;
use std::process;
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn part1(input: &str) -> u32 {
    // iterate adding:
    let opts: [(usize, isize, char); 4] = [
        (0, 1, '→'),  // 0 lines, 1 char
        (1, 0, '↓'),  // 1 line, 0 char
        (1, 1, '↘'),  // 1 line, 1 char
        (1, -1, '↙'), // 1 line, -1 char
    ];
    let search_word: Vec<Option<char>> = "XMAS".chars().map(Some).collect();
    let word_len = search_word.len();
    let lines_vec: Vec<_> = input.lines().map(|l| l.trim()).collect();
    // we believe the input is correct
    let line_len = lines_vec.get(0).unwrap().len();
    let lines_count = lines_vec.iter().count();
    debug!("line_len {}, lines_count {}", line_len, lines_count);

    (0..lines_count)
        .flat_map(|line_index| std::iter::repeat(line_index).zip(0..line_len))
        .flat_map(|(line_index, char_index)| {
            std::iter::repeat((line_index, char_index)).zip(opts.iter())
        })
        .filter(
            |((line_index, char_index), (line_delta, char_delta, dir))| {
                let built_word: Vec<Option<char>> = (0..word_len)
                    .map(|i| {
                        lines_vec
                            .get(line_index + i * line_delta)
                            .map(|line| {
                                let ch = char_index
                                    .checked_add_signed((i as isize) * char_delta)
                                    .and_then(|valid_char_index| {
                                        line.chars().nth(valid_char_index)
                                    });
                                ch
                            })
                            .flatten()
                    })
                    .collect();

                let res = built_word.iter().eq(search_word.iter())
                    || built_word.iter().eq(search_word.iter().rev());
                debug!(
                    "line {} char {} {} {:?} {}",
                    line_index,
                    char_index,
                    dir,
                    String::from_iter(built_word.into_iter().map(|ch| ch.unwrap_or('-'))),
                    res
                );
                res
            },
        )
        .count() as u32
}

fn part2(input: &str) -> u32 {
    // ↘ (MAS || SAM) && ↙ (MAS || SAM)
    let lines_vec: Vec<_> = input.lines().map(|l| l.trim()).collect();
    // we believe the input is correct
    let line_len = lines_vec.get(0).unwrap().len();
    let lines_count = lines_vec.iter().count();

    let search_word: Vec<Option<char>> = "MAS".chars().map(Some).collect();

    debug!("line_len {}, lines_count {}", line_len, lines_count);

    (0..lines_count)
        .flat_map(|line_index| std::iter::repeat(line_index).zip(0..line_len))
        .filter(|(line_index, char_index)| {
            // delta line, delta char
            let criss: Vec<_> = [(0, 0), (1, 1), (2, 2)]
                .iter()
                .map(|(line_delta, char_delta)| {
                    lines_vec
                        .get(line_index + line_delta)
                        .map(|line| line.chars().nth(char_index + char_delta))
                        .flatten()
                })
                .collect();
            let cross: Vec<_> = [(0, 2), (1, 1), (2, 0)]
                .iter()
                .map(|(line_delta, char_delta)| {
                    lines_vec
                        .get(line_index + line_delta)
                        .map(|line| line.chars().nth(char_index + char_delta))
                        .flatten()
                })
                .collect();

            debug!(
                "line {}, char {}, criss {:?}, cross: {:?}",
                line_index, char_index, criss, cross
            );
            let res = true
                && (criss.iter().eq(search_word.iter())
                    || criss.iter().eq(search_word.iter().rev()))
                && (cross.iter().eq(search_word.iter())
                    || cross.iter().eq(search_word.iter().rev()));
            res
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT0: &str = "\
        ..X...\n\
        .SAMX.\n\
        .A..A.\n\
        XMAS.S\n\
        .X....";

    const TEST_INPUT1: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

    const TEST_INPUT2: &str = "\
        .M.S......\n\
        ..A..MSMS.\n\
        .M.S.MAA..\n\
        ..A.ASMSM.\n\
        .M.S.M....\n\
        ..........\n\
        S.S.S.S.S.\n\
        .A.A.A.A..\n\
        M.M.M.M.M.\n\
        ..........";

    #[test]
    fn part0_test() {
        init_logger();
        assert_eq!(part1(TEST_INPUT0), 4);
    }

    #[test]
    fn part1_test() {
        init_logger();
        assert_eq!(part1(TEST_INPUT1), 18);
    }
    #[test]
    fn part2_test() {
        init_logger();
        assert_eq!(part2(TEST_INPUT2), 9);
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
        .get("https://adventofcode.com/2024/day/4/input")
        .header("cookie", session_cookie)
        .send()?
        .text_with_charset("utf-8")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
