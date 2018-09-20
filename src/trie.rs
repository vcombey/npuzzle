use std::ops::Index;
use std::ops::IndexMut;
use taquin::Dir;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrieNode([TrieType; 4]);

impl Index<Dir> for TrieNode {
    type Output = TrieType;
    fn index(&self, dir: Dir) -> &Self::Output {
        &self.0[dir as usize]
    }
}

impl IndexMut<Dir> for TrieNode {
    fn index_mut(&mut self, dir: Dir) -> &mut TrieType {
        &mut self.0[dir as usize]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TrieType {
    Redundant,
    Failure(usize),
    Match(usize),
}
use self::TrieType::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trie(pub Vec<TrieNode>);

impl Trie {
    pub fn new() -> Self {
        Trie(vec![TrieNode([Failure(0); 4])])
    }
    pub fn check_integrity(&self) {
        for x in 0..self.0.len() {
            if self.all_redundant(x) {
                panic!("all redundant");
            }
        }
    }
    pub fn all_redundant(&self, state: usize) -> bool {
        self.0[state].0.iter().all(|x| *x == Redundant)
    }
    fn greatest_match(&self, path: &[Dir]) -> TrieType {
        for j in 0..path.len() {
            match self.match_word_no_failure(path[j..].iter()) {
                Failure(_) => {
                    continue;
                }
                Match(s) => return Failure(s),
                Redundant => {
                    return Redundant;
                }
            }
        }
        Failure(0)
    }
    fn update_failure_aux(&mut self, state: usize, path: &mut Vec<Dir>) {
        for (i, t) in self.0[state].0.clone().iter().enumerate() {
            path.push(Dir::from(i));
            self.0[state].0[i] = match t {
                Match(new_state) => {
                    self.update_failure_aux(*new_state, path);
                    Match(*new_state)
                }
                Failure(_) => self.greatest_match(&path[1..]),
                Redundant => Redundant,
            };
            path.pop();
        }
    }
    pub fn update_failure(&mut self) {
        let mut path = Vec::new();
        self.update_failure_aux(0, &mut path);
    }
    fn add_word_aux(&mut self, state: usize, word: &Vec<Dir>, i: usize, debug: bool) {
        if let Some(letter) = word.get(i) {
            match self.0[state][*letter] {
                Match(next_state) => {
                    if i == word.len() - 1 {
                        self.0[state][*letter] = Redundant;
                        panic!("subword of a redundant");
                    } else {
                        self.add_word_aux(next_state, word, i + 1, debug)
                    }
                }
                Failure(_) => self.new_down(state, *letter, word, i, debug),
                Redundant => {
                    //panic!("already redundant");
                    return;
                }
            }
        }
    }
    pub fn add_word(&mut self, word: &Vec<Dir>, debug: bool) -> bool {
        if let Redundant = self.greatest_match(word) {
            if debug {
                println!("already redundant");
            }
            return false;
        }
        self.add_word_aux(0, word, 0, debug);
        return true;
    }
    fn new_down(&mut self, state: usize, curr_letter: Dir, word: &Vec<Dir>, i: usize, debug: bool) {
        if i == word.len() - 1 {
            self.0[state][curr_letter] = Redundant;
            if debug {
                println!("last letter, {:?}", self.0[state]);
            }
            return;
        }
        self.0[state][curr_letter] = Match(self.0.len());

        for j in i + 1..word.len() {
            let mut new_node = TrieNode([Failure(0); 4]);
            let l = word[j];
            new_node[l] = if j == word.len() - 1 {
                Redundant
            } else {
                Match(self.0.len() + 1)
            };
            self.0.push(new_node);
        }
        if debug {
            println!("{:?}", self.0.last());
        }
    }
    fn change_state(&self, old_state: usize, dir: Dir) -> TrieType {
        self.0[old_state][dir]
    }
    pub fn change_true_state(&self, old_state: &TrieType, dir: Dir) -> TrieType {
        match old_state {
            Redundant => Redundant,
            Failure(s) | Match(s) => self.0[*s][dir],
        }
    }
}

impl<'a> Trie {
    pub fn match_word<I: Iterator<Item = &'a Dir>>(&self, word: I) -> TrieType {
        let mut state = 0;
        for d in word {
            state = match self.change_state(state, *d) {
                Redundant => {
                    return Redundant;
                }
                Failure(new_state) => new_state,
                Match(new_state) => new_state,
            };
        }
        //if state < self.0.len() && self.all_redundant(state) {
        //   return Redundant;
        //}
        Match(state)
    }
    pub fn match_word_no_failure<I: Iterator<Item = &'a Dir>>(&self, word: I) -> TrieType {
        let mut state = 0;
        for d in word {
            state = match self.change_state(state, *d) {
                Redundant => {
                    return Redundant;
                }
                Failure(new_state) => return Failure(new_state),
                Match(new_state) => new_state,
            };
        }
        Match(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use construct_pruning_trie::construct_pruning_trie;
    use rand::prelude::*;
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        println!("trie: {:?}", trie);

        let path = vec![Dir::Right, Dir::Right];
        trie.add_word(&path, false);
        println!("trie: {:?}", trie);
        assert_eq!(trie.match_word(path.iter()), Redundant);
        assert_ne!(
            trie.match_word(vec![Dir::Right, Dir::Left].iter()),
            Redundant
        );
    }
    #[test]
    fn test_all_redundant_path() {
        let (trie, all_redundant_pahts, primitive_paths) = construct_pruning_trie(4, 13);
        //trie.check_integrity();
        //    println!("trie: {:#?}", trie);
        println!("len: {:#?}", all_redundant_pahts.len());
        let mut primitive_not_found = Vec::new();
        let mut redundant_not_found = Vec::new();

        let mut i: usize = 0;
        let mut non_matching = 0;
        let choices = [Dir::Up, Dir::Right, Dir::Left, Dir::Down];
        let mut rng = thread_rng();
        for path in all_redundant_pahts {
            i += 1;
            println!("i: {}", i);
            println!("path: {:?}", path);

            if trie.match_word(
                (0..random::<usize>() % 10)
                    .map(|_| rng.choose(&choices).unwrap())
                    .chain(path.iter()),
            ) != Redundant
            {
                non_matching += 1;
                redundant_not_found.push(path);
            }
            //assert_eq!(trie.match_word(path.iter()), Redundant);
        }
        i = 0;
        for path in primitive_paths {
            i += 1;
            println!("i: {}", i);
            println!("path: {:?}", path);
            if trie.match_word(path.iter()) == Redundant {
                non_matching += 1;
                primitive_not_found.push(path);
            }
            //kassert_ne!(trie.match_word(path.iter()), Redundant);
        }
        println!("redundant not found");
        for r in redundant_not_found {
            println!("r: {:?}", r);
        }
        println!("primitive not found");
        for r in primitive_not_found {
            println!("r: {:?}", r);
        }
        println!("trie: {:#?}", trie.0.len());
        trie.match_word(
            [
                Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Left,
            ]
                .iter(),
        );
        assert_eq!(non_matching, 0);
    }
    use self::Dir::*;
    #[test]
    fn test_14() {
        let mut trie = Trie::new();
        let v1 = vec![
            Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Right,
        ];
        let v2 = vec![
            Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Up,
        ];
        let v3 = vec![
            Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Down,
        ];
        let v4 = vec![
            Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Left,
        ];
        trie.add_word(&v1, false);
        trie.add_word(&v2, false);
        trie.add_word(&v3, false);
        trie.add_word(&v4, false);
        trie.update_failure();
        assert_eq!(trie.match_word(v1.iter()), Redundant);
        assert_eq!(trie.match_word(v2.iter()), Redundant);
        assert_eq!(trie.match_word(v3.iter()), Redundant);
        assert_eq!(trie.match_word(v4.iter()), Redundant);
    }
    #[test]
    fn test_suffix() {
        let mut trie = Trie::new();
        let v1 = vec![Right, Up, Left, Down, Right];

        let v2 = vec![Up, Left, Down];
        trie.add_word(&v1, false);
        println!("trie: {:#?}", trie);
        trie.add_word(&v2, false);
        println!("trie: {:#?}", trie);
        trie.update_failure();
        println!("trie: {:#?}", trie);
        assert_eq!(trie.match_word([Right, Up, Left, Down].iter()), Redundant);
    }
    #[test]
    fn big_and_substring() {
        let mut trie = Trie::new();
        let big = vec![Dir::Up, Dir::Right, Dir::Left, Dir::Down, Dir::Up];
        let sub_big = vec![Dir::Up, Dir::Right, Dir::Left, Dir::Down];
        trie.add_word(&big, false);
        trie.add_word(&sub_big, false);
        assert_eq!(trie.match_word(big.iter()), Redundant);
        assert_eq!(trie.match_word(sub_big.iter()), Redundant);
    }
    use std::collections::HashSet;
    const MAX_SIZE_TEST: usize = 10;
    #[test]
    fn fuser() {
        let mut trie = Trie::new();
        let choices = [Dir::Up, Dir::Right, Dir::Left, Dir::Down];
        let mut rng = thread_rng();
        let mut redundant = HashSet::new();
        let mut primitive = HashSet::new();
        for i in 0..MAX_SIZE_TEST {
            let v: Vec<Dir> = (0..14).map(|_| *rng.choose(&choices).unwrap()).collect();
            println!("{:?}", v);
            if !redundant.contains(&v) {
                if trie.match_word(v.iter()) != Redundant {
                    primitive.insert(v);
                }
            } else if !primitive.contains(&v) && rng.gen() {
                trie.add_word(&v, false);
                redundant.insert(v);
            }
        }
        for r in redundant {
            assert_eq!(trie.match_word(r.iter()), Redundant);
        }
        for r in primitive {
            assert_ne!(trie.match_word(r.iter()), Redundant);
        }
    }
}
