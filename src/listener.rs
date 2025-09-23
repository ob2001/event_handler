use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use crate::{Event, IListener};

#[derive(Debug, Clone)]
pub struct DefListener<Ev: Event> {
    triggers: Vec<Ev>,
    id: usize,
}

impl<Ev: Event> PartialEq for DefListener<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> DefListener<Ev> {
    pub fn new(triggers: Vec<Ev>) -> Self {
        Self { triggers, id: SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis() as usize }
    }
}

impl<Ev: Event> IListener<Ev> for DefListener<Ev> {
    fn dispatch(&self, trigger: &Ev) -> Option<Vec<Ev>> {
        // todo
        None
    }
    fn on_triggers(&self, triggers: Vec<&Ev>) -> Option<Vec<Ev>> {
        // todo
        None
    }
    fn get_triggers(&self) -> Vec<&Ev> {
        // todo
        vec![]
    }
}