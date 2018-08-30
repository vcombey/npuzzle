use taquin::Taquin;
use taquin::Dir;
use	std::f32::INFINITY;
use	std::cmp::{Ord, Ordering, Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug)]
pub struct State {
	/// Cost of current State
	pub gcost: f32,

	/// Heuristical search cost
	pub hcost: f32,
	/// Actual Taquin state
	taquin: Taquin,

	/// Dir of predecessor
	pub predecessor: Option<Dir>,
	
	/// Key of the State
	hash: u64,
}

impl State {
	pub fn new(predecessor: Option<Dir>, gcost: f32, taquin: Taquin) -> State {
		let mut hash = DefaultHasher::new();
		taquin.hash(&mut hash);
		
		State {
			gcost,
			hcost: INFINITY,
			taquin,
			predecessor, 
			hash: hash.finish(), // rewrite this
		}
	}

	/// Returns weither or not the state of the taquin is solvable
    pub fn is_solvable(&self) -> bool {
        let nb_trans = self.taquin.nb_transposition();
        let nb_move = self.taquin.nb_move_zero();

        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }

	pub fn get_hash(&self) -> u64 {
		self.hash
	}
	
	/// Set state's cost to new_cost
	pub fn set_gcost(&mut self, new_gcost: f32) {
		self.gcost = new_gcost;
	}

	/// Set state's fcost to new_fcost
	pub fn set_hcost(&mut self, new_hcost: f32) {
		self.hcost = new_hcost;
	}

	/// Get the inner taquin of state
	pub fn get_taquin(&self) -> &Taquin {
		&self.taquin
	}

	pub fn is_solved(&self) -> bool {
		self.taquin.is_solved()
	}
	
	pub fn iter_on_possible_states<'a>(&'a self) -> Neighbours<'a> {
        Neighbours::new(&self)
	}
    pub fn move_piece(&self, dir: Dir) -> Option<Self> {
        Some(State::new(None, 0.0, self.taquin.move_piece(dir)?))
    }
}

impl Hash for State {
	fn hash<H>(&self, state: &mut H)
		where H: Hasher {
		self.taquin.hash(state);
	}
}

pub struct Neighbours<'a> {
    state: &'a State,
    dir: Iter<'a, Dir>,
}

impl<'a> Neighbours<'a> {
    pub fn new(state: &'a State) -> Self {
        Neighbours {
            state,
            dir: [Dir::Right, Dir::Down, Dir::Left, Dir::Up].into_iter(),
        }
    }
}

use std::slice::Iter;

impl<'a> Iterator for Neighbours<'a> {
    type Item = State;
    fn next(&mut self) -> Option<State> {
       let (taquin_next, dir) = loop {
            let dir = *self.dir.next()?;
            if let Some(t) = self.state.get_taquin().move_piece(dir) {
                break (t, dir);
            }
       };
       // to get the predecessor go to the oposite direction
       Some(State::new(Some(dir.oposite()), self.state.gcost + 1.0,  taquin_next))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    /// Implementation of cmp in reverse order since we want a min_heap
    // Be careful here
    fn cmp(&self, other: &Self) -> Ordering {
        if self.gcost + self.hcost <= other.gcost + other.hcost { Ordering::Greater } else { Ordering::Less }
    }
}

impl Eq for State {} // derive ? 

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
		self.taquin.eq(&other.taquin)
    }
/*
    fn ne(&self, other: &Self) -> bool {
		self.taquin.ne(&other.taquin)
    }
    */
}

use std::fmt::Display;
use std::fmt;

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n {})", self, self.taquin)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static;
    #[test]
    fn neighbours() {
        let t = "3
            1 5 4
            8 0 6
            3 7 2".parse::<Taquin>().unwrap();
        let state = State::new(None, 0.0, t.clone());
        let mut dir_iter = [Dir::Right, Dir::Down, Dir::Left, Dir::Up].iter();
        let mut dir;

        assert!(state.iter_on_possible_states().next().is_some());

        for neighbour in state.iter_on_possible_states() {
            dir = *dir_iter.next().unwrap();
            println!("{:?}", neighbour);
            assert_eq!(neighbour, State::new(None, 0.0, t.clone().move_piece(dir).unwrap()));
        }
    }
    #[test]
    fn solvable() {
        lazy_static::initialize(&::taquin::static_spiral);
        let taquin = Taquin::new(3, vec![0,8,3,1,6,4,5,7,2]);
        let mut s = ::taquin::static_spiral.lock().unwrap();
        (*s) = Taquin::spiral(taquin.dim()); 
        drop(s);
        let initial_state = State::new(None, 0.0, taquin.clone());
        assert!(initial_state.is_solvable());
    }
    #[test]
    fn unsolvable() {
        lazy_static::initialize(&::taquin::static_spiral);
        let taquin = Taquin::new(3, vec![1,7,8,2,0,5,3,4,6]);
        let mut s = ::taquin::static_spiral.lock().unwrap();
        (*s) = Taquin::spiral(taquin.dim()); 
        drop(s);
        let initial_state = State::new(None, 0.0, taquin.clone());
        assert!(!initial_state.is_solvable());
    }
}
