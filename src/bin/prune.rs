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
    let size = match usize::from_str(&matches.opt_str("s").unwrap_or("4".to_string()))  {
		Ok(size) => size,
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	};

	if size == 0 {
		eprintln!("Invalid size specified: {}", size);
		::std::process::exit(1);
	}
	
	let depth = match usize::from_str(&matches.opt_str("d").unwrap_or("10".to_string()))  {
		Ok(d) => d,
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	};
    let output = matches.opt_str("o").unwrap_or(format!(
        "prunning_automaton_{}x{}_d{}.serde",
        size, size, depth
    ));
    println!("size {}, depth {}, output {}", size, depth, output);
    let (trie, _, _) = construct_pruning_trie(size, depth);
    let encoded: Vec<u8> = match serialize(&trie)  {
		Ok(file) => file,
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	};
    let mut f = match File::create(output)  {
		Ok(file) => file,
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	};
    match f.write(&encoded) {
		Ok(_) => (),
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	}
    // 8 bytes for the length of the vector, 4 bytes per float.
    println!("{} | {}", encoded.len(), trie.0.len());

    let decoded: Trie = match deserialize(&encoded[..]) {
		Ok(file) => file,
		Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, opts);
            ::std::process::exit(1);
		}
	};

    assert_eq!(trie, decoded);
}
