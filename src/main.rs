use std::fs::File;
use std::io::Read;
use std::env;
use std::num::ParseIntError;
use std::str::FromStr;
#[macro_use] extern crate failure;

use failure::Error;

#[derive(Debug, Fail, PartialEq)]
enum ParseNpuzzleError {
    #[fail(display = "invalid dimension")]
    BadDim,
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug, PartialEq)]
struct Npuzzle {
    n: u64, 
    pieces: Vec<u64>,
}

impl FromStr for Npuzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // remove comments
        let mut lines = s.lines().map(|l| {
            l.get(..l.find('#').unwrap_or(l.len()))
                .unwrap().trim() // can't fail
        }).filter(|l| l != &"");

        // get dimension
        let n: u64 = lines.next().ok_or(ParseNpuzzleError::BadDim)?.trim().parse()?;

        // collect pieces
        let pieces = lines.flat_map(|l| {
             l.split_whitespace().map(|num| num.trim().parse())
        }).collect::<Result<Vec<u64>, ParseIntError>>()?;

        Ok(Npuzzle {
            n,
            pieces,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn comments() {
        let s = "# This puzzle is solvable
3 #lol
5 1 0 #1 2 3
8 4 6            #
3 7 2";
        assert_eq!(s.parse::<Npuzzle>().unwrap(), Npuzzle { n: 3, pieces: vec![5, 1, 0, 8, 4, 6, 3, 7, 2] });
    }
    #[test]
    fn tabulations() {
        let s = "# This puzzle is solvable
3#
 5 1 0 #1 2 3
8 4 6            #
3 7 2";
        assert_eq!(s.parse::<Npuzzle>().unwrap(), Npuzzle { n: 3, pieces: vec![5, 1, 0, 8, 4, 6, 3, 7, 2] });
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { 
        eprintln!("bad number of args, expected one");
        return ;
    }
    let s = match read_file(&args[1]) {
        Ok(s) => s,
        Err(e) => {eprintln!("{}", e); return ;},
    };
    println!("{}", s);
    let npuzzle = s.parse::<Npuzzle>().unwrap();
    println!("{:?}", npuzzle);
}
