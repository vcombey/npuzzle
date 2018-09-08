use std::cmp::Ordering;
//use std::collections::BinaryHeap;
use maxdir::MaxDir;
use std::collections::HashMap;
use std::collections::VecDeque;
use taquin::{Dir, Taquin};
use trie::{Trie, TrieType};

const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;
const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
#[derive(new, Clone, Debug, PartialEq, Eq)]
struct Node {
    path: Vec<Dir>,
    taquin: Taquin,
    max_dir: MaxDir,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        if self.path.len() > other.path.len() {
            return Ordering::Less;
        } else if self.path.len() < other.path.len() {
            return Ordering::Greater;
        }
        self.max_dir.cmp(&other.max_dir)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        if self.path.len() > other.path.len() {
            return Some(Ordering::Less);
        } else if self.path.len() < other.path.len() {
            return Some(Ordering::Greater);
        }
        self.max_dir.partial_cmp(&other.max_dir)
    }
}

impl Node {
    pub fn from_curr(curr: &Self, dir: Dir, taquin: Taquin) -> Self {
        let mut new = Node {
            path: curr.path.clone(),
            taquin: taquin,
            max_dir: curr.max_dir,
        };
        new.path.push(dir);
        new.max_dir.update_curr_dir(dir);
        new
    }
}

pub fn construct_pruning_trie(
    taquin_dim: usize,
    depth: usize,
) -> (Trie, Vec<Vec<Dir>>, Vec<Vec<Dir>>) {
    let spiral = Taquin::spiral(2 * taquin_dim - 1);
    let mut closed_set: HashMap<Taquin, Vec<Node>> =
        HashMap::with_capacity(DEFAULT_CLOSED_SET_SIZE);
    let mut open_set = VecDeque::with_capacity(DEFAULT_OPEN_SET_SIZE);
    let init_node = Node::new(Vec::new(), spiral.clone(), MaxDir::new([0; 4], 0, 0));
    let mut trie = Trie::new();
    let mut primitive_paths = Vec::new();
    let mut redundant_paths = Vec::new();

    open_set.push_back(init_node.clone());
    closed_set.insert(spiral, vec![init_node].into());
    let mut nb_duplicate = 0;
    while let Some(curr) = open_set.pop_front() {
        if curr.path.len() > depth - 1 {
            break;
        }
        for d in [Dir::Right, Dir::Up, Dir::Down, Dir::Left].into_iter() {
            if let Some(neighbour) = curr.taquin.move_piece(*d) {
                let neighbour_node = Node::from_curr(&curr, *d, neighbour.clone());
                if closed_set.contains_key(&neighbour) {
                    closed_set
                        .get_mut(&neighbour)
                        .unwrap()
                        .push(neighbour_node.clone());
                    //println!("\redundant {:?}", neighbour_node.path);

                    trie.add_word(&neighbour_node.path, false);
                    //    debug_assert!(trie.match_word(neighbour_node.path.iter()) == TrieType::Redundant);

                    redundant_paths.push(neighbour_node.path);
                } else {
                    //println!("\nprimitive {:?}", neighbour_node.path);
                    open_set.push_back(neighbour_node.clone());
                    closed_set.insert(neighbour, vec![neighbour_node.clone()].into());
                    primitive_paths.push(neighbour_node.path);
                }
            }
        }
    }

    /*let mut v: Vec<Vec<Node>> = closed_set.into_iter().map(|(t, c)| c).collect();
    v.sort_by_key(|b| b[0].path.len());
    for mut e in v {
    let primitive = e.remove(0);
    println!("\nprimitive {:?}", primitive.path);

    for redundant in e.into_iter() {
    println!("redundant {:?}", redundant.path);
    nb_duplicate += 1;
    redundant_paths.push(redundant.path);
}
    primitive_paths.push(primitive.path);
}
    redundant_paths.sort_by_key(|b| b.len());
    for r in &redundant_paths {
    trie.add_word(r);
    debug_assert!(trie.match_word(r.iter()) == TrieType::Redundant);
}
    println!("nb duplicate {}", nb_duplicate);
     */
    trie.update_failure();
    (trie, redundant_paths, primitive_paths)
}
/*
pub fn construct_pruning_trie() -> (Trie, Vec<Vec<Dir>>, Vec<Vec<Dir>>) {
let spiral = Taquin::spiral(7);
let mut closed_set: HashMap<Taquin, BinaryHeap<Node>> =
HashMap::with_capacity(DEFAULT_CLOSED_SET_SIZE);
let mut open_set = VecDeque::with_capacity(DEFAULT_OPEN_SET_SIZE);
let init_node = Node::new(Vec::new(), spiral.clone(), MaxDir::new([0; 4], 0, 0));
let mut trie = Trie::new();
let mut primitive_paths = Vec::new();
let mut redundant_paths = Vec::new();

open_set.push_back(init_node.clone());
closed_set.insert(spiral, vec![init_node].into());
let mut nb_duplicate = 0;
while let Some(curr) = open_set.pop_front() {
if curr.path.len() > MAX_DEPTH - 1 {
break;
        }
        for d in [Dir::Right, Dir::Up, Dir::Down, Dir::Left].into_iter() {
            if let Some(neighbour) = curr.taquin.move_piece(*d) {
                let neighbour_node = Node::from_curr(&curr, *d, neighbour.clone());
                if closed_set.contains_key(&neighbour) {
                    closed_set.get_mut(&neighbour).unwrap().push(neighbour_node);
                } else {
                    open_set.push_back(neighbour_node.clone());
                    closed_set.insert(neighbour, vec![neighbour_node].into());
                }
                /*if closed_set.contains_key(&neighbour) {
                    //println!("{:?}", neighbour_path);
                    if trie.add_word(&neighbour_node.path) {
                        nb_duplicate += 1;
                    }
                }
                else {
                    all_redundant_paths.push(neighbour_node.path.clone());
                    //println!("tree: {:#?}", trie);
                    primitive_paths.push(neighbour_node.path.clone());
                    open_set.push_back(neighbour_node);
                    closed_set.insert(neighbour);
                }*/
            }
        }
    }
    let mut v: Vec<BinaryHeap<Node>> = closed_set.into_iter().map(|(t, c)| c).collect();
    v.sort_unstable_by_key(|b| b.peek().unwrap().path.len());
    for mut e in v {
        let primitive = e.pop().unwrap();
        println!("\nprimitive {:?}", primitive.path);
        
        for redundant in e {
            println!("redundant {:?}", redundant.path);
            //if redundant.partial_cmp(&primitive).is_some() {
                nb_duplicate += 1;
                trie.add_word(&redundant.path);
                redundant_paths.push(redundant.path);
            //} else if trie.match_word(redundant.path.iter()) != TrieType::Redundant {
            //    primitive_paths.push(redundant.path);
            //}
        }
        //if trie.match_word(primitive.path.iter()) != TrieType::Redundant {
            primitive_paths.push(primitive.path);
        //}
    }
    println!("nb duplicate {}", nb_duplicate);
    (trie, redundant_paths, primitive_paths)
}
*/
