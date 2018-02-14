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
}
