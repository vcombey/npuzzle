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
	const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
	const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;
	
    pub fn new(taquin: Taquin, heuristic: Box<FnOnce(&State, &Taquin) -> f32>) -> Self {
    	let mut open_set = BinaryHeap::with_capacity(Self::DEFAULT_OPEN_SET_SIZE);
		let spiral = Taquin::spiral(taquin.dim());
		open_set.push(State::new(None, taquin));
        Solver {
            spiral,
			heuristic,
			open_set,
			closed_set: HashSet::with_capacity(Self::DEFAULT_OPEN_SET_SIZE),
        }
    }

	/// Returns weither or not the inital state of the taquin is solvable
    pub fn is_solvable(&self) -> bool {
        let taquin = self.open_set.peek().unwrap().get_taquin();

        let nb_trans = taquin.nb_transposition(&self.spiral);
        let nb_move = taquin.nb_move_zero();

        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }

	/// Returns weither or not the considered state in is the open set
	fn is_in_open_set(&self, state: &State) -> bool {
		self.open_set.iter().any(|iter_state| iter_state == state)
	}

	/// Returns weither or not the considered state is in the closed set
	fn is_in_closed_set(&self, state: &State) -> bool {
		self.closed_set.get(state).is_some()
	}
	
    /// A* algorithm
    pub fn astar(&mut self) {
        while !self.open_set.is_empty() {
            if self.open_set.peek().expect("Tried to peek none existing open state").is_solved(&self.spiral) {
                // the solution is found
                break ;
            }

            let current_state = self.open_set.pop().expect("Tried to pop none existing open state");

            for mut state in current_state.iter_on_possible_states() {
                state.set_fcost((self.heuristic)(&state, &self.spiral));
                if !self.is_in_closed_set(&state) && !self.is_in_open_set(&state) {
                    self.open_set.push(state);
                }
                else {
                    // get old state in the open or closed set
                    let old_state = self.open_set.iter().find(|s| **s == state)
                        .unwrap_or_else(|| self.closed_set.get(&state).unwrap());
                    if old_state.cost > state.cost {
                        old_state.cost = state.cost;
                        if self.is_in_closed_set(&state) {
                            self.closed_set.remove(&state);
                            self.open_set.push(state);
                        }
                    }
                }
            }
            if self.closed_set.insert(current_state) {
                panic!("can't be already in closed set ?");
            }
        }
        self.unwind_solution_path();
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
