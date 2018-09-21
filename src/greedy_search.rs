use maxHeap::BinaryHeap;
use num_traits::Zero;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use complexity::Complexity;

pub fn greedy_search<N, C, FN, IN, FH, FS, FA, A>(
    start: &N,
    neighbours_actions: FN,
    perform_action: FA,
    heuristic: FH,
    success: FS,
) -> Option<(Vec<N>, Complexity)>
where
    N: Clone + Hash + Eq + Debug + Display,
    C: Zero + Ord + Copy + Debug,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (A, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool,
    FA: Fn(&N, A) -> N,
    A: Copy,
{
    const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;

    let mut closed_set = HashSet::with_capacity(DEFAULT_CLOSED_SET_SIZE);
    let mut complexity = Complexity { in_time : 0, in_size : 0};

	let mut current_state = start.clone();
	let mut paths = Vec::new();
	'blocked: while !success(&current_state) {
		complexity.in_time += 1;
		for (action, _) in neighbours_actions(&current_state) {
			let next_state = perform_action(&current_state, action);
			if closed_set.insert(next_state.clone()) == false {
				continue
			}
			paths.push(current_state);
			current_state = next_state;
			continue 'blocked
		}
		current_state = match paths.pop() {
			Some(state) => state,
			None => {
				eprintln!("YOU LIED TO ME, IT'S NOT SOLVABLE, YOU KNOW IT, I WILL FIND AND I WILL KILL YOU");
				::std::process::exit(1);
			}
		}
	}
	complexity.in_size = closed_set.len();
	paths.reverse();
    Some((paths, complexity))
}
