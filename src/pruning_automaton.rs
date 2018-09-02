use taquin::Dir;

pub enum State {
    Start,
    Up,
    Down,
    Right,
    Left,
    Redundant,
}

impl State {
    pub fn new() -> State {
        State::Start
    }
}

pub fn dir_to_state(dir: &Dir) -> State {
    match dir {
        Dir::Up => State::Up,
        Dir::Down => State::Down,
        Dir::Right => State::Right,
        Dir::Left => State::Left,
    }
}

pub fn transition(state: &State, dir: &Dir) -> State {
    match (state, dir) {
        (State::Start, d) => dir_to_state(d),
        (State::Up, Dir::Down) => State::Redundant,
        (State::Down, Dir::Up) => State::Redundant,
        (State::Left, Dir::Right) => State::Redundant,
        (State::Right, Dir::Left) => State::Redundant,
        _ => dir_to_state(dir),
    }
}
