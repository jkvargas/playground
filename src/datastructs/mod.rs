use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

mod trienode;

#[derive(Debug)]
pub enum TrieError {
    EmptyString,
}

pub type TrieResult<TResult> = Result<TResult, TrieError>;

impl Display for TrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TrieError::EmptyString => write!(f, "Empty string"),
        }
    }
}

impl Error for TrieError {}

pub trait TrieT {
    fn insert(&mut self, key: String) -> TrieResult<()>;
}
