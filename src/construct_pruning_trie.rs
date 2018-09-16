use maxdir::MaxDir;
use std::cmp::Ordering;
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
    trie.update_failure();
    (trie, redundant_paths, primitive_paths)
}
