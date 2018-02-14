use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Taquin {
    n: usize,
    pub pieces: Vec<u64>,
}

impl Taquin {
    pub fn new(n: usize, pieces: Vec<u64>) -> Self {
        assert_eq!(pieces.len(), n * n);
        Taquin { n, pieces }
    }
    pub fn spiral(n: usize) -> Self {
        let mut pieces: Vec<u64> = vec![0; n * n];
        enum Dir {
            Right,
            Up,
            Down,
            Left,
        }
        struct Move {
            dir: Dir,
            n: usize,
        }
        impl Move {
            fn change_dir(&mut self) {
                self.dir = match self.dir {
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                    Dir::Up => Dir::Right,
                }
            }
            fn next(&self, i: usize) -> Option<usize> {
                match self.dir {
                    Dir::Right => {
                        if (i + 1) % self.n != 0 {
                            Some(i + 1)
                        } else {
                            None
                        }
                    }
                    Dir::Down => {
                        if i + self.n < self.n * self.n {
                            Some(i + self.n)
                        } else {
                            None
                        }
                    }
                    Dir::Left => {
                        if (i - 1) % self.n != self.n - 1 {
                            Some(i - 1)
                        } else {
                            None
                        }
                    }
                    Dir::Up => {
                        /* don't go up on the the first case */
                        if i > self.n {
                            Some(i - self.n)
                        } else {
                            None
                        }
                    }
                }
            }
        }
        let mut i = 0;
        let mut count: u64 = 0;
        let mut m = Move { dir: Dir::Right, n };
        while (count as usize) < n * n - 1 {
            loop {
                i = m.next(i).unwrap();
                count += 1;
                pieces[i] = count;
                match m.next(i) {
                    None => {
                        break;
                    }
                    Some(i) => if pieces[i] != 0 {
                        break;
                    },
                }
            }
            m.change_dir();
        }
        Taquin { n, pieces }
    }
    pub fn iter(&self) -> ::std::slice::Iter<u64> {
        self.pieces.iter()
    }
    pub fn dim(&self) -> usize {
        self.n
    }
    /// calc nb move to put the zero at the center
    pub fn nb_move_zero(&self) -> u64 {
        let index_pieces: i64 = self.pieces.iter().position(|&x| x == 0).unwrap() as i64;
        let n: i64 = self.n as i64;
        (n / 2 - index_pieces % n).abs() as u64 + (index_pieces / n - n / 2).abs() as u64
    }
}

use std::fmt::Display;

impl Display for Taquin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.n;
        let s = self.pieces
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, &nb)| {
                if i % n == 0 {
                    format!("{}\n{}", acc, nb)
                } else {
                    format!("{} {}", acc, nb)
                }
            });

        write!(f, "({}\n {})", n, s)
    }
}

impl FromStr for Taquin {
    type Err = ParseTaquinError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // remove comments
        let mut lines = s.lines()
            .map(|l| {
                l.get(..l.find('#').unwrap_or_else(|| l.len()))
                    .unwrap()
                    .trim() // can't fail
            })
            .filter(|l| l != &"");

        // get dimension
        let n: usize = lines.next().ok_or(ParseTaquinError::Empty)?.trim().parse()?;
        if n == 0 {
            return Err(ParseTaquinError::Empty);
        }

        // collect pieces
        let pieces = lines
            .map(|l| l.split_whitespace().map(|num| num.trim().parse()).collect())
            .collect::<Result<Vec<Vec<u64>>, ParseIntError>>()?;

        if pieces.len() != n {
            return Err(ParseTaquinError::BadNbLine);
        }

        if !pieces.iter().all(|v| v.len() == n) {
            return Err(ParseTaquinError::BadNbColonne);
        }

        let pieces = pieces.into_iter().flat_map(|l| l).collect::<Vec<u64>>();

        for i in 0..n * n {
            if !pieces.iter().any(|&k| k == i as u64) {
                return Err(ParseTaquinError::MissingNb(i as u64));
            }
        }

