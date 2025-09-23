use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use crate::{EHRc, Event, IEmitter};

#[derive(Debug, Clone)]
pub struct DefEmitter<Ev: Event> {
        parents: Vec<EHRc<Ev>>,
        id: usize,
}

impl<Ev: Event> PartialEq for DefEmitter<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> DefEmitter<Ev> {
    pub fn new(parents: Vec<EHRc<Ev>>) -> Self {
        Self { parents, id: SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis() as usize }
    }
}

impl<Ev: Event> IEmitter<Ev> for DefEmitter<Ev>  {
    fn emit(&self) -> Option<Vec<Ev>> {
        // todo
        None
    }
    fn add_parent(&mut self, parent: EHRc<Ev>) {
        self.parents.push(parent);
    }
    fn get_parents(&self) -> Vec<EHRc<Ev>> {
        let mut ret = vec![];
        for p in &self.parents {
            ret.push(p.clone());
        }
        ret
    }
}