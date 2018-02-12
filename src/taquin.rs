use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Taquin {
    n: usize,
    pieces: Vec<u64>,
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
}
