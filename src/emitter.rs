use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::{prelude::*};
use crate::IDCOUNTER;

pub trait IEmitter<T: Tag, I: Id>: Debug {
    fn add_handler(&mut self, parent: EHRc<T, I>);
    fn get_handlers(&self) -> Vec<EHRc<T, I>>;
    fn get_id(&self) -> I;
    fn into_emrc(self) -> EmRC<T, I>;
}

impl<T: Tag, I: Id> PartialEq for dyn IEmitter<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub type EmRC<T, I> = Rc<RefCell<dyn IEmitter<T, I>>>;

#[derive(Clone, PartialEq)]
pub struct DefEmitter<T: Tag> {
    id: usize,
    handlers: Vec<EHRc<T, usize>>,
}

impl<T: Tag> Debug for DefEmitter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefEmitter")
            .field("id", &self.id)
            .field("handler ids", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .finish()
    }
}

impl<T: Tag> Into<EmRC<T, usize>> for DefEmitter<T> {
    fn into(self) -> EmRC<T, usize> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> DefEmitter<T> {
    pub fn new(handlers: Vec<EHRc<T, usize>>) -> Self {
        Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn new_emrc(handlers: Vec<EHRc<T, usize>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)}))
    }
}

impl<T: Tag> IEmitter<T, usize> for DefEmitter<T>  {
    fn add_handler(&mut self, handler: EHRc<T, usize>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<T, usize>> {
        self.handlers.clone()
    }
    fn get_id(&self) -> usize {
        self.id
    }
    fn into_emrc(self) -> EmRC<T, usize> {
        self.into()
    }
}