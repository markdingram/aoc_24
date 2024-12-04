use std::{collections::HashMap, fs, io::Write as _, str::FromStr};

wasi::cli::command::export!(Day4);

struct Day4;

impl wasi::exports::cli::run::Guest for Day4 {
    fn run() -> Result<(), ()> {
        let mut stdout = wasi::cli::stdout::get_stdout();

        let input: String = fs::read_to_string("dat/input.txt").unwrap();
        let r = day4part1(input.clone()).unwrap();
        stdout
            .write_all(format!("Day 4 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        let r = day4part2(input).unwrap();
        stdout
            .write_all(format!("Day 4 Pt 2 Result: {r}\n").as_bytes())
            .unwrap();
        stdout.flush().unwrap();

        Ok(())
    }
}

pub fn day4part1(data: String) -> anyhow::Result<usize> {
    let grid: Grid = data.parse()?;

    Ok(grid.xmas())
}

pub fn day4part2(data: String) -> anyhow::Result<usize> {
    let grid: Grid = data.parse()?;

    Ok(grid.xmas_pt2())
}

pub struct Grid {
    data: Vec<char>,
    cols: usize,
    rows: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut data = vec![];
        let mut cols = 0;
        for line in s.lines() {
            cols = line.len();
            rows += 1;
            data.extend(line.chars());
        }

        Ok(Grid { data, cols, rows })
    }
}

impl Grid {
    fn cells(&self) -> usize {
        self.rows * self.cols
    }

    fn coord(&self, index: usize) -> Coord {
        Coord {
            row: index / self.cols,
            col: index % self.cols,
        }
    }

    #[allow(unused)]
    fn value(&self, index: usize) -> char {
        *self.data.get(index).unwrap()
    }

    fn index(&self, coord: Coord) -> usize {
        (coord.row * self.rows) + coord.col
    }

    fn value_coord(&self, coord: Coord) -> char {
        *self.data.get(self.index(coord)).unwrap()
    }

    fn xmas(&self) -> usize {
        let mut four_letter_words = 0;

        let cells = self.cells();
        for cell in 0..cells {
            let words = self.four_letter_words(cell);
            let xmas = words.iter().filter(|w| *w == "XMAS").count();
            if xmas > 0 {
                //let coord = self.coord(cell);
                //println!("coord: {:?} - words: {:?} - xmas: {xmas}", coord, words);
            }

            four_letter_words += xmas;
        }

        four_letter_words
    }

    /// 8 possible directions from each cell
    fn four_letter_word(&self, index: usize, dir: Direction) -> Option<String> {
        let start = self.coord(index);

        let mut chars = vec![];
        chars.push(self.value_coord(start));
        // 2
        let start = self._move(start, dir)?;
        chars.push(self.value_coord(start));

        // 3
        let start = self._move(start, dir)?;
        chars.push(self.value_coord(start));

        // 4
        let start = self._move(start, dir)?;
        chars.push(self.value_coord(start));

        return Some(chars.iter().collect());
    }

    fn four_letter_words(&self, index: usize) -> Vec<String> {
        let words = vec![
            self.four_letter_word(index, Direction::Up),
            self.four_letter_word(index, Direction::UpRight),
            self.four_letter_word(index, Direction::Right),
            self.four_letter_word(index, Direction::DownRight),
            self.four_letter_word(index, Direction::Down),
            self.four_letter_word(index, Direction::DownLeft),
            self.four_letter_word(index, Direction::Left),
            self.four_letter_word(index, Direction::UpLeft),
        ];
        //println!("{:?}", words);
        words.into_iter().flatten().collect()
    }

    fn xmas_pt2(&self) -> usize {
        let mut middles = vec![];

        let cells = self.cells();
        for cell in 0..cells {
            let words = self.mas_middle_cells(cell);
            middles.extend(words);
        }

        let mut indexes: Vec<usize> = middles.into_iter().map(|c| self.index(c)).collect();
        indexes.sort();

        let mut counts = HashMap::new();
        for item in indexes {
            *counts.entry(item).or_insert(0) += 1;
        }

        //println!("{:?}", counts);

        counts.values().filter(|i| **i > 1).count()
    }