        Ok(Taquin { n, pieces })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseTaquinError {
    Empty,
    BadNbColonne,
    BadNbLine,
    BadNoTakin(ParseIntError),
    MissingNb(u64),
}

impl fmt::Display for ParseTaquinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseTaquinError::Empty => write!(f, "the taquin is empty"),
            ParseTaquinError::BadNbColonne => write!(f, "bad number of colonne"),
            ParseTaquinError::BadNbLine => write!(f, "bad number of line"),
            ParseTaquinError::BadNoTakin(ref e) => write!(f, "{}", e.description()),
            ParseTaquinError::MissingNb(n) => write!(f, "missing nb: {}", n),
        }
    }
}

impl Error for ParseTaquinError {
    fn description(&self) -> &str {
        match *self {
            ParseTaquinError::Empty => "the taquin is empty",
            ParseTaquinError::BadNbColonne => "bad number of colonne",
            ParseTaquinError::BadNbLine => "bad number of line",
            ParseTaquinError::BadNoTakin(ref e) => e.description(),
            ParseTaquinError::MissingNb(_) => "missing nb",
        }
    }
}

impl From<ParseIntError> for ParseTaquinError {
    fn from(error: ParseIntError) -> Self {
        ParseTaquinError::BadNoTakin(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty() {
        let s = "# This puzzle is solvable


            ";
        assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::Empty));
    }
    #[test]
    fn zero_taquin() {
        let s = "# This puzzle is solvable
                0


            ";
        assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::Empty));
    }
    #[test]
    fn comments() {
        let s = "# This puzzle is solvable
            3 #lol
            5 1 0 #1 2 3
            8 4 6            #
            3 7 2";
        assert_eq!(
            s.parse::<Taquin>().unwrap(),
            Taquin {
                n: 3,
                pieces: vec![5, 1, 0, 8, 4, 6, 3, 7, 2],
            }
        );
    }
    #[test]
    fn tabulations() {
        let s = "# This puzzle is solvable
            3#
            5 1 0 #1 2 3
            8 4 6            #
            3 7 2";
        assert_eq!(
            s.parse::<Taquin>().unwrap(),
            Taquin {
                n: 3,
                pieces: vec![5, 1, 0, 8, 4, 6, 3, 7, 2],
            }
        );
    }
    #[test]
    fn bad_integer() {
        let s = " 3
            a 1 0
            8 4 6
            3 7 2";
        assert!(s.parse::<Taquin>().is_err());
        //assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::BadNoTakin(_)));
    }
    #[test]
    fn bad_nb_colonne() {
        let s = "3
            1 1 0 1
            8 4 6
            3 7 2";
        assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::BadNbColonne));
    }
    #[test]
    fn bad_nb_line() {
        let s = "3
            1 1 0
            8 4 6
            8 4 6
            3 7 2";
        assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::BadNbLine));
    }
    #[test]
    fn missing_nb() {
        let s = "3
            1 5 0
            9 4 6
            3 7 2";
        assert_eq!(s.parse::<Taquin>(), Err(ParseTaquinError::MissingNb(8)));
    }
    #[test]
    fn spiral() {
        assert_eq!(
            Taquin::spiral(1),
            Taquin {
                n: 1,
                pieces: vec![0],
            }
        );
        assert_eq!(
            Taquin::spiral(2),
            Taquin {
                n: 2,
                pieces: vec![0, 1, 3, 2],
            }
        );
        assert_eq!(
            Taquin::spiral(3),
            Taquin {
                n: 3,
                pieces: vec![0, 1, 2, 7, 8, 3, 6, 5, 4],
            }
        );
        assert_eq!(
            Taquin::spiral(4),
            Taquin {
                n: 4,
                pieces: vec![0, 1, 2, 3, 11, 12, 13, 4, 10, 15, 14, 5, 9, 8, 7, 6],
            }
        );
        assert_eq!(
            Taquin::spiral(5),
            Taquin {
                n: 5,
                pieces: vec![
                    0, 1, 2, 3, 4, 15, 16, 17, 18, 5, 14, 23, 24, 19, 6, 13, 22, 21, 20, 7, 12, 11,
                    10, 9, 8,
                ],
            }
        );
    }
}
