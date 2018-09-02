extern crate npuzzle;
use npuzzle::construct_pruning_trie::construct_pruning_trie;
use npuzzle::taquin::Dir;
use npuzzle::trie::*;
use std::env;
use std::ops::Index;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("bad number of args, expected one");
        return;
    }
    let dim: u64 = match args[1].parse() {
        Err(e) => {
            println!("prune: {}", e);
            process::exit(1);
        }
        Ok(n) => n,
    };

    construct_pruning_trie();
}
