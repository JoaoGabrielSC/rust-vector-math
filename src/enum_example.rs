use core::fmt;

#[derive(Debug)]
pub enum IpAddrKint {
    V4,
    V6,
}

impl fmt::Display for IpAddrKint {
    // -> compilador : meu tipo IpAddrKint agora sabe como imprimir em formato humano
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrKint::V4 => write!(f, "IpV4"),
            IpAddrKint::V6 => write!(f, "IpV6"),
        }
    }
}
