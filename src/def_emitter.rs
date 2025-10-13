use crate::emit_obj::EmRC;
use crate::listener::DefListener;
use crate::{prelude::*};
use crate::IDCOUNTER;

#[derive(Debug, Clone, PartialEq)]
pub struct DefEmitter<T: Tag> {
    id: usize,
    def_tag: Option<T>,
}

impl<T: Tag> Into<EmRC<usize>> for DefEmitter<T> {
    fn into(self) -> EmRC<usize> {
        EmRC(Rc::new(RefCell::new(self)))
    }
}

impl<T: Tag> EmitObj<usize> for DefEmitter<T> {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl<T: Tag> DefEmitter<T> {
    pub fn new(def_tag: Option<T>) -> Self {
        Self { id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag }
    }
    pub fn new_emrc(def_tag: Option<T>) -> EmRC<usize> {
        EmRC(Rc::new(RefCell::new(Self { id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag})))
    }
    pub fn into_emrc(self) -> EmRC<usize> {
        self.into()
    }
}

impl<T: Tag> PartialEq<DefListener<T>> for DefEmitter<T> {
    fn eq(&self, other: &DefListener<T>) -> bool {
        self.get_id() == other.get_id()
    }
}