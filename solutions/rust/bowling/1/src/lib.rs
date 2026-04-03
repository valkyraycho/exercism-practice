#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    scores: Vec<u16>,
    current_frame: u8,
    current_roll: u8,
    ended: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            current_frame: 1,
            current_roll: 1,
            ended: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.validate(pins)?;
        self.scores.push(pins);
        self.advance_state(pins);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.ended {
            return None;
        }

        let mut total = 0;
        let mut i = 0;

        for frame in 1..=10 {
            if frame < 10 {
                if self.scores[i] == 10 {
                    // strike
                    total += 10 + self.scores[i + 1] + self.scores[i + 2];
                    i += 1;
                } else if self.scores[i] + self.scores[i + 1] == 10 {
                    // spare
                    total += 10 + self.scores[i + 2];
                    i += 2;
                } else {
                    total += self.scores[i] + self.scores[i + 1];
                    i += 2;
                }
            } else {
                total += self.scores[i..].iter().sum::<u16>()
            }
        }
        Some(total)
    }

    fn validate(&self, pins: u16) -> Result<(), Error> {
        if self.ended {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.current_roll == 2
            && self.current_frame != 10
            && (pins + self.scores.last().copied().unwrap()) > 10
        {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.current_frame == 10 {
            if self.current_roll == 2 {
                let roll1 = self.scores.last().copied().unwrap();
                if roll1 != 10 && pins + roll1 > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if self.current_roll == 3 {
                let roll2 = self.scores.last().copied().unwrap();
                let roll1 = self.scores[self.scores.len() - 2];
                if roll1 == 10 && roll2 != 10 && pins + roll2 > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }
        Ok(())
    }
    fn advance_normal_frame(&mut self, pins: u16) {
        match (self.current_roll, pins) {
            (1, 10) => self.current_frame += 1,
            (1, _) => self.current_roll = 2,
            (_, _) => {
                self.current_roll = 1;
                self.current_frame += 1;
            }
        }
    }
    fn advance_tenth_frame(&mut self, pins: u16) {
        match self.current_roll {
            1 => self.current_roll = 2,
            2 => {
                let roll1 = self.scores[self.scores.len() - 2];
                if roll1 == 10 || roll1 + pins == 10 {
                    self.current_roll = 3;
                } else {
                    self.ended = true;
                }
            }
            3 => self.ended = true,
            _ => unreachable!(),
        }
    }
    fn advance_state(&mut self, pins: u16) {
        if self.current_frame == 10 {
            self.advance_tenth_frame(pins);
        } else {
            self.advance_normal_frame(pins);
        }
    }
}
