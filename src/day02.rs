use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| {
            line.windows(2).all(|w| w[0] < w[1] && w[0] + 4 > w[1])
                || line.windows(2).all(|w| w[1] < w[0] && w[1] + 4 > w[0])
        })
        // .inspect(|x| println!("{:?}", x))
        .count()
        .to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| {
            if line.windows(2).all(|w| w[0] < w[1] && w[0] + 4 > w[1])
                || line.windows(2).all(|w| w[1] < w[0] && w[1] + 4 > w[0])
            {
                return true;
            }
            for i in 0..line.len() {
                let new_line = line
                    .iter()
                    .enumerate()
                    .filter(|(j, _x)| i != *j)
                    .map(|(_, x)| *x)
                    .collect::<Vec<u32>>();
                if new_line.windows(2).all(|w| w[0] < w[1] && w[0] + 4 > w[1])
                    || new_line.windows(2).all(|w| w[1] < w[0] && w[1] + 4 > w[0])
                {
                    return true;
                }
            }
            false
        })
        .inspect(|x| println!("{:?}", x))
        .count()
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
                b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            "2"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            "4"
        );
    }
}
