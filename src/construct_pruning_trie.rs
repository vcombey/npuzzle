use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use taquin::{Dir, Taquin};
use trie::Trie;
use trie::TrieType;

const DEFAULT_CLOSED_SET_SIZE: usize = 0x1_0000;
const DEFAULT_OPEN_SET_SIZE: usize = 0x1_0000;
const MAX_DEPTH: usize = 14;

#[derive(new, Copy, Clone, Debug, PartialEq, Eq)]
struct MaxDir {
    pub max_in_dir: [u32; 4],
    pub curr_right_left: i32,
    pub curr_up_down: i32,
}

impl Ord for MaxDir {
    fn cmp(&self, other: &MaxDir) -> Ordering {
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

impl PartialOrd for MaxDir {
    fn partial_cmp(&self, other: &MaxDir) -> Option<Ordering> {
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

impl MaxDir {
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
    pub fn update_curr_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Right => self.curr_right_left += 1,
            Dir::Left => self.curr_right_left -= 1,
            Dir::Up => self.curr_up_down += 1,
            Dir::Down => self.curr_up_down -= 1,
        }
        self.update_max_in_dir();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_dir() {
        let mut m = MaxDir::new([0; 4], 0, 0);
        m.update_curr_dir(Dir::Right);
        m.update_curr_dir(Dir::Right);
        m.update_curr_dir(Dir::Left);
        m.update_curr_dir(Dir::Left);
        m.update_curr_dir(Dir::Left);
        assert_eq!(m.curr_right_left, -1);
        m.update_curr_dir(Dir::Up);
        m.update_curr_dir(Dir::Up);
        m.update_curr_dir(Dir::Down);
        m.update_curr_dir(Dir::Down);
        m.update_curr_dir(Dir::Down);
        assert_eq!(m.curr_up_down, -1);
    }
    #[test]
    fn test_partial_cmp_max_dir() {
        let mut m = MaxDir::new([0; 4], 0, 0);
        m.update_curr_dir(Dir::Up);
        m.update_curr_dir(Dir::Right);
        m.update_curr_dir(Dir::Right);
        m.update_curr_dir(Dir::Up);
        m.update_curr_dir(Dir::Left);
        m.update_curr_dir(Dir::Left);
        m.update_curr_dir(Dir::Left);
        m.update_curr_dir(Dir::Up);
        m.update_curr_dir(Dir::Down);
        let mut res: [u32; 4] = [0; 4];
        res[Dir::Right as usize] = 2;
        res[Dir::Left as usize] = 1;
        res[Dir::Down as usize] = 0;
        res[Dir::Up as usize] = 3;
        assert_eq!(m.max_in_dir, res);
    }
}

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

pub fn construct_pruning_trie() -> (Trie, Vec<Vec<Dir>>, Vec<Vec<Dir>>) {
    let spiral = Taquin::spiral(7);
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
        if curr.path.len() > MAX_DEPTH - 1 {
            break;
        }
        for d in [Dir::Right, Dir::Up, Dir::Down, Dir::Left].into_iter() {
            if let Some(neighbour) = curr.taquin.move_piece(*d) {
                let neighbour_node = Node::from_curr(&curr, *d, neighbour.clone());
                if closed_set.contains_key(&neighbour) {
                    closed_set.get_mut(&neighbour).unwrap().push(neighbour_node.clone());
                    println!("\redundant {:?}", neighbour_node.path);

                    trie.add_word(&neighbour_node.path);
                    redundant_paths.push(neighbour_node.path);

                } else {

                    println!("\nprimitive {:?}", neighbour_node.path);
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
    (trie, redundant_paths, primitive_paths)
}
