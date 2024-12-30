#[derive(Debug, PartialEq)]
pub enum Sector {
    Small(Box<[u8; 128]>),
    Standard(Box<[u8; 512]>),
    Large(Box<[u8; 4096]>),
}
