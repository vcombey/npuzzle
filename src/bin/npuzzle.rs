extern crate getopts;
extern crate npuzzle;
use getopts::Options;
use npuzzle::astar::astar;
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
    opts.optopt("g", "alg", "Algorithm", "astar | idastar");
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
    let algorithm = matches.opt_str("g").unwrap();
    let automaton: Trie = deserialize(&fs::read(automaton_file).unwrap()[..]).unwrap();

    let s = match read_file(&taquin_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    // println!("{}", s);

    let taquin = s.parse::<Taquin>().unwrap();
    let spiral = Taquin::spiral(taquin.dim());
    let mut s = taquin::static_spiral.lock().unwrap();
    (*s) = Taquin::spiral(taquin.dim());
    drop(s);
    if !taquin.is_solvable() {
        println!("this is unsolvable");
        return;
    }
    let mut path = match algorithm.as_str() {
        "idastar" => idastar(
            &taquin,
            |t| t.sorted_neighbours().into_iter().zip(repeat(1)),
            |t, a| t.move_piece(a).unwrap(),
            |t| t.manhattan_heuristic_linear_conflict(),
            |t| t.is_solved(),
            TrieType::Match(0),
            |old_state, dir| automaton.change_true_state(old_state, dir),
            |t| *t == TrieType::Redundant,
        ).unwrap(),
        "astar" => astar(
            &taquin,
            |t| t.neighbours().into_iter().zip(repeat(1)),
            |t, a| t.move_piece(a).unwrap(),
            |t| //t.manhattan_heuristic_linear_conflict()
			t.manhattan_heuristic_linear_conflict()
				,
            |t| t.is_solved(),
        ).unwrap(),
        _ => {
            eprintln!("Unknown algorithm");
            print_usage(&program, opts);
            ::std::process::exit(1);
        }
    };

    path.0.reverse();
    for p in path.0 {
        println!("{}", p);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command;
    use std::str::from_utf8;
    const NB_TEST: usize = 5;
    #[test]
    fn eq_idastar_astar() {
        for _ in 0..NB_TEST {
            let output = Command::new("./npuzzle-gen.py")
                .arg("-s")
                .arg("3")
                .output()
                .expect("failed to execute process");
            let s = from_utf8(&output.stdout).unwrap();

            let automaton: Trie =
                deserialize(&fs::read("prunning_automaton_3x3_d10.serde").unwrap()[..]).unwrap();
            let taquin = s.parse::<Taquin>().unwrap();
            let spiral = Taquin::spiral(taquin.dim());
            let mut s = taquin::static_spiral.lock().unwrap();
            (*s) = Taquin::spiral(taquin.dim());
            drop(s);
            if !taquin.is_solvable() {
                println!("this is unsolvable");
                return;
            }

            assert_eq!(
                idastar(
                    &taquin,
                    |t| t.sorted_neighbours().into_iter().zip(repeat(1)),
                    |t, a| t.move_piece(a).unwrap(),
                    |t| t.manhattan_heuristic(),
                    |t| t.is_solved(),
                    TrieType::Match(0),
                    |old_state, dir| automaton.change_true_state(old_state, dir),
                    |t| *t == TrieType::Redundant,
                ).unwrap()
                .0
                .len(),
                astar(
                    &taquin,
                    |t| t.neighbours().into_iter().zip(repeat(1)),
                    |t, a| t.move_piece(a).unwrap(),
                    |t| t.manhattan_heuristic(),
                    |t| t.is_solved()
                ).unwrap()
                .0
                .len()
            );
        }
    }
}
