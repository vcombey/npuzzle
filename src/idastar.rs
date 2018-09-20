use num_traits::Zero;
use std::fmt::Debug;
use complexity::Complexity;

enum Res<C> {
    Found,
    MinFCost(C),
}

use self::Res::*;

pub fn idastar<N, C, FN, IN, FH, FS, S, CS, IR, FA, A>(
    start: &N,
    neighbours_actions: FN,
    perform_action: FA,
    heuristic: FH,
    success: FS,
    init_state: S,
    change_state: CS,
    is_redundant: IR,
) -> Option<(Vec<N>, Complexity)>
where
    N: Clone,
    C: Zero + Ord + Copy + Debug,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (A, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool,
    S: Copy,
    CS: Fn(&S, A) -> S,
    IR: Fn(&S) -> bool,
    FA: Fn(&N, A) -> N,
    A: Copy,
{
    fn aux<N, C, FN, IN, FH, FS, S, CS, IR, FA, A>(
        start: N,
        neighbours_actions: &FN,
        perform_action: &FA,
        heuristic: &FH,
        success: &FS,
        path: &mut Vec<N>,
        g_cost: C,
        threshold: C,
        init_state: S,
        change_state: &CS,
        is_redundant: &IR,
        complexity: &mut Complexity,
        depth: usize,
    ) -> Res<C>
    where
        N: Clone,
        C: Zero + Ord + Copy + Debug,
        FN: Fn(&N) -> IN,
        IN: IntoIterator<Item = (A, C)>,
        FH: Fn(&N) -> C,
        FS: Fn(&N) -> bool,
        FA: Fn(&N, A) -> N,
        S: Copy,
        CS: Fn(&S, A) -> S,
        IR: Fn(&S) -> bool,
        A: Copy,
    {
        if success(&start) {
            path.push(start);
            return Found;
        }
        if depth > complexity.in_size {
            complexity.in_size = depth;
        }
        let mut min_fcost = C::zero();
        let f_cost = g_cost + heuristic(&start);
        if f_cost > threshold {
            return MinFCost(f_cost);
        }
        for (a, c) in neighbours_actions(&start) {
            let new_state = change_state(&init_state, a);
            if is_redundant(&new_state) {
                continue;
            }
            let n = perform_action(&start, a);
            complexity.in_time+=1; 
            match aux(
                n,
                neighbours_actions,
                perform_action,
                heuristic,
                success,
                path,
                g_cost + c,
                threshold,
                new_state,
                change_state,
                is_redundant,
                complexity,
                depth + 1,
            ) {
                Found => {
                    path.push(start);
                    return Found;
                }
                MinFCost(c) => {
                    if min_fcost == C::zero() || (c < min_fcost && c != C::zero()) {
                        min_fcost = c;
                    }
                }
            }
        }
        return MinFCost(min_fcost);
    }

    let mut complexity = Complexity { in_time : 0, in_size : 0};
    let mut threshold = heuristic(start);
    let mut path = Vec::new();
    while let MinFCost(new_threshold) = aux(
        start.clone(),
        &neighbours_actions,
        &perform_action,
        &heuristic,
        &success,
        &mut path,
        C::zero(),
        threshold,
        init_state,
        &change_state,
        &is_redundant,
        &mut complexity,
        1,
    ) {
        threshold = new_threshold;
    }
    //TODO: See final cost
    return Some((path, complexity));
}
