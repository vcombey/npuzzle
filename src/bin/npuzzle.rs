extern crate getopts;
extern crate npuzzle;
extern crate sdl2;
#[macro_use]
extern crate itertools;

use getopts::Options;
use npuzzle::astar::astar;
use npuzzle::greedy_search::greedy;
use npuzzle::idastar::idastar;
use npuzzle::trie::*;
use npuzzle::{taquin, taquin::Taquin};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::repeat;
use std::str::FromStr;
extern crate bincode;
use bincode::deserialize;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILENAME [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "", "serde file of automaton", "PATH");
    opts.optopt("g", "alg", "Algorithm", "(astar | idastar)");
    opts.optopt(
        "q",
        "heurisique",
        "Heuristique",
        "(manhattan | linear_conflict)",
    );
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

    let algorithm = match matches.opt_str("g") {
        Some(a) => a,
        None => String::from_str("astar").unwrap(),
    };

    let heuristique: fn(&Taquin, &Taquin) -> u64 = match matches.opt_str("q") {
        Some(s) => match s.as_str() {
            "manhattan" => |t: &Taquin, s: &Taquin| t.manhattan_heuristic(s),
            "linear_conflict" => |t: &Taquin, s: &Taquin| t.manhattan_heuristic_linear_conflict(s),
            _ => {
                eprintln!("Unknown algorithm");
                print_usage(&program, opts);
                ::std::process::exit(1);
            }
        },
        None => |t: &Taquin, s: &Taquin| t.manhattan_heuristic(s),
    };

    let s = match read_file(&taquin_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::exit(1);
            return;
        }
    };
    // println!("{}", s);

    let taquin = s.parse::<Taquin>().unwrap();
    let spiral = Taquin::spiral(taquin.dim());

    if !taquin.is_solvable(&spiral) {
        println!("this is unsolvable");
        return;
    }
    let mut path = match algorithm.as_str() {
        "idastar" => {
            let automaton_file = matches.opt_str("a").unwrap();
            let automaton: Trie = deserialize(&fs::read(automaton_file).unwrap()[..]).unwrap();
            idastar(
                &taquin,
                |t| t.neighbours().into_iter().zip(repeat(1)),
                |t, a| t.move_piece(a).unwrap(),
                |t| heuristique(t, &spiral),
                |t| t.is_solved(&spiral),
                TrieType::Match(0),
                |old_state, dir| automaton.change_true_state(old_state, dir),
                |t| *t == TrieType::Redundant,
            ).unwrap()
        }
        "greedy" => {
            let automaton_file = matches.opt_str("a").unwrap();
            let automaton: Trie = deserialize(&fs::read(automaton_file).unwrap()[..]).unwrap();
            greedy(
                &taquin,
                |t| t.sorted_neighbours(&spiral).into_iter().zip(repeat(1)),
                |t, a| t.move_piece(a).unwrap(),
                |t| heuristique(t, &spiral),
                |t| t.is_solved(&spiral),
                TrieType::Match(0),
                |old_state, dir| automaton.change_true_state(old_state, dir),
                |t| *t == TrieType::Redundant,
            ).unwrap()
        }
        "astar" => astar(
            &taquin,
            |t| t.neighbours().into_iter().zip(repeat(1)),
            |t, a| t.move_piece(a).unwrap(),
            |t| heuristique(t, &spiral),
            |t| t.is_solved(&spiral),
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
            if !taquin.is_solvable(&spiral) {
                println!("this is unsolvable");
                return;
            }

            assert_eq!(
                idastar(
                    &taquin,
                    |t| t.sorted_neighbours(&spiral).into_iter().zip(repeat(1)),
                    |t, a| t.move_piece(a).unwrap(),
                    |t| t.manhattan_heuristic(&spiral),
                    |t| t.is_solved(&spiral),
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
                    |t| t.manhattan_heuristic(&spiral),
                    |t| t.is_solved(&spiral),
                ).unwrap()
                .0
                .len()
            );
        }
    }
}
