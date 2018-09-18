use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Mutex;

#[derive(Copy, Hash, Clone, Debug, PartialEq, Eq)]
pub enum Dir {
    Right,
    Up,
    Down,
    Left,
}

impl Dir {
    pub fn oposite(self) -> Self {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
    pub fn other(self) -> [Dir; 3] {
        match self {
            Dir::Right => [Dir::Left, Dir::Up, Dir::Down],
            Dir::Down => [Dir::Left, Dir::Up, Dir::Right],
            Dir::Left => [Dir::Down, Dir::Up, Dir::Right],
            Dir::Up => [Dir::Left, Dir::Down, Dir::Right],
        }
    }
}

impl From<usize> for Dir {
    fn from(u: usize) -> Dir {
        match u {
            0 => Dir::Right,
            1 => Dir::Up,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => panic!("usize out of dir range"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Taquin {
    n: usize,
    pieces: Vec<u64>,
    cur_pos: usize,
}

impl Hash for Taquin {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.pieces.hash(state)
    }
}

impl Taquin {
    pub fn new(n: usize, pieces: Vec<u64>) -> Self {
        debug_assert!((0..n).all(|i| pieces.iter().any(|&k| k == i as u64)));
        let cur_pos = pieces.iter().position(|&x| x == 0).unwrap();
        assert_eq!(pieces.len(), n * n);
        Taquin { n, pieces, cur_pos }
    }

    pub fn sorted_neighbours<'a>(&self, static_spiral: &Taquin) -> Vec<Dir> {
        let mut v = Vec::with_capacity(4);
        for dir in [Dir::Right, Dir::Down, Dir::Left, Dir::Up].into_iter() {
            if let Some(t) = self.move_piece(*dir) {
                v.push((t, *dir));
            }
        }
        v.sort_by_key(|(k, dir)| k.manhattan_heuristic_linear_conflict(static_spiral)); // OK I don't understand why this. yeah I should not have commented it, whatever really
        v.into_iter().map(|(t, dir)| dir).collect() // tomcuh cloning
        //Neighbours::new(self.clone())
    }

    pub fn neighbours<'a>(&self) -> Vec<Dir> {
        let mut v = Vec::with_capacity(4);
        for dir in [Dir::Right, Dir::Up, Dir::Down, Dir::Left].into_iter() {
            if let Some(t) = self.move_piece(*dir) {
                v.push((t, *dir));
            }
        }
        v.into_iter().map(|(t, dir)| dir).collect()
        //Neighbours::new(self.clone())
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
                if i == 0 {
                    None
                } else if (i - 1) % n != n - 1 {
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
        let mut dir_cycle = [Dir::Right, Dir::Down, Dir::Left, Dir::Up].iter().cycle();
        let mut dir = dir_cycle.next().unwrap();
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
            dir = dir_cycle.next().unwrap();
        }
        Self::new(n, pieces)
    }

    pub fn iter(&self) -> ::std::slice::Iter<u64> {
        self.pieces.iter()
    }

    /// Get current dimension of the taquin
    pub fn dim(&self) -> usize {
        self.n
    }

    /// calc nb move to put the zero at the center
    pub fn nb_move_zero(&self) -> u64 {
        let index_pieces: i64 = self.pieces.iter().position(|&x| x == 0).unwrap() as i64;
        let n: i64 = self.n as i64;
        (n / 2 - index_pieces % n).abs() as u64 + ((n - 1) / 2 - index_pieces / n).abs() as u64
    }

    pub fn nb_transposition(&self, static_spiral: &Taquin) -> u64 {
        let mut trans_count = 0;
        let mut pieces = self.pieces.clone();
        for (index_spiral, nb) in static_spiral.iter().enumerate() {
            let index_pieces = pieces.iter().position(|&x| x == *nb).unwrap();

            if index_spiral != index_pieces {
                trans_count += 1;
                pieces.swap(index_pieces, index_spiral);
            }
        }
        trans_count
    }

	/// Get the goal index for the tile at `index`
	pub fn get_goal_index(&self, index: u64, goal_ref: &Taquin) -> Option<usize> {
		let nbr = self.pieces[index as usize];
		goal_ref.iter().position(|&x| x == nbr)
	}

    /// calculate the manhattan distance between two position represended as the
    /// index of the piece
    fn manhattan_distance(index_1: i64, index_2: i64, n: i64) -> u64 {
        (index_1 % n - index_2 % n).abs() as u64 + (index_1 / n - index_2 / n).abs() as u64
    }

    pub fn manhattan_heuristic(&self, static_spiral: &Taquin) -> u64 {
        let mut dist = 0;
        for (index_spiral, nb) in static_spiral.iter().enumerate() {
            let index_pieces = self.pieces.iter().position(|&x| x == *nb).unwrap();
            if index_spiral != index_pieces {
                dist += Self::manhattan_distance(
                    index_pieces as i64,
                    index_spiral as i64,
                    self.n as i64,
                );
            }
        }
        dist as u64
    }

	pub fn hamming_distance_heuristic(&self, static_spiral: &Taquin) -> u64 {
		let mut dist = 0u64;

		for (spiral_piece, piece) in static_spiral.iter().zip(self.iter()) {
			if piece != spiral_piece {
				dist += 1;
			}
		}
		dist
	}

	fn is_piece_in_row_goal(&self, current_index: u64, goal_index: u64) -> bool {
		current_index / (self.n as u64) == goal_index / (self.n as u64)
	}

	fn are_pieces_aligned(&self, index_1: u64, index_2: u64) -> bool {
		(index_1 / (self.n as u64) == index_2 / (self.n as u64))
			|| (index_1 % (self.n as u64) == index_2 % (self.n as u64))
	}

	fn is_piece_in_column_goal(&self, current_index: u64, goal_index: u64) -> bool {
		current_index % (self.n as u64) == goal_index % (self.n as u64)
	}

	fn is_piece_partially_at_goal(&self, current_index: u64, goal_index: u64) -> bool {
		self.is_piece_in_row_goal(current_index, goal_index)
			^ self.is_piece_in_column_goal(current_index, goal_index)
	}

	fn linear_conflict(&self, current_index: u64, goal_index: u64, goal_ref: &Taquin) -> u64 {
		let mut total_conflicts = 0u64;

		assert_eq!(goal_index, self.get_goal_index(current_index, goal_ref).unwrap() as u64);
		if self.is_piece_partially_at_goal(current_index as u64, goal_index as u64) {
				for (search_index, tile_nbr) in self.iter().enumerate().filter(|(index, &x)| *index as u64 != current_index && x != 0) {
					let search_goal_index = self.get_goal_index(search_index as u64, goal_ref).expect("Send some bad index in get_goal_index");
					assert!(self.pieces[search_index as usize] != 0);
					if self.is_piece_partially_at_goal(search_index as u64, search_goal_index as u64)
						&& self.are_pieces_aligned(current_index as u64, search_goal_index as u64)
						&& self.are_pieces_aligned(current_index as u64, search_index as u64)
						&& self.are_pieces_aligned(search_index as u64, goal_index as u64)
						&& ((current_index < search_index as u64 && search_index as u64 <= goal_index
							|| current_index > search_index as u64 && search_index as u64 >= goal_index)
						|| ((search_index as u64) < current_index && current_index as u64 <= search_goal_index as u64
							|| (search_index as u64) > current_index && current_index as u64 >= search_goal_index as u64)) {
							total_conflicts += 1;
							// println!("There is a linear conflict between value: {} and value: {}\nTaquin: {}, spiral: {}"
							// 		 , self.pieces[current_index as usize], self.pieces[search_index as usize], self, goal_ref);
						}
				}
		}
		if total_conflicts != 0 {
//			println!("There was {} conflicts with current_index: {}", total_conflicts, current_index);
			;
		}
		total_conflicts
	}

	pub fn manhattan_heuristic_linear_conflict(&self, static_spiral: &Taquin) -> u64 {
		let mut dist = 0;
		let mut tmp_dist = 0;
		let mut lcn = 0;

		for (index_spiral, nb) in static_spiral.iter().enumerate().filter(|(_, &x)| x != 0) {
			let index_pieces = self.pieces.iter().position(|&x| x == *nb).unwrap();
			if index_spiral != index_pieces {
				let tmp = Self::manhattan_distance(
					index_pieces as i64,
					index_spiral as i64,
					self.n as i64,
				);
				dist += tmp;
				tmp_dist += tmp;
				let linear_conflicts = self.linear_conflict(index_pieces as u64, index_spiral as u64, &static_spiral);
				lcn += linear_conflicts;
				dist += linear_conflicts;
			}
		}
//		println!("heuristic cost would have {} instead is {} with {} lcn", tmp_dist, dist, lcn);
		dist as u64
	}

    /// Returns weither or not the state of the taquin is solvable
    pub fn is_solvable(&self, static_spiral: &Taquin) -> bool {
        let nb_trans = self.nb_transposition(static_spiral);
        let nb_move = self.nb_move_zero();

        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }

    pub fn is_solved(&self, spiral: &Taquin) -> bool {
        self.pieces
            .iter()
            .zip(spiral.iter())
            .all(|(x, y)| x == y)
    }
}

use std::fmt::Display;

impl Display for Taquin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.n;
        let s = self
            .pieces
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
        let mut lines = s
            .lines()
            .map(|l| {
                l.get(..l.find('#').unwrap_or_else(|| l.len()))
                    .unwrap()
                    .trim() // can't fail
            }).filter(|l| l != &"");

