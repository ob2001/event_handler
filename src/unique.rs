use crate::prelude::*;

pub trait Id = PartialEq + Debug + Clone;

pub trait Unique<I: Id> {
    fn get_id(&self) -> I;
}

impl<I: Id> PartialEq for dyn Unique<I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}