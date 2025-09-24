use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use crate::{Event, IListener};

#[derive(Debug, Clone)]
pub struct DefListener<Ev: Event> {
    id: usize,
    triggers: Vec<Ev>,
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
    fn on_triggers(&self, triggers: Vec<&Ev>) {
        for t in triggers {
            match t {
                _ => {}
            }
        }
    }
    fn get_triggers(&self) -> Vec<&Ev> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
}