use std::fmt;

pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})\n", self.x, self.y)
    }
}
