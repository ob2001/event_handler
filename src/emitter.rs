use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;
use crate::IDCOUNTER;

pub trait IEmitter<Ev: Event, I: Id>: Debug {
    // Cause emitter to emit events without regard
    // for context.
    // Implementation specific to each emitter.
    // May return any number of events in reaction.
    fn emit(&self) -> Option<Vec<Ev>>;
    fn add_handler(&mut self, parent: EHRc<Ev, I>);
    fn get_handlers(&self) -> Vec<EHRc<Ev, I>>;
    fn get_id(&self) -> I;
}

impl<Ev: Event, I: Id> PartialEq for dyn IEmitter<Ev, I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub type EmRC<Ev, Id> = Rc<RefCell<dyn IEmitter<Ev, Id>>>;

#[derive(Clone, PartialEq)]
pub struct DefEmitter<Ev: Event> {
    id: usize,
    handlers: Vec<EHRc<Ev, usize>>,
}

impl<Ev: Event> Debug for DefEmitter<Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefEmitter")
            .field("id", &self.id)
            .field("handler ids", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .finish()
    }
}

impl<Ev: Event> Into<EmRC<Ev, usize>> for DefEmitter<Ev> {
    fn into(self) -> EmRC<Ev, usize> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> DefEmitter<Ev> {
    pub fn new(handlers: Vec<EHRc<Ev, usize>>) -> Self {
        Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn new_emrc(handlers: Vec<EHRc<Ev, usize>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)}))
    }
}

impl<Ev: Event> DefEmitter<Ev> {
    pub fn into_emrc(self) -> EmRC<Ev, usize> {
        self.into()
    }
}

impl<Ev: Event> IEmitter<Ev, usize> for DefEmitter<Ev>  {
    fn emit(&self) -> Option<Vec<Ev>> {
        // todo
        None
    }
    fn add_handler(&mut self, handler: EHRc<Ev, usize>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<Ev, usize>> {
        self.handlers.clone()
    }
    fn get_id(&self) -> usize {
        self.id
    }
}