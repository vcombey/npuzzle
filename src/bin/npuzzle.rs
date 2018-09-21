extern crate getopts;
extern crate itertools;
extern crate npuzzle;
extern crate sdl2;

use getopts::Options;
use npuzzle::astar::astar;
use npuzzle::greedy_search::greedy_search;
use npuzzle::idastar::idastar;
use npuzzle::taquin::Taquin;
use npuzzle::trie::*;
use npuzzle::visualizable::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::repeat;
use std::str::FromStr;
extern crate bincode;
use bincode::deserialize;
use std::time::SystemTime;

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
    opts.optopt("a", "automaton", "serde file of automaton", "PATH");
    opts.optopt(
        "v",
        "visu",
        "visualization mode",
        "IMAGE_PATH",
    );
    opts.optopt(
        "r",
        "random",
        "generate a random taquin instead of passing a file",
        "SIZE",
    );
    opts.optopt(
        "g",
        "alg",
        "Algorithm",
        "(astar | idastar | uniform_cost | greedy_search)",
    );
    opts.optopt(
        "q",
        "heurisique",
        "Heuristique",
        "(manhattan | linear_conflict | hamming_distance)",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f);
            ::std::process::exit(1);
        },
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let algorithm = match matches.opt_str("g") {
        Some(a) => a,
        None => String::from_str("astar").unwrap(),
    };

    let heuristique: fn(&Taquin, &Taquin) -> u64 = match matches.opt_str("q") {
        Some(s) => match s.as_str() {
            "manhattan" => |t: &Taquin, s: &Taquin| t.manhattan_heuristic(s),
            "linear_conflict" => |t: &Taquin, s: &Taquin| t.manhattan_heuristic_linear_conflict(s),
            "hamming_distance" => |t: &Taquin, s: &Taquin| t.hamming_distance_heuristic(s),
            _ => {
                eprintln!("Unknown algorithm");
                print_usage(&program, opts);
                ::std::process::exit(1);
            }
        },
        None => |t: &Taquin, s: &Taquin| t.manhattan_heuristic(s),
    };

    let taquin = match matches.opt_str("r") {
        Some(size) => Taquin::new_random(usize::from_str(&size).unwrap()),
        None => {
            let taquin_file = if matches.free.len() == 1 {
                matches.free[0].clone()
            } else {
                print_usage(&program, opts);
                return;
            };
            let s = match read_file(&taquin_file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", e);
                    ::std::process::exit(1);
                }
            };
            match s.parse::<Taquin>() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}", e);
                    ::std::process::exit(1);
                }
            }
        }
    };
    println!("TAQUIN: {}", taquin);
    let spiral = Taquin::spiral(taquin.dim());

    if !taquin.is_solvable(&spiral) {
        println!("this is unsolvable");
        return;
    }
    let now = SystemTime::now();

    let mut sol = match algorithm.as_str() {
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
        "greedy_search" => {
            greedy_search(
                &taquin,
                |t| t.sorted_neighbours(&|t| heuristique(t, &spiral)).into_iter().zip(repeat(1)),
                |t, a| t.move_piece(a).unwrap(),
                |t| heuristique(t, &spiral),
                |t| t.is_solved(&spiral),
            ).unwrap()
        }
        "astar" => astar(
            &taquin,
            |t| t.neighbours().into_iter().zip(repeat(1)),
            |t, a| t.move_piece(a).unwrap(),
            |t| heuristique(t, &spiral),
            |t| t.is_solved(&spiral),
        ).unwrap(),
        "uniform_cost" => astar(
            &taquin,
            |t| t.neighbours().into_iter().zip(repeat(1)),
            |t, a| t.move_piece(a).unwrap(),
            |_t| 0,
            |t| t.is_solved(&spiral),
        ).unwrap(),
        _ => {
            eprintln!("Unknown algorithm");
            print_usage(&program, opts);
            ::std::process::exit(1);
        }
    };

    println!("PATH: ");
    sol.0.reverse();
    for p in &sol.0 {
        println!("{}", p);
    }
    match now.elapsed() {
        Ok(elapsed) => {
            println!(
                "RESOLVED TIME:\t\t{} secondes and {} milisecondes",
                elapsed.as_secs(),
                elapsed.subsec_millis()
            );
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("COMPLEXITY IN SIZE:\t{}", sol.1.in_size);
    println!("COMPLEXITY IN TIME:\t{}", sol.1.in_time);
    println!("PATH LEN:\t\t{}", sol.0.len());
    match matches.opt_str("v") {
        Some(image_path) => {
            if let Err(_) = visualize_path(sol.0, image_path, &spiral) {
                std::process::exit(1);
            }
        },
        None => println!("you should realy try the realy super visualisator mode available with -v option"),
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
