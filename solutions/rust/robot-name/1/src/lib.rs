use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rand::Rng;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    used_names: Rc<RefCell<HashSet<String>>>,
    name: String,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used_names: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let mut name = generate_name(rng);
        while self.used_names.borrow().contains(&name) {
            name = generate_name(rng);
        }
        self.used_names.borrow_mut().insert(name.clone());
        Robot {
            used_names: Rc::clone(&self.used_names),
            name,
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        let mut name = generate_name(rng);
        while self.used_names.borrow().contains(&name) {
            name = generate_name(rng);
        }
        self.used_names.borrow_mut().remove(&self.name);
        self.used_names.borrow_mut().insert(name.clone());
        self.name = name
    }
}

fn generate_name<R: Rng>(rng: &mut R) -> String {
    let mut name = String::with_capacity(5);
    for _ in 0..2 {
        name.push((b'A' + (rng.next_u32() % 26) as u8) as char)
    }
    for _ in 0..3 {
        name.push((b'0' + (rng.next_u32() % 10) as u8) as char)
    }
    name
}
