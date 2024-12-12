use regex::Regex;
use std::io::{read_to_string, BufRead};

pub fn star_one(input: impl BufRead) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let data = read_to_string(input).unwrap();
    re.captures_iter(&data)
        .map(|c| c.extract())
        .map(|(_, [p1, p2])| p1.parse::<u32>().unwrap() * p2.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let data = read_to_string(input).unwrap();
    let mut enabled = true;
    let mut value = 0;
    for start in 0..data.len() {
        if let Some(c) = re.captures(&data[start..]) {
            if enabled {
                let (_, [p1, p2]) = c.extract();
                value += p1.parse::<u32>().unwrap() * p2.parse::<u32>().unwrap();
            }
        } else if data[start..].starts_with("do()") {
            enabled = true;
        } else if data[start..].starts_with("don't()") {
            enabled = false;
        }
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            "161"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            "48"
        );
    }
}
