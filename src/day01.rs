use std::{collections::HashMap, io::BufRead};

pub fn star_one(input: impl BufRead) -> String {
    let mut left: Vec<u32> = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let line = line.unwrap();
        let (l, r) = line.split_once(' ').unwrap();

        left.push(l.trim().parse().unwrap());
        right.push(r.trim().parse().unwrap());
    }

    left.sort();
    right.sort();
    dbg!(&left);
    dbg!(&right);

    left.iter()
        .zip(right.iter())
        .map(|(l, &r)| l.abs_diff(r))
        .inspect(|x| println!("{}", x))
        .sum::<u32>()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();
        let (l, r) = line.split_once(' ').unwrap();

        left.push(l.trim().parse().unwrap());
        *right.entry(r.trim().parse().unwrap()).or_insert(0) += 1;
    }

    left.iter()
        .map(|l| l * right.get(l).unwrap_or(&0))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            "11"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            "31"
        );
    }
}
