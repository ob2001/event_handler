use std::fmt::Debug;
use crate::{prelude::*};

pub trait Tag = Debug + PartialEq + Copy + 'static;

#[derive(Clone, Debug)]
pub struct Event<T: Tag, I: Id> {
    emitter: EmRC<T, I>,
    tag: Option<T>,
}

impl<T: Tag, I: Id> PartialEq for Event<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.emitter.borrow().get_id() == other.emitter.borrow().get_id() && self.tag == other.tag
    }
}

impl<'a, T: Tag, I: Id> Event<T, I> {
    pub fn new(emitter: EmRC<T, I>, tag: Option<T>) -> Self {
        Self { emitter, tag }
    }
    pub fn get_emitter(&self) -> EmRC<T, I> {
        self.emitter.clone()
    }
    pub fn get_tag(&self) -> Option<T> {
        self.tag
    }
}