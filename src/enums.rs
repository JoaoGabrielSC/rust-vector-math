use core::fmt;

#[allow(dead_code)]
pub enum VectorOp {
    Add,
    Sub,
    Mul,
    Div,
}

// this will be used for another purpose later rsrsrs
#[derive(Debug)]
pub enum IpAddrKint {
    V4,
    V6,
}

impl fmt::Display for IpAddrKint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrKint::V4 => write!(f, "IpV4"),
            IpAddrKint::V6 => write!(f, "IpV6"),
        }
    }
}
