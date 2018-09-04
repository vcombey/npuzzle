use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;
use taquin::Dir;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrieType {
    Redundant,
    Failure(usize),
    Match(usize),
}
use self::TrieType::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Trie(Vec<TrieNode>);

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
    fn add_word_aux(&mut self, state: usize, word: &Vec<Dir>, i: usize) {
        if let Some(letter) = word.get(i) {
            match self.0[state][*letter] {
                Match(next_state) => {
                    if i == word.len() - 1 {
                        self.0[state][*letter] = Redundant;
                        panic!("subword of a redundant");
                    }
                    else {
                        self.add_word_aux(next_state, word, i + 1)
                    }
                }
                Failure(_) => self.new_down(state, *letter, word, i),
                Redundant => {
                    panic!("already redundant");
                    return;
                }
            }
        }
    }
    pub fn add_word(&mut self, word: &Vec<Dir>) -> bool {
        if let Redundant = self.match_word(word.iter()) {
            return false;
        }
        self.add_word_aux(0, word, 0);
        return true;
    }
    fn new_down_aux(&mut self, state: usize, word: &Vec<Dir>, i: usize) {
        for j in i..word.len() {
            let mut new_node = TrieNode([Redundant; 4]);
            let l = word[j];
            new_node[l] = if j == word.len() - 1 {
                Redundant
            } else {
                Match(self.0.len() + 1)
            };
            for o in l.other().into_iter() {
                new_node[*o] = match self.match_word(word[1..j].iter().chain([*o].iter())) {
                    Redundant => Redundant,
                    Match(state) => Failure(state),
                    Failure(_) => unimplemented!(),
                };
            }
            self.0.push(new_node);
        }
    }
    fn new_down(&mut self, state: usize, curr_letter: Dir, word: &Vec<Dir>, i: usize) {
        if i == word.len() - 1 {
            self.0[state][curr_letter] = Redundant;
            return ;
        }
        self.0[state][curr_letter] = Match(self.0.len());
        self.new_down_aux(state, word, i + 1);
    }
    pub fn change_state(&self, old_state: usize, dir: Dir) -> TrieType {
        self.0[old_state][dir]
    }
}

impl<'a> Trie {
    pub fn match_word<I: Iterator<Item = &'a Dir>>(&self, mut word: I) -> TrieType {
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
        if state < self.0.len() && self.all_redundant(state) {
            return Redundant;
        }
        Match(state)
    }
}

use rand::prelude::*;

#[cfg(test)]
mod test {
    use super::*;
    use construct_pruning_trie::construct_pruning_trie;
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        println!("trie: {:?}", trie);

        let path = vec![Dir::Right, Dir::Right];
        trie.add_word(&path);
        println!("trie: {:?}", trie);
        assert_eq!(trie.match_word(path.iter()), Redundant);
        assert_ne!(
            trie.match_word(vec![Dir::Right, Dir::Left].iter()),
            Redundant
        );
    }
    #[test]
    fn test_all_redundant_path() {
        let (trie, all_redundant_pahts, primitive_paths) = construct_pruning_trie();
        //trie.check_integrity();
        //    println!("trie: {:#?}", trie);
        println!("len: {:#?}", all_redundant_pahts.len());

        let mut i: usize = 0;
        let mut non_matching = 0;
        for path in all_redundant_pahts {
            i += 1;
            println!("i: {}", i);
            println!("path: {:?}", path);
            if trie.match_word(path.iter()) != Redundant {
                non_matching += 1;
            }
            assert_eq!(trie.match_word(path.iter()), Redundant);
        }
        i = 0;
        for path in primitive_paths {
            i += 1;
            println!("i: {}", i);
            println!("path: {:?}", path);
            if trie.match_word(path.iter()) == Redundant {
                non_matching += 1;
            }
            assert_ne!(trie.match_word(path.iter()), Redundant);
        }
        assert_eq!(non_matching, 0);
        println!("trie: {:#?}", trie.0.len());
    }
    use self::Dir::*;
    #[test]
    fn test_14() {
        let mut trie = Trie::new();
        let v1 = vec![Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Right];
        let v2 = vec![Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Up];
        let v3 = vec! [Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Down];
        let v4 = vec! [Right, Up, Left, Down, Left, Up, Left, Down, Left, Up, Right, Right, Up, Left];
        trie.add_word(&v1);
        trie.add_word(&v2);
        trie.add_word(&v3);
        trie.add_word(&v4);
        assert_eq!(trie.match_word(v1.iter()), Redundant);
        assert_eq!(trie.match_word(v2.iter()), Redundant);
        assert_eq!(trie.match_word(v3.iter()), Redundant);
        assert_eq!(trie.match_word(v4.iter()), Redundant);
    }
    #[test]
    fn big_and_substring() {
        let mut trie = Trie::new();
        let big = vec![Dir::Up, Dir::Right, Dir::Left, Dir::Down, Dir::Up];
        let sub_big = vec![Dir::Up, Dir::Right, Dir::Left, Dir::Down];
        trie.add_word(&big);
        trie.add_word(&sub_big);
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
            let v: Vec<Dir> = (0..14)
                .map(|_| *rng.choose(&choices).unwrap())
                .collect();
            println!("{:?}", v);
            if !redundant.contains(&v) {
                if trie.match_word(v.iter()) != Redundant {
                    primitive.insert(v);
                }
            } else if !primitive.contains(&v) && rng.gen() {
                trie.add_word(&v);
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
