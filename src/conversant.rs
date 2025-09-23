use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use crate::{EHRc, Event, IEmitter, IListener};

#[derive(Debug, Clone)]
pub struct Conversant<Ev: Event> {
    parents: Vec<EHRc<Ev>>,
    id: usize
}

impl<Ev: Event> Conversant<Ev>  {
    pub fn new(parents: Vec<EHRc<Ev>>) -> Self {
        Conversant { parents, id: SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis() as usize }
    }
}

impl<Ev: Event> PartialEq for Conversant<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> IEmitter<Ev> for Conversant<Ev> {
    fn add_parent(&mut self, parent: crate::EHRc<Ev>) {
        todo!();
    }
    fn emit(&self) -> Option<Vec<Ev>> {
        todo!();
    }
    fn get_parents(&self) -> Vec<crate::EHRc<Ev>> {
        todo!();
    }
}

impl<Ev: Event> IListener<Ev> for Conversant<Ev>  {
    fn dispatch(&self, trigger: &Ev) -> Option<Vec<Ev>> {
        todo!();
    }
    fn get_triggers(&self) -> Vec<&Ev> {
        todo!();
    }
    fn on_triggers(&self, triggers: Vec<&Ev>) -> Option<Vec<Ev>> {
        todo!();
    }
}