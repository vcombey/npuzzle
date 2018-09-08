use std::cmp::Ordering;
use taquin::Dir;

#[derive(new, Copy, Clone, Debug, PartialEq, Eq)]
pub struct MaxDir {
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
