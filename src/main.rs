use rand::Rng;
use std::collections::VecDeque;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Snake,
    Food,
    Wall
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_vec(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Debug)]
struct Game {
    state: Vec<Vec<Option<Field>>>,
    snake: VecDeque<(isize, isize)>,
    direction: Direction,
}

impl Game {
    fn new() -> Self {
        let mut state = vec!(vec!(None; 10); 10);
        state[5][5] = Some(Field::Snake);
        let mut snake = VecDeque::new();
        snake.push_back((5, 5));

        Game {
            state,
            direction: Direction::Right,
            snake,
        }
    }

    fn add_food(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let y = rng.gen_range(0, self.state.len());
            let x = rng.gen_range(0, self.state[0].len());

            if self.state[y][x] == None {
                self.state[y][x] = Some(Field::Food);
                break;
            }
        }
    }

    fn tick(&mut self) -> bool {
        let head = self.snake.back().unwrap();
        let dir = self.direction.to_vec();
        let next = (dir.0 + head.0, dir.1 + head.1);
        let element = self.state.get(next.0 as usize).map(|r| r.get(next.1 as usize));

        if element == None {
            return false;
        }

        let element = element.unwrap().unwrap();

        match element {
            Some(Field::Wall) => {
                return false;
            },
            Some(Field::Food) => {
                self.snake.push_back(next);
                self.state[next.0 as usize][next.1 as usize] = Some(Field::Snake);
                self.add_food();
            },
            Some(Field::Snake) => {
                return false;
            },
            None => {
                self.snake.push_back(next);
                self.state[next.0 as usize][next.1 as usize] = Some(Field::Snake);
                let last = self.snake.pop_front().unwrap();
                self.state[last.0 as usize][last.1 as usize] = None;
            }
        }



        true
    }

    fn dump(&self) {
        for row in &self.state {
            print!("\r|");
            for field in row {
                match field {
                    None => print!(" "),
                    Some(Field::Snake) => print!("H"),
                    Some(Field::Food) => print!("F"),
                    Some(Field::Wall) => print!("W"),
                }
            }
            println!("|");
        }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin();

    let mut game = Game::new();
    game.add_food();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Right => game.direction = Direction::Right,
            Key::Left => game.direction = Direction::Left,
            Key::Up => game.direction = Direction::Up,
            Key::Down => game.direction = Direction::Down,
            _ => break,
        }
        if !game.tick() {
            break;
        }
        game.dump();
    }
}
