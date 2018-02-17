use taquin::Taquin;
use	state::State;
use	std::collections::{HashMap, BinaryHeap};

pub struct Solver {
    taquin: Taquin,
    spiral: Taquin,
	heuristic: Box<FnOnce(&Taquin, &Taquin) -> f32>,
	open_set: BinaryHeap<State>,
	closed_set: HashMap<State>,
}

impl Solver {
	const DEFAULT_OPEN_SET_SIZE: usize = 65536;
	const DEFAULT_CLOSED_SET_SIZE: usize = 65536;
	
    pub fn new(taquin: Taquin, heuristic: Box<FnOnce(&Taquin, &Taquin) -> f32>) -> Self {
    	let mut open_set = BinaryHeap::with_capacity(Self::DEFAULT_OPEN_SET_SIZE);
		open.push(taquin.clone());//clone ?
        Solver {
            taquin,
            spiral: Taquin::spiral(n),
			heuristic,
			open_set,
			closed_set: HashMap::with_capacity(Self::DEFAULT_OPEN_SET_SIZE),
        }
    }
    pub fn is_solved(&self) -> bool {
        self.taquin
            .iter()
            .zip(self.spiral.iter())
            .all(|(x, y)| x == y)
    }
    fn nb_transposition(&self) -> u64 {
        let mut trans_count = 0;
        let mut pieces = self.taquin.pieces.clone();
        for (index_spiral, nb) in self.spiral.iter().enumerate() {
            let index_pieces = pieces.iter().position(|&x| x == *nb).unwrap();

            if index_spiral != index_pieces {
                trans_count+=1;
                pieces.swap(index_pieces, index_spiral);
            }
        }
        trans_count
    }
    pub fn is_solvable(&self) -> bool {
        let n = self.taquin.dim();
        let nb_trans = self.nb_transposition();
        let nb_move = self.taquin.nb_move_zero();

        println!("{}", nb_trans);
        println!("{}", nb_move);
        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }

	/// Returns weither or not the considered state in the open set
	fn is_in_open_set(&self, state: &State) -> bool {
		self.open_set.iter().find(|&iter_state| iter_state == state).is_some()
	}

	fn is_in_closed_set(&self, state: &State) -> bool {
		self.closed_set.entry(state).is_some()
	}
	
	/// A* algorithm
	pub fn astar(&mut self) {
		let mut success = false;
		while self.open_set.len() != 0 {
			let state = self.open_set.peek().expect("Tried to peek none existing open state");
			if state.is_solved() {
				success = true;
			} else {
				let actual_state = self.open_set.pop().expect("Tried to pop none existing open state");
				self.closed_set.insert(self.closed_set.hasher(state.taquin), state);
				for state in &actual_state.iter_mut() {
					if self.is_in_closed_set(state) {
						continue ;
					}
					
					if self.is_in_open_set() == false
					{
						state.set_predecessor(&actual_state);
						state.set_cost(actual_state.cost + 1)
						self.open_set.push(state);//clone ?
					}
					
					let try_cost = actual_state.cost + 1;
					if try_cost >= actual_state.cost  {
						continue ;
					}
					
					state.set_predecessor(&actual_state);
					state.set_cost(try_score);
					state.set_fcost(try_score +
									self.heuristic(&state, &self.spiral));
				}
			}
		}
		if success {
			self.unwind_solution_path();
		}
	}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solved() {
        let taquin = Taquin::spiral(42);
        let solver = Solver::new(taquin);
        assert!(solver.is_solved());
    }
    #[test]
    fn unsolved() {
        let taquin = Taquin::new(3, vec![5, 1, 0, 8, 4, 6, 3, 7, 2]);
        let solver = Solver::new(taquin);
        assert!(!solver.is_solved());
    }
    #[test]
    fn solvable() {
        let taquin = Taquin::new(3, vec![0,8,3,1,6,4,5,7,2]);
        let solver = Solver::new(taquin);
        assert!(solver.is_solvable());
    }
    #[test]
    fn unsolvable() {
        let taquin = Taquin::new(3, vec![1,7,8,2,0,5,3,4,6]);
        let solver = Solver::new(taquin);
        assert!(!solver.is_solvable());
    }
}
