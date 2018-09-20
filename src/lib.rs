pub mod astar;
pub mod complexity;
pub mod construct_pruning_trie;
pub mod greedy_search;
pub mod idastar;
#[allow(non_snake_case)]
pub mod maxHeap;
pub mod maxdir;
pub mod solver;
pub mod taquin;
pub mod trie;
pub mod visualizable;

extern crate sdl2;
#[macro_use]
extern crate itertools;

extern crate core;
extern crate num_traits;
#[macro_use]
extern crate derive_new;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
