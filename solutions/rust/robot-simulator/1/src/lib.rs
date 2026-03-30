// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    cur_dir: Direction,
    position: (i32, i32),
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            cur_dir: d,
            position: (x, y),
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            cur_dir: match self.cur_dir {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            cur_dir: match self.cur_dir {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = self.position;
        Self {
            position: match self.cur_dir {
                Direction::East => (x + 1, y),
                Direction::South => (x, y - 1),
                Direction::West => (x - 1, y),
                Direction::North => (x, y + 1),
            },
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'A' => robot.advance(),
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.cur_dir
    }
}
