use taquin::Taquin;
use	state::State;
use	std::collections::{HashSet, BinaryHeap};

pub struct Solver {
    spiral: Taquin,
	heuristic: Box<FnOnce(&State, &Taquin) -> f32>,
	open_set: BinaryHeap<State>,
	closed_set: HashSet<State>,
}

impl Solver {
	const DEFAULT_OPEN_SET_SIZE: usize = 65536;
	const DEFAULT_CLOSED_SET_SIZE: usize = 65536;
	
    pub fn new(taquin: Taquin, heuristic: Box<FnOnce(&State, &Taquin) -> f32>) -> Self {
    	let mut open_set = BinaryHeap::with_capacity(Self::DEFAULT_OPEN_SET_SIZE);
		open_set.push(State::new(None, taquin.clone()));//clone ?
		let spiral = Taquin::spiral(taquin.dim());
        Solver {
            spiral,
			heuristic,
			open_set,
			closed_set: HashSet::with_capacity(Self::DEFAULT_OPEN_SET_SIZE),
        }
    }

	/// Returns weither or not the current state of the taquin is solved
    pub fn is_solvable(&self) -> bool {
        let nb_trans = self.open_set.peek()
			.unwrap()
			.get_taquin()
			.nb_transposition(&self.spiral);
        let nb_move = self.open_set
			.peek()
			.unwrap()
			.get_taquin()
			.nb_move_zero();

        println!("{}", nb_trans);
        println!("{}", nb_move);
        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }

	/// Returns weither or not the considered state in is the open set
	fn is_in_open_set(&self, state: &State) -> bool {
		self.open_set.iter().find(|&iter_state| iter_state == state).is_some()
	}

	/// Returns weither or not the considered state is in the closed set
	fn is_in_closed_set(&mut self, state: &State) -> bool {
		self.closed_set.get(state).is_some()
	}
	
	/// A* algorithm
	pub fn astar(&mut self) {
		let mut success = false;
		while self.open_set.len() != 0 {
			if self.open_set.peek().expect("Tried to peek none existing open state").is_solved(&self.spiral) {
				success = true;
			} else {
				let current_state = self.open_set.pop().expect("Tried to pop none existing open state");
				self.closed_set.insert(current_state);
				for mut state in current_state.iter_on_possible_states() {
					if self.is_in_closed_set(&state) {
						continue ;
					}
					
					if self.is_in_open_set(&state) == false {
						self.open_set.push(state);//clone ?
					} else {
						self.open_set.
					}
					
					let try_cost = current_state.cost + 1.0;
					if try_cost >= state.cost {
						continue ;
					}
					
					state.set_cost(try_cost);
					state.set_fcost(try_cost +
						(self.heuristic)(&state, &self.spiral));
				}
			}
		}
		if success {
			self.unwind_solution_path();
		}
	}

	fn unwind_solution_path(&self) {
		unimplemented!()
	}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solvable() {
        let taquin = Taquin::new(3, vec![0,8,3,1,6,4,5,7,2]);
        let solver = Solver::new(taquin, Box::new(|ref state, ref spiral| { 1.0 }));
        assert!(solver.is_solvable());
    }
    #[test]
    fn unsolvable() {
        let taquin = Taquin::new(3, vec![1,7,8,2,0,5,3,4,6]);
        let solver = Solver::new(taquin, Box::new(|ref state, ref spiral| { 1.0 }));
        assert!(!solver.is_solvable());
    }
}
