#[derive(Debug, PartialEq)]
pub enum Error {
    Parse,
}

pub type Result<T> = std::result::Result<T, Error>;