    fn mas_middle_cells(&self, index: usize) -> Vec<Coord> {
        let mas = "MAS";
        let mas = vec![
            self.matching_three_letter_word_middle_coord(index, Direction::UpRight, mas),
            self.matching_three_letter_word_middle_coord(index, Direction::DownRight, mas),
            self.matching_three_letter_word_middle_coord(index, Direction::DownLeft, mas),
            self.matching_three_letter_word_middle_coord(index, Direction::UpLeft, mas),
        ];
        //println!("{:?}", mas);
        mas.into_iter().flatten().collect()
    }

    fn matching_three_letter_word_middle_coord(
        &self,
        index: usize,
        dir: Direction,
        target: &str,
    ) -> Option<Coord> {
        let start = self.coord(index);

        let mut chars = vec![];
        chars.push(self.value_coord(start));
        // 2
        let middle = self._move(start, dir)?;
        chars.push(self.value_coord(middle));

        // 3
        let last = self._move(middle, dir)?;
        chars.push(self.value_coord(last));

        let word: String = chars.iter().collect();

        if word == target {
            Some(middle)
        } else {
            None
        }
    }

    fn _move(&self, start: Coord, dir: Direction) -> Option<Coord> {
        match dir {
            Direction::Up => {
                if start.row > 0 {
                    Some(Coord {
                        col: start.col,
                        row: start.row - 1,
                    })
                } else {
                    None
                }
            }
            Direction::UpRight => {
                if start.row > 0 && start.col < self.cols - 1 {
                    Some(Coord {
                        col: start.col + 1,
                        row: start.row - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if start.col < self.cols - 1 {
                    Some(Coord {
                        col: start.col + 1,
                        row: start.row,
                    })
                } else {
                    None
                }
            }
            Direction::DownRight => {
                if start.row < self.rows - 1 && start.col < self.cols - 1 {
                    Some(Coord {
                        col: start.col + 1,
                        row: start.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if start.row < self.rows - 1 {
                    Some(Coord {
                        col: start.col,
                        row: start.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::DownLeft => {
                if start.row < self.rows - 1 && start.col > 0 {
                    Some(Coord {
                        col: start.col - 1,
                        row: start.row + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if start.col > 0 {
                    Some(Coord {
                        col: start.col - 1,
                        row: start.row,
                    })
                } else {
                    None
                }
            }
            Direction::UpLeft => {
                if start.row > 0 && start.col > 0 {
                    Some(Coord {
                        col: start.col - 1,
                        row: start.row - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord {
    col: usize,
    row: usize,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    pub fn test_grid() {
        let demo: String = fs::read_to_string("dat/demo.txt").unwrap();
        let grid = Grid::from_str(&demo).unwrap();

        assert_eq!(grid.cols, 10);
        assert_eq!(grid.rows, 10);

        assert_eq!(grid.coord(0), Coord { col: 0, row: 0 });
        assert_eq!(grid.value(0), 'M');

        assert_eq!(grid.value_coord(Coord { col: 0, row: 0 }), 'M');

        assert_eq!(
            grid.four_letter_word(0, Direction::Right),
            Some("MMMS".to_string())
        );
        assert_eq!(
            grid.four_letter_word(3, Direction::Left),
            Some("SMMM".to_string())
        );
        assert_eq!(grid.four_letter_word(3, Direction::Up), None);

        assert_eq!(grid.four_letter_words(0), vec!["MMMS", "MSXM", "MMAM"]);

        assert_eq!(grid.value_coord(Coord { col: 9, row: 8 }), 'M');

        assert_eq!(grid.coord(67), Coord { col: 7, row: 6 });

        assert_eq!(
            grid.four_letter_words(67),
            vec!["XAAS", "XAMA", "XSXA", "XSAS", "XXMA"]
        );

        assert_eq!(grid.xmas(), 18);

        assert_eq!(grid.xmas_pt2(), 9);
    }
}
