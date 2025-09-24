use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use crate::{EHRc, Event, IEmitter, IListener};

#[derive(Debug, Clone)]
pub struct DefConversant<Ev: Event> {
    parents: Vec<EHRc<Ev>>,
    triggers: Vec<Ev>,
    id: usize
}

impl<Ev: Event> DefConversant<Ev>  {
    pub fn new(parents: Vec<EHRc<Ev>>) -> Self {
        DefConversant { parents, triggers: Vec::new(), id: SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis() as usize }
    }
}

impl<Ev: Event> PartialEq for DefConversant<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> IEmitter<Ev> for DefConversant<Ev> {
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

impl<Ev: Event> IListener<Ev> for DefConversant<Ev>  {
    fn get_triggers(&self) -> Vec<&Ev> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
    fn on_triggers(&self, triggers: Vec<&Ev>) {
        for t in triggers {
            match t {
                _ => {}
            }
        }
    }
}