use std::hash::{Hash, Hasher};
use num_traits::Zero;
use std::fmt::Debug;
use std::cmp::{PartialOrd, Ord, Ordering};
use std::collections::hash_map::DefaultHasher;
use maxHeap::BinaryHeap;
use std::fmt;
use std::fmt::Display;
use std::collections::HashSet;
use std::f32::INFINITY;

struct State<N, Action, C>
	where N: Clone,
		  Action: Copy,
		  C: Zero + Ord + Copy + Debug,
{

    /// Cost of current State
    pub gcost: C,

    /// Heuristical search cost
    pub hcost: C,
    /// Actual Taquin state
    pub taquin: N,

    /// Dir of predecessor
    pub predecessor: Option<Action>,

    /// Key of the State
    hash: u64,
}

impl<N: Clone + Hash, Action: Copy, C: Zero + Ord + Copy + Debug> State<N, Action, C> {
    pub fn new(predecessor: Option<Action>, gcost: C, taquin: N) -> State<N, Action, C> {
        let mut hash = DefaultHasher::new();
        taquin.hash(&mut hash);
        State {
            gcost,
            hcost: C::zero(),
            taquin,
            predecessor,
            hash: hash.finish(), // rewrite this
        }
    }	
}

impl<N: Clone + Hash, Action: Copy, C: Zero + Ord + Copy + Debug> Hash for State<N, Action, C> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.taquin.hash(state);
    }
}

impl<N: Clone + Eq, Action: Copy, C: Zero + Ord + Copy + Debug> Eq for State<N, Action, C> {} // derive ?

impl<N: Clone + Eq, Action: Copy, C: Zero + Ord + Copy + Debug> PartialEq for State<N, Action, C> {
    fn eq(&self, other: &Self) -> bool {
        self.taquin.eq(&other.taquin)
    }
    /*
    fn ne(&self, other: &Self) -> bool {
		self.taquin.ne(&other.taquin)
    }
    */
}

impl<N: Clone + Eq, Action: Copy, C: Zero + Ord + Copy + Debug> PartialOrd for State<N, Action, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: Clone + Eq, Action: Copy, C: Zero + Ord + Copy + Debug> Ord for State<N, Action, C> {
    /// Implementation of cmp in reverse order since we want a min_heap
    // Be careful here
    fn cmp(&self, other: &Self) -> Ordering {
        if self.gcost + self.hcost <= other.gcost + other.hcost {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

pub fn astar<N, C, FN, IN, FH, FS, FA, A>(
    start: &N,
    neighbours_actions: FN,
    perform_action: FA,
    heuristic: FH,
    success: FS,
) -> Option<(Vec<N>, C)>
	where
    N: Clone + Hash + Eq + Debug,
    C: Zero + Ord + Copy + Debug,
FN: Fn(&N) -> IN,
IN: IntoIterator<Item = (A, C)>,
FH: Fn(&N) -> C,
FS: Fn(&N) -> bool,
FA: Fn(&N, A) -> N,
A: Copy,
{
	const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
	const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;

   	let mut open_set: BinaryHeap<State<N, A, C>> = BinaryHeap::with_capacity(DEFAULT_OPEN_SET_SIZE);
	let mut closed_set = HashSet::with_capacity(DEFAULT_CLOSED_SET_SIZE);
	open_set.push(State::new(None, C::zero(), start.clone()));

    while !open_set.is_empty() {
        if success(&open_set.peek().expect("Tried to peek none existing open state").taquin) {
            println!("solution found");
            // the solution is found
            break ;
        }

        let current_state = open_set.pop().expect("Tried to pop none existing open state");
		println!("while current_state: {:?}", current_state.taquin);

        for (action, cost) in neighbours_actions(&current_state.taquin) {			
			let mut state = State::new(current_state.predecessor
									   , current_state.gcost + cost
									   , perform_action(&current_state.taquin, action));
            state.hcost = heuristic(&state.taquin);

            //println!("neighbour: {}", state);
            if !closed_set.get(&state).is_some() && !open_set.iter().any(|iter_state| *iter_state == state) {
                open_set.push(state);
            } else {
                // get old state in the open or closed set
                let &State {
                    gcost, ..
                } = open_set.iter().find(|s| **s == state)
					.unwrap_or_else(|| closed_set.get(&state).unwrap());

                if gcost > state.gcost {
                    if open_set.iter().any(|iter_state| *iter_state == state) {
                        open_set.update_value(state);
                    }
                    else if closed_set.get(&state).is_some() {
                        closed_set.remove(&state);
                        open_set.push(state);
                    }
                }
            }

        }
        if !closed_set.insert(current_state) {
            panic!("can't be already in closed set ?");
        }
    }
	
	fn unwind_solution_path<'a, N, C, Action, FA> (closed_set: &'a mut HashSet<State<N, Action, C>>
											   , initial_state: State<N, Action, C>
											   , perform_action: FA) -> Vec<N>
		where
		N: Clone + Hash + Eq + Debug,
		C: Zero + Ord + Copy + Debug,
		Action: Copy,
		FA: Fn(&N, Action) -> N,
	{
		let mut path: Vec<N> = Vec::with_capacity(128 * 2);

		path.push(initial_state.taquin.clone());
		let mut current_state = initial_state;
		while let Some(pred) = current_state.predecessor {
			current_state = closed_set.take(&State::new(None, C::zero(), perform_action(&current_state.taquin, pred))).unwrap();
			println!("pushing current_state: {:?}", current_state.taquin);
			path.push(current_state.taquin.clone());
		}
		path
	}
	let state = open_set.pop().expect("Tried to peek none existing open state");
	let total_cost: C = state.gcost + state.hcost;
	Some((unwind_solution_path(&mut closed_set, state, perform_action), total_cost))
}
