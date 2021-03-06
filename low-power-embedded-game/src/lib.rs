pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let full = dividend / divisor;
    let remainder = dividend % divisor;
    (full, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // best idea: iter.step_by(2)
    // more verbose:
    iter.enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, val)| val)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        // self.0.abs() + self.1.abs()
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}