        // get dimension
        let n: usize = lines
            .next()
            .ok_or(ParseTaquinError::Empty)?
            .trim()
            .parse()?;
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

//pub struct Neighbours<'a> {
//    taquin: Taquin,
//    dir: Iter<'a, Dir>,
//}
//
//impl<'a> Neighbours<'a> {
//    pub fn new(taquin: Taquin) -> Self {
//
//        Neighbours {
//            taquin,
//            dir: [Dir::Right, Dir::Down, Dir::Left, Dir::Up].into_iter(),
//        }
//    }
//}
//)
//impl<'a> Iterator for Neighbours<'a> {
//    type Item = (Taquin, u32);
//    fn next(&mut self) -> Option<(Taquin, u32)> {
//       let (taquin_next, dir) = loop {
//            let dir = *self.dir.next()?;
//            if let Some(t) = self.taquin.move_piece(dir) {
//                break (t, dir);
//            }
//       };
//       // to get the predecessor go to the oposite direction
//       Some((taquin_next, 1))
//    }
//}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static;
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
            Taquin::new(3, vec![5, 1, 0, 8, 4, 6, 3, 7, 2],)
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
            Taquin::new(3, vec![5, 1, 0, 8, 4, 6, 3, 7, 2],)
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
        assert_eq!(Taquin::spiral(1), Taquin::new(1, vec![0],));
        assert_eq!(Taquin::spiral(2), Taquin::new(2, vec![1, 2, 0, 3],));
        assert_eq!(
            Taquin::spiral(3),
            Taquin::new(3, vec![1, 2, 3, 8, 0, 4, 7, 6, 5],)
        );
        assert_eq!(
            Taquin::spiral(4),
            Taquin::new(
                4,
                vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7],
            )
        );
        assert_eq!(
            Taquin::spiral(5),
            Taquin::new(
                5,
                vec![
                    1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12,
                    11, 10, 9
                ],
            )
        );
    }
    #[test]
    fn move_piece() {
        let s = "3
            1 5 0
            8 4 6
            3 7 2";
        let s_after = "3
            1 5 6
            8 4 0
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        let t_after = s_after.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Down).unwrap(), t_after);
        assert_eq!(
            t.move_piece(Dir::Down)
                .unwrap()
                .move_piece(Dir::Up)
                .unwrap(),
            t
        );

        let s = "3
            1 5 2
            8 4 6
            3 7 0";
        let t = s.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Down), None);

        let s = "3
            1 5 6
            8 4 0
            3 7 2";
        let s_after = "3
            1 5 0
            8 4 6
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        let t_after = s_after.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Up).unwrap(), t_after);
        let s = "3
            1 5 0
            8 4 6
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Up), None);

        let s = "3
            1 5 0
            8 4 6
            3 7 2";
        let s_after = "3
            1 0 5
            8 4 6
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        let t_after = s_after.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Left).unwrap(), t_after);
        assert_eq!(
            t.move_piece(Dir::Left)
                .unwrap()
                .move_piece(Dir::Right)
                .unwrap(),
            t
        );
        let s = "3
            1 5 2
            8 4 6
            0 7 3";
        let t = s.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Left), None);

        let s = "3
            1 0 5
            8 4 6
            3 7 2";
        let s_after = "3
            1 5 0
            8 4 6
            3 7 2";
        let t = s.parse::<Taquin>().unwrap();
        let t_after = s_after.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Right).unwrap(), t_after);
        let s = "3
            1 5 2
            8 4 6
            3 7 0";
        let t = s.parse::<Taquin>().unwrap();
        assert_eq!(t.move_piece(Dir::Right), None);
    }
    #[test]
    fn solved() {
        let taquin = Taquin::spiral(42);
        let mut s = Taquin::spiral(taquin.dim());
        assert!(taquin.is_solved(&s));
    }
    #[test]
    fn unsolved() {
        let taquin = Taquin::new(3, vec![5, 1, 0, 8, 4, 6, 3, 7, 2]);
        let mut s = Taquin::spiral(taquin.dim());
        assert!(!taquin.is_solved(&s));
    }
    #[test]
    fn oposite() {
        assert_eq!(Dir::Right.oposite(), Dir::Left);
        assert_eq!(Dir::Left.oposite(), Dir::Right);
        assert_eq!(Dir::Up.oposite(), Dir::Down);
        assert_eq!(Dir::Down.oposite(), Dir::Up);
    }
    #[test]
    #[should_panic]
    fn new_taquin() {
        let taquin = Taquin::new(3, vec![5, 10, 0, 8, 4, 6, 3, 7, 2]);
    }
}
