use crate::*;

#[derive(Debug)]
pub struct FocusColumn <T> {
    pub index: usize,
    pub items: Vec<T>,
    pub focus: bool
}
