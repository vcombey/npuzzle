use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use taquin::{Dir, Taquin};
use trie::Trie;

const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;
const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
const MAX_DEPTH: usize = 14;

#[derive(new, Clone, Debug, PartialEq, Eq)]
struct Node {
    path: Vec<Dir>,
    taquin: Taquin,
    max_in_dir: [u32; 4],
    curr_right_left: i32,
    curr_up_down: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        if self
            .max_in_dir
            .iter()
            .zip(other.max_in_dir.iter())
            .all(|(a, b)| a < b)
        {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        if self
            .max_in_dir
            .iter()
            .zip(other.max_in_dir.iter())
            .all(|(a, b)| a <= b)
        {
            return Some(Ordering::Greater);
        } else if self
            .max_in_dir
            .iter()
            .zip(other.max_in_dir.iter())
            .all(|(a, b)| b <= a)
        {
            return Some(Ordering::Less);
        }
        return None;
    }
}

impl Node {
    fn update_max_in_dir(&mut self) {
        if self.curr_right_left > 0
            && self.curr_right_left as u32 > self.max_in_dir[Dir::Right as usize]
        {
            self.max_in_dir[Dir::Right as usize] = self.curr_right_left as u32;
        } else if self.curr_right_left < 0
            && self.curr_right_left.abs() as u32 > self.max_in_dir[Dir::Left as usize]
        {
            self.max_in_dir[Dir::Left as usize] = self.curr_right_left.abs() as u32;
        } else if self.curr_up_down > 0
            && self.curr_up_down as u32 > self.max_in_dir[Dir::Up as usize]
        {
            self.max_in_dir[Dir::Up as usize] = self.curr_up_down as u32;
        } else if self.curr_up_down < 0
            && self.curr_up_down.abs() as u32 > self.max_in_dir[Dir::Down as usize]
        {
            self.max_in_dir[Dir::Down as usize] = self.curr_up_down.abs() as u32;
        }
    }
    pub fn from_curr(curr: &Self, dir: Dir, taquin: Taquin) -> Self {
        let mut new = Node {
            path: curr.path.clone(),
            taquin: taquin,
            max_in_dir: curr.max_in_dir,
            curr_right_left: curr.curr_right_left,
            curr_up_down: curr.curr_up_down,
        };
        new.path.push(dir);
        match dir {
            Dir::Right => new.curr_right_left += 1,
            Dir::Left => new.curr_right_left -= 1,
            Dir::Up => new.curr_up_down += 1,
            Dir::Down => new.curr_up_down -= 1,
        }
        new.update_max_in_dir();
        new
    }
}

pub fn construct_pruning_trie() -> (Trie, Vec<Vec<Dir>>, Vec<Vec<Dir>>) {
    let spiral = Taquin::spiral(7);
    let mut closed_set: HashMap<Taquin, BinaryHeap<Node>> =
        HashMap::with_capacity(DEFAULT_CLOSED_SET_SIZE);
    let mut open_set = VecDeque::with_capacity(DEFAULT_OPEN_SET_SIZE);
    let init_node = Node::new(Vec::new(), spiral.clone(), [0; 4], 0, 0);
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
                /*if !closed_set.contains_key(&neighbour) {
                    primitive_paths.push(neighbour_node.path.clone());
                    open_set.push_back(neighbour_node);
                    closed_set.insert(neighbour);
                }
                else {
                    //println!("{:?}", neighbour_path);
                    if trie.add_word(&neighbour_node.path) {
                        nb_duplicate += 1;
                    }
                    all_redundant_paths.push(neighbour_node.path.clone());
                    //println!("tree: {:#?}", trie);
                }*/
            }
        }
    }
    let mut v: Vec<BinaryHeap<Node>> = closed_set.into_iter().map(|(t, c)| c).collect();
    v.sort_unstable_by_key(|b| b.peek().unwrap().path.len());
    for mut e in v {
        let primitive = e.pop().unwrap();
        for redundant in e {
            if redundant.partial_cmp(&primitive).is_some() && trie.add_word(&redundant.path) {
                nb_duplicate += 1;
            }
            redundant_paths.push(redundant.path);
        }
        primitive_paths.push(primitive.path);
    }
    println!("nb duplicate {}", nb_duplicate);
    (trie, redundant_paths, primitive_paths)
}
