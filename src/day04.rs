use std::{collections::HashMap, io::BufRead};

pub fn star_one(input: impl BufRead) -> String {
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];
    let xmas = ['X', 'M', 'A', 'S'];
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<_>>();

    let mut stack = vec![];

    for (r, row) in data.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if cell == &'X' {
                stack.extend(directions.iter().map(|dir| ((r, c), dir, 1)));
            }
        }
    }

    let mut count = 0;
    while let Some((pos, dir, index)) = stack.pop() {
        let new_r = pos.0 as i32 + dir.0;
        let new_c = pos.1 as i32 + dir.1;
        if new_r < 0 || new_c < 0 {
            continue;
        }
        let new_r = new_r as usize;
        let new_c = new_c as usize;

        if data
            .get(new_r)
            .and_then(|row| row.get(new_c))
            .map(|cell| cell == &xmas[index])
            .unwrap_or(false)
        {
            if index == xmas.len() - 1 {
                count += 1;
            } else {
                stack.push(((new_r, new_c), dir, index + 1));
            }
        }
    }
    count.to_string()
}

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get(&self, r: i32, c: i32) -> Option<&char> {
        if r < 0 || c < 0 {
            return None;
        }
        self.0.get(r as usize).and_then(|row| row.get(c as usize))
    }

    fn matches_pattern(&self, pattern: HashMap<(i32, i32), char>) -> bool {
        for (pos, cell) in pattern.iter() {
            if self.get(pos.0, pos.1).map(|c| c != cell).unwrap_or(true) {
                return false;
            }
        }
        true
    }
}

pub fn star_two(input: impl BufRead) -> String {
    let directions: [Directions; 4] = [
        Directions::Left,
        Directions::Up,
        Directions::Right,
        Directions::Down,
    ];
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<_>>();

    let grid = Grid(data);

    grid.0
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, cell)| {
                    let mut count = 0;
                    if cell == &'A' {
                        let r = r as i32;
                        let c = c as i32;
                        // Directions are where the Ms are
                        for dir in directions.iter() {
                            let pattern = match dir {
                                Directions::Up => vec![
                                    ((r - 1, c - 1), 'M'),
                                    ((r - 1, c + 1), 'M'),
                                    ((r + 1, c - 1), 'S'),
                                    ((r + 1, c + 1), 'S'),
                                ]
                                .into_iter()
                                .collect::<HashMap<_, _>>(),
                                Directions::Down => vec![
                                    ((r - 1, c - 1), 'S'),
                                    ((r - 1, c + 1), 'S'),
                                    ((r + 1, c - 1), 'M'),
                                    ((r + 1, c + 1), 'M'),
                                ]
                                .into_iter()
                                .collect::<HashMap<_, _>>(),
                                Directions::Left => vec![
                                    ((r - 1, c - 1), 'M'),
                                    ((r - 1, c + 1), 'S'),
                                    ((r + 1, c - 1), 'M'),
                                    ((r + 1, c + 1), 'S'),
                                ]
                                .into_iter()
                                .collect::<HashMap<_, _>>(),
                                Directions::Right => vec![
                                    ((r - 1, c - 1), 'S'),
                                    ((r - 1, c + 1), 'M'),
                                    ((r + 1, c - 1), 'S'),
                                    ((r + 1, c + 1), 'M'),
                                ]
                                .into_iter()
                                .collect::<HashMap<_, _>>(),
                            };
                            if grid.matches_pattern(pattern) {
                                count += 1;
                            }
                        }
                    }
                    // println!("{}, {}: {}", r, c, count);
                    count
                })
                .sum::<usize>()
        })
        .sum::<usize>()
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
                b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            "18"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            "9"
        );
    }
}
