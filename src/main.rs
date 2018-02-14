extern crate npuzzle;
use npuzzle::taquin::Taquin;
use npuzzle::solver::Solver;
use std::fs::File;
use std::io::Read;
use std::env;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

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
    //   println!("{}", s);
    let taquin = s.parse::<Taquin>().unwrap();
    println!("{:?}", taquin);
    let solver = Solver::new(taquin);
    println!("{}", solver.is_solved());
}
