use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    s: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.s)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let lookup = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        let mut num = num;
        for (val, s) in lookup {
            while num >= val {
                num -= val;
                result.push_str(s);
            }
        }

        Self { s: result }
    }
}
