extern crate npuzzle;
use npuzzle::{taquin, taquin::Taquin};
//use npuzzle::solver::Solver;
use npuzzle::idastar::idastar;
//use	npuzzle::state::State;
use std::env;
use std::fs::File;
use std::io::Read;
//use	std::collections::HashSet;
use std::iter::repeat;
//use std::slice::reverse;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//fn unwind_solution_path(closed_set: &HashSet<State>, state: &State) {
//    match state.predecessor {
//        None => {return;},
//        Some(p) => {
//            unwind_solution_path(closed_set, closed_set.get(&(state.move_piece(p).unwrap())).unwrap());
//            println!("{}", state.get_taquin());
//        }
//    }
//}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("bad number of args, expected one");
        return;
    }
    let s = match read_file(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    println!("{}", s);

    let taquin = s.parse::<Taquin>().unwrap();
    let spiral = Taquin::spiral(taquin.dim());
    let mut s = taquin::static_spiral.lock().unwrap();
    (*s) = Taquin::spiral(taquin.dim());
    drop(s);
    //println!("{:?}", taquin);
    // let initial_state = State::new(None, 0.0, taquin.clone());
    if !taquin.is_solvable() {
        println!("this is unsolvable");
        return;
    }
    //let mut solver = Solver::new(initial_state);
    //solver.with_heuristic(Solver::manhattan_heuristic);
    //solver.astar();
    //unwind_solution_path(&solver.closed_set, &solver.open_set.peek().unwrap());
    let mut path = idastar(
        &taquin,
        |t| t.sorted_neighbours().zip(repeat(1)),
        |t| t.manhattan_heuristic(),
        |t| t.is_solved(),
    ).unwrap();

    path.0.reverse();
    for p in path.0 {
        println!("{}", p);
    }
}
