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
    fn add_word_aux<I: Iterator<Item = Dir>>(&mut self, state: usize, mut word: I) {
        if let Some(letter) = word.next() {
            match self.0[state][letter] {
                Match(next_state) => self.add_word_aux(next_state, word),
                Failure(_) => self.new_down(state, letter, word),
                Redundant => panic!("subword redundant in add_word"),
            }
        }
        //panic!("end of word")
        //TODO: see
    }
    pub fn add_word(&mut self, word: Vec<Dir>) {
        self.add_word_aux(0, word.into_iter());
    }
    fn new_down_aux(&mut self, state: usize, end_word: Vec<Dir>) {
        for i in 1..end_word.len() {
            let mut new_node = TrieNode([Redundant; 4]);
            let l = end_word[i];
            new_node[l] = if i == end_word.len() {
                Match(self.0.len() + 1)
            } else {
                Redundant
            };
            for o in l.other().into_iter() {
                new_node[*o] = self.match_word(end_word[0..i].iter().chain([*o].iter()));
            }
            println!("new_node {:?}", new_node);
            self.0.push(new_node);
            println!("new_node {:?}", self.0);
        }
    }
    fn new_down<I: Iterator<Item = Dir>>(&mut self, state: usize, curr_letter: Dir, rest: I) {
        println!("coucou");
        self.0[state][curr_letter] = Match(self.0.len());
        println!("coucou {:?}", self.0);
        let mut end_word: Vec<Dir> = vec![curr_letter];
        end_word.extend(rest);
        self.new_down_aux(state, end_word);
    }
    pub fn change_state(&self, old_state: usize, dir: Dir) -> TrieType {
        self.0[old_state][dir]
    }
}

impl<'a> Trie {
    pub fn match_word<I: Iterator<Item = &'a Dir>>(&self, mut word: I) -> TrieType {
        let mut state = 0;
        for d in word.next() {
            state = match self.change_state(state, *d) {
                Redundant => {
                    return Redundant;
                }
                Failure(new_state) => new_state,
                Match(new_state) => new_state,
            }
        }
        Match(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        println!("trie: {:?}", trie);

        let path = vec![Dir::Right, Dir::Right];
        println!("trie: {:?}", trie);
        trie.add_word(path.clone());
        assert_eq!(trie.match_word(path.iter()), Redundant);
        assert_ne!(
            trie.match_word(vec![Dir::Right, Dir::Left].iter()),
            Redundant
        );
    }
}
