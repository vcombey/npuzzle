use num_traits::Zero;
use std::hash::Hash;

enum Res<C> {
    Found,
    MinFCost(C),
}

use self::Res::*;

fn aux<N, C, FN, IN, FH, FS>(
    start: N, 
    neighbours: &FN, 
    heuristic: &FH, 
    success: &FS,
    path: &mut Vec<N>,
    g_cost: C,
    threeshold: C,
) -> Res<C> where
    N: Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool, 
{
    if success(&start) {
        path.push(start);
        return Found;
    }
    let mut min_fcost = C::zero();
    if min_fcost > threeshold {
        return MinFCost(min_fcost);
    }
    for (n, c) in neighbours(&start) {
        match aux(n, neighbours, heuristic, success, path, g_cost + c, threeshold) {
            Found => {
                path.push(start);
                return Found;
            }
            MinFCost(c) => {
                if min_fcost == C::zero() || c < min_fcost {
                    min_fcost = c;
                }
            }
        }
    }
    return MinFCost(min_fcost);
}

pub fn idastar<N, C, FN, IN, FH, FS>(
    start: &N, 
    neighbours: FN, 
    heuristic: FH, 
    success: FS
) -> Option<(Vec<N>, C)> where
    N: Clone,
    C: Zero + Ord + Copy,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: Fn(&N) -> C,
    FS: Fn(&N) -> bool, 
{
    let mut threeshold = heuristic(start);
    let mut path = Vec::new();
    while let MinFCost(new_threeshold) = aux(start.clone(), &neighbours, &heuristic, &success, &mut path, C::zero(), threeshold)
    {
        threeshold = new_threeshold;
    }
    //TODO: See final cost
    return Some((path, threeshold));
}
