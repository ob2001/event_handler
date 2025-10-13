use crate::prelude::*;

pub trait Tag = Debug + PartialEq + Copy + 'static;

pub struct Event<T: Tag, I: Id> {
    emitter: EmRC<I>,
    tag: Option<T>,
}

impl<T: Tag, I: Id> Clone for Event<T, I> {
    fn clone(&self) -> Self {
        Event { emitter: self.emitter.clone(), tag: self.tag }
    }
}

impl<T: Tag, I: Id> Debug for Event<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("EmitObj id", &self.emitter.borrow().get_id())
            .field("tag", &self.tag)
            .finish()
    }
}

impl<T: Tag, I: Id> PartialEq for Event<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.emitter.borrow().get_id() == other.emitter.borrow().get_id() && self.tag == other.tag
    }
}

impl<'a, T: Tag, I: Id> Event<T, I> {
    pub fn new(emitter: EmRC<I>, tag: Option<T>) -> Self {
        Self { emitter, tag }
    }
    pub fn get_emitter(&self) -> EmRC<I> {
        self.emitter.clone()
    }
    pub fn get_tag(&self) -> Option<T> {
        self.tag
    }
}