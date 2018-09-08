extern crate getopts;
extern crate npuzzle;
use getopts::Options;
use npuzzle::idastar::idastar;
use npuzzle::trie::*;
use npuzzle::{taquin, taquin::Taquin};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::repeat;
extern crate bincode;
use bincode::deserialize;

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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILENAME [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "", "serde file of automaton", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let taquin_file = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let automaton_file = matches.opt_str("a").unwrap();
    let automaton: Trie = deserialize(&fs::read(automaton_file).unwrap()[..]).unwrap();

    let s = match read_file(&taquin_file) {
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
    if !taquin.is_solvable() {
        println!("this is unsolvable");
        return;
    }
    let mut path = idastar(
        &taquin,
        |t| t.sorted_neighbours().into_iter().zip(repeat(1)),
        |t, a| t.move_piece(a).unwrap(),
        |t| t.manhattan_heuristic(),
        |t| t.is_solved(),
        TrieType::Match(0),
        |old_state, dir| automaton.change_true_state(old_state, dir),
        |t| *t == TrieType::Redundant,
    ).unwrap();

    path.0.reverse();
    for p in path.0 {
        println!("{}", p);
    }
}
