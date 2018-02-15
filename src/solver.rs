use taquin::Taquin;

pub struct Solver {
    taquin: Taquin,
    spiral: Taquin,
}

impl Solver {
    pub fn new(taquin: Taquin) -> Self {
        let n = taquin.dim();
        Solver {
            taquin,
            spiral: Taquin::spiral(n),
        }
    }
    pub fn is_solved(&self) -> bool {
        self.taquin
            .iter()
            .zip(self.spiral.iter())
            .all(|(x, y)| x == y)
    }
    fn nb_transposition(&self) -> u64 {
        let mut trans_count = 0;
        let mut pieces = self.taquin.pieces.clone();
        for (index_spiral, nb) in self.spiral.iter().enumerate() {
            let index_pieces = pieces.iter().position(|&x| x == *nb).unwrap();

            if index_spiral != index_pieces {
                trans_count+=1;
                pieces.swap(index_pieces, index_spiral);
            }
        }
        trans_count
    }
    pub fn is_solvable(&self) -> bool {
        let n = self.taquin.dim();
        let nb_trans = self.nb_transposition();
        let nb_move = self.taquin.nb_move_zero();

        println!("{}", nb_trans);
        println!("{}", nb_move);
        // the taquin is solvable if nb_trans and nb_move have the same parity
        (nb_trans + nb_move) % 2 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solved() {
        let taquin = Taquin::spiral(42);
        let solver = Solver::new(taquin);
        assert!(solver.is_solved());
    }
    #[test]
    fn unsolved() {
        let taquin = Taquin::new(3, vec![5, 1, 0, 8, 4, 6, 3, 7, 2]);
        let solver = Solver::new(taquin);
        assert!(!solver.is_solved());
    }
    #[test]
    fn solvable() {
        let taquin = Taquin::new(3, vec![0,8,3,1,6,4,5,7,2]);
        let solver = Solver::new(taquin);
        assert!(solver.is_solvable());
    }
    #[test]
    fn unsolvable() {
        let taquin = Taquin::new(3, vec![1,7,8,2,0,5,3,4,6]);
        let solver = Solver::new(taquin);
        assert!(!solver.is_solvable());
    }
}
