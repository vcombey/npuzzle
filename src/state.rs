use taquin::Taquin;
use taquin::Dir;
use	std::f32::INFINITY;
use	std::cmp::{Ord, Ordering, Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct State {
	/// Cost of current State
	pub cost: f32,

	/// Heuristical search cost
	pub fcost: f32,
	/// Actual Taquin state
	taquin: Taquin,

	/// Hash of predecessor
	predecessor: Option<Dir>,
	
	/// Key of the State
	key: u64,
}

struct StateIter<'a> {
	predecessor: &'a State,
	count: u8,
}

impl State {
	pub fn new(predecessor: Option<Dir>, taquin: Taquin) -> State {
		let mut hash = DefaultHasher::new();
		taquin.hash(&mut hash);
		
		State {
			cost: INFINITY,
			fcost: INFINITY,
			taquin,
			predecessor, 
			key: hash.finish(), // rewrite this
		}
	}

	pub fn get_key(&self) -> u64 {
		self.key
	}
	
	// pub fn create_states(&self) -> StateIter {

	// }

	/// Set state's cost to new_cost
	pub fn set_cost(&mut self, new_cost: f32) {
		self.cost = new_cost;
	}

	/// Set state's fcost to new_fcost
	pub fn set_fcost(&mut self, new_fcost: f32) {
		self.fcost = new_fcost;
	}

	/// Set the state's predecessor to `predecessor`
	pub fn set_predecessor(&mut self, predecessor: &State) {
		self.predecessor = Some(predecessor.key)
	}
	
	pub fn get_taquin(&self) -> &Taquin {
		&self.taquin
	}

	pub fn is_solved(&self, spiral: &Taquin) -> bool {
		self.taquin.is_solved(spiral)
	}
	
	pub fn iter_on_possible_states<'a>(&self) -> Neighbours<'a> {
        Neighbours::new(&self)
	}
}

struct Neighbours<'a> {
    state: &'a State,
    dir: Dir,
}

impl<'a> Neighbours<'a> {
    pub fn new(state: &'a State) -> Self {
        Neighbours {
            state,
            dir: Dir::new(),
        }
    }
}

impl<'a> Iterator for Neighbours<'a> {
    type Item = State;
    fn next(&mut self) -> Option<State> {
       let taquin_next = loop {
            self.dir.next()?;
            if let Some(t) = self.state.get_taquin().move_piece(self.dir) {
                break t;
            }
       };
       Some(State::new(Some(self.dir), taquin_next))
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
        match self.fcost <= other.fcost {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}

impl Eq for State {} // derive ? 

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.fcost == other.fcost
    }

    fn ne(&self, other: &Self) -> bool {
        self.fcost != other.fcost
    }
}
