pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if !sides.iter().all(|&side| side > 0) {
            return None;
        }

        let mut sides = sides;
        sides.sort();
        if sides[0] + sides[1] <= sides[2] {
            return None;
        }
        Some(Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
