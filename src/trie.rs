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
    fn add_word_aux(&mut self, state: usize, word: &Vec<Dir>, i: usize) {
        if let Some(letter) = word.get(i) {
            match self.0[state][*letter] {
                Match(next_state) => self.add_word_aux(next_state, word, i + 1),
                Failure(_) => self.new_down(state, *letter, word, i),
                Redundant => {panic!("already redundant"); return;},
            }
        }
    }
    pub fn add_word(&mut self, word: &Vec<Dir>) -> bool {
        if let Redundant = self.match_word(word.iter()) {
            //println!("already redundant {:?}", word);
            return false;
        }
        //println!("word {:?}", word);
        self.add_word_aux(0, word, 0);
        //println!("tree {:?}", self.0);
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
            //println!("tree {:?}",  self.0);
            for o in l.other().into_iter() {
                /*for k in 1..j {
                    self.match_word(word[1..j].iter().chain([*o].iter()))
                }*/
                new_node[*o] = match self.match_word(word[1..j].iter().chain([*o].iter())) {
                    Redundant => Redundant,
                    Match(state) => Failure(state),
                    Failure(_) => unimplemented!(),
                };
            }
            //println!("new_node {:?}", new_node);
            self.0.push(new_node);
            //println!("tree {:?}", self.0);
        }
    }
    fn new_down(&mut self, state: usize, curr_letter: Dir, word: &Vec<Dir>, i: usize) {
        self.0[state][curr_letter] = if i == word.len() - 1 {
            Redundant
        } else {
            Match(self.0.len())
        };
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
            //println!("state {}", state);
            state = match self.change_state(state, *d) {
                Redundant => {
                    return Redundant;
                }
                Failure(new_state) => new_state,
                Match(new_state) => new_state,
            };
            //println!("state after {}", state);
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
        println!("trie: {:#?}", trie);
        println!("len: {:#?}", all_redundant_pahts.len());

        let mut i:usize = 0;
        let mut non_matching = 0;
        for path in all_redundant_pahts {
            i+=1;
            println!("i: {}", i);
            println!("path: {:?}", path);
            if trie.match_word(path.iter()) != Redundant {
                non_matching+=1;
            }
            assert_eq!(trie.match_word(path.iter()), Redundant);
        }
        i = 0;
        for path in primitive_paths {
            i+=1;
            println!("i: {}", i);
            println!("path: {:?}", path);
            if trie.match_word(path.iter()) == Redundant {
                non_matching+=1;
            }
            assert_ne!(trie.match_word(path.iter()), Redundant);
        }
        assert_eq!(non_matching, 0);
        println!("trie: {:#?}", trie.0.len());
    }
    use std::collections::HashSet;
    const MAX_SIZE_TEST: usize = 100000;
    #[test]
    fn fuser() {

        let mut trie = Trie::new();
        let choices = [Dir::Up, Dir::Right, Dir::Left, Dir::Down];
        let mut rng = thread_rng();
        let mut redundant = HashSet::new();
        let mut primitive = HashSet::new();
        for i in 0..MAX_SIZE_TEST {
            let v: Vec<Dir> = (0..(random::<usize>() % 14) + 4).map(|_| *rng.choose(&choices).unwrap()).collect();
            println!("{:?}", v);
            if !redundant.contains(&v) {
                if trie.match_word(v.iter()) != Redundant {
                    primitive.insert(v);
                }
            }

            else if !primitive.contains(&v) && rng.gen() {
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
