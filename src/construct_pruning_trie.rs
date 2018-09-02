use std::collections::HashSet;
use std::collections::VecDeque;
use taquin::{Dir, Taquin};
use trie::Trie;

const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;
const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
const MAX_DEPTH: usize = 6;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    path: Vec<Dir>,
    taquin: Taquin,
}

impl Node {
    pub fn new(path: Vec<Dir>, taquin: Taquin) -> Self {
        Node { path, taquin }
    }
}

pub fn construct_pruning_trie() -> Trie {
    let spiral = Taquin::spiral(7);
    let mut closed_set = HashSet::with_capacity(DEFAULT_CLOSED_SET_SIZE);
    let mut open_set = VecDeque::with_capacity(DEFAULT_OPEN_SET_SIZE);
    let init_node = Node::new(Vec::new(), spiral.clone());
    let mut trie = Trie::new();

    open_set.push_back(init_node);
    closed_set.insert(spiral);
    while let Some(curr) = open_set.pop_front() {
        if curr.path.len() > MAX_DEPTH - 1 {
            break;
        }
        for d in [Dir::Right, Dir::Up, Dir::Down, Dir::Left].into_iter() {
            if let Some(neighbour) = curr.taquin.move_piece(*d) {
                let mut neighbour_path = curr.path.clone();
                neighbour_path.push(*d);
                if !closed_set.contains(&neighbour) {
                    let neighbour_node = Node::new(neighbour_path, neighbour.clone());
                    open_set.push_back(neighbour_node);
                    closed_set.insert(neighbour);
                } else {
                    println!("{:?}", neighbour_path);
                    trie.add_word(neighbour_path);
                }
            }
        }
    }
    trie
}
