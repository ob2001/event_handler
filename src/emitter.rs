use std::{fmt::Debug};
use crate::{EHRc, EmRC, Event, IEmitter, EMCOUNTER};

#[derive(Debug, Clone)]
pub struct DefEmitter<Ev: Event> {
    id: usize,
    parents: Vec<EHRc<Ev>>,
}

impl<Ev: Event> PartialEq for DefEmitter<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event + 'static> Into<EmRC<Ev>> for DefEmitter<Ev> {
    fn into(self) -> EmRC<Ev> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> DefEmitter<Ev> {
    pub fn new(parents: Vec<EHRc<Ev>>) -> Self {
        Self { parents, id: EMCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
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
    fn add_handler(&mut self, parent: EHRc<Ev>) {
        self.parents.push(parent);
    }
    fn get_handlers(&self) -> Vec<EHRc<Ev>> {
        self.parents.clone()
    }
}