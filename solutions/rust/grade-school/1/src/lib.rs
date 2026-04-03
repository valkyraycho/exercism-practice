use std::collections::BTreeMap;

pub struct School {
    roster: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .roster
            .values()
            .any(|students| students.contains(&student.to_string()))
        {
            return;
        }

        self.roster
            .entry(grade)
            .or_default()
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.roster.get(&grade).cloned().unwrap_or_default();
        students.sort_unstable();
        students
    }
}
