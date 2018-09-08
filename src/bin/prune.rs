extern crate getopts;
use getopts::Options;
use std::env;
extern crate npuzzle;
use npuzzle::construct_pruning_trie::construct_pruning_trie;
use npuzzle::trie::*;
use std::str::FromStr;
extern crate bincode;
use bincode::{deserialize, serialize};
use std::fs::File;
use std::io::Write;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optopt("s", "", "size of the taquin", "INTEGER");
    opts.optopt("d", "", "stop at depth", "INTEGER");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let size = usize::from_str(&matches.opt_str("s").unwrap_or("4".to_string())).unwrap();
    let depth = usize::from_str(&matches.opt_str("d").unwrap_or("10".to_string())).unwrap();
    let output = matches.opt_str("o").unwrap_or(format!(
        "prunning_automaton_{}x{}_d{}.serde",
        size, size, depth
    ));
    println!("size {}, depth {}, output {}", size, depth, output);
    let (trie, _, _) = construct_pruning_trie(size, depth);
    let encoded: Vec<u8> = serialize(&trie).unwrap();
    let mut f = File::create(output).unwrap();
    f.write(&encoded);
    // 8 bytes for the length of the vector, 4 bytes per float.
    println!("{} | {}", encoded.len(), trie.0.len());

    let decoded: Trie = deserialize(&encoded[..]).unwrap();

    assert_eq!(trie, decoded);
}
