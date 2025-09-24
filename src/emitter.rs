use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;

pub trait IEmitter<Ev: Event>: Debug {
    // Cause emitter to emit events without regard
    // for context.
    // Implementation specific to each emitter.
    // May return any number of events in reaction.
    fn emit(&self) -> Option<Vec<Ev>>;
    fn add_handler(&mut self, parent: EHRc<Ev>);
    fn get_handlers(&self) -> Vec<EHRc<Ev>>;
}

pub type EmRC<Ev> = Rc<RefCell<dyn IEmitter<Ev>>>;

#[derive(Debug, Clone)]
pub struct DefEmitter<Ev: Event> {
    id: usize,
    handlers: Vec<EHRc<Ev>>,
}

impl<Ev: Event> PartialEq for DefEmitter<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event + 'static> Into<EmRC<Ev>> for DefEmitter<Ev> {
    fn into(self) -> EmRC<Ev> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> DefEmitter<Ev> {
    pub fn new(handlers: Vec<EHRc<Ev>>) -> Self {
        Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl<Ev: Event + 'static> DefEmitter<Ev> {
    pub fn into_emrc(self) -> EmRC<Ev> {
        self.into()
    }
}

impl<Ev: Event> IEmitter<Ev> for DefEmitter<Ev>  {
    fn emit(&self) -> Option<Vec<Ev>> {
        // todo
        None
    }
    fn add_handler(&mut self, handler: EHRc<Ev>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<Ev>> {
        self.handlers.clone()
    }
}