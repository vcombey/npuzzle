use taquin::Taquin;
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
	predecessor: Option<u64>,
	
	/// Key of the State
	key: u64,
}

struct StateIter<'a> {
	predecessor: &'a State,
	count: u8,
}

impl State {
	pub fn new(predecessor: Option<u64>, taquin: Taquin) -> State {
		let hash = DefaultHasher::new();
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
	
	pub fn is_solved(&self) -> bool {
		//		self.taquin.is_solved()
		unimplemented!()
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
