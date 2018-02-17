use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Copy, Clone)]
pub enum Dir {
    Right,
    Up,
    Down,
    Left,
}

impl Iterator for Dir {
    type Item = Dir;
    fn next(&mut self) -> Option<Self> {
       *self = match *self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => {return None},
       };
       Some(*self)
    }
}

impl Dir {
}

#[derive(Debug, PartialEq)]
pub struct Taquin {
    n: usize,
    pieces: Vec<u64>,
    cur_pos: usize,
}

impl Taquin {
    pub fn new(n: usize, pieces: Vec<u64>) -> Self {
        //TODO: remove for opti
        for i in 0..n * n {
            if !pieces.iter().any(|&k| k == i as u64) {
                panic!("missing nb in pieces");
            }
        }
        let cur_pos = pieces.iter().position(|&x| x == 0).unwrap();
        assert_eq!(pieces.len(), n * n);
        Taquin {
            n,
            pieces,
            cur_pos,
        }
    }
    /// get indice of piece next 'i' in direction 'dir'.
    fn get_index(dir: &Dir, i: usize, n: usize) -> Option<usize> {
        match *dir {
            Dir::Right => {
                if (i + 1) % n != 0 {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Dir::Down => {
                if i + n < n * n {
                    Some(i + n)
                } else {
                    None
                }
            }
            Dir::Left => {
                if (i - 1) % n != n - 1 {
                    Some(i - 1)
                } else {
                    None
                }
            }
            Dir::Up => {
                if i >= n {
                    Some(i - n)
                } else {
                    None
                }
            }
        }
    }
    pub fn move_piece(&self, dir: Dir) -> Option<Self> {
        let index_to_go = Taquin::get_index(&dir, self.cur_pos, self.n)?;
        let mut new_pieces = self.pieces.clone();
        new_pieces.swap(self.cur_pos, index_to_go);
        Some(Taquin {
            n: self.n,
            pieces: new_pieces,
            cur_pos: index_to_go,
        })
    }
    pub fn spiral(n: usize) -> Self {
        let mut pieces: Vec<u64> = vec![0; n * n];
        let mut i = 0;
        let mut count: u64 = 1;
        let mut dir = Dir::Right;
        while (count as usize) < n * n {
            loop {
                pieces[i] = count;
                i = Taquin::get_index(&dir, i, n).unwrap();
                count += 1;
                match Taquin::get_index(&dir, i, n) {
                    None => {
                        break;
                    }
                    Some(i) => if pieces[i] != 0 || i == 0 {
                        break;
                    },
                }
            }
            dir = match dir.next() {
                None => Dir::Right,
                Some(d) => d,
            }
        }
        Self::new(n, pieces)
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
        (n / 2 - index_pieces % n).abs() as u64 + ((n - 1)/ 2 - index_pieces / n).abs() as u64
    }
    pub fn nb_transposition(&self, spiral: &Taquin) -> u64 {
        let mut trans_count = 0;
        let mut pieces = self.pieces.clone();
        for (index_spiral, nb) in spiral.iter().enumerate() {
            let index_pieces = pieces.iter().position(|&x| x == *nb).unwrap();

            if index_spiral != index_pieces {
                trans_count+=1;
                pieces.swap(index_pieces, index_spiral);
            }
        }
        trans_count
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

        Ok(Taquin::new(n, pieces))
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
            Taquin::new(
                3,
                vec![5, 1, 0, 8, 4, 6, 3, 7, 2],
                )
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
            Taquin::new(
                3,
                vec![5, 1, 0, 8, 4, 6, 3, 7, 2],
                )
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
            Taquin::new(
                1,
                vec![0],
                )
            );
        assert_eq!(
            Taquin::spiral(2),
            Taquin::new(
                2,
                vec![1,2,0,3],
                )
            );
        assert_eq!(
            Taquin::spiral(3),
            Taquin::new(
                3,
                vec![1,2,3,8,0,4,7,6,5],
                )
            );
        assert_eq!(
            Taquin::spiral(4),
            Taquin::new(
                4,
                vec![1,2,3,4,12,13,14,5,11,0,15,6,10,9,8,7],
                )
            );
        assert_eq!(
            Taquin::spiral(5),
            Taquin::new(
                5,
                vec![1,2,3,4,5,16,17,18,19,6,15,24,0,20,7,14,23,22,21,8,13,12,11,10,9],
                )
            );
    }
    #[test]
    fn move_piece() {
        let s = "3
            1 5 0
            8 4 6
            3 7 2";
        let s_down = "3
            1 5 6
            8 4 0
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        let t_down = s_down.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Down).unwrap(), t_down);
        let s = "3
            1 5 2
            8 4 6
            3 7 0";
        let t = s.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Down), None);
    }
}
