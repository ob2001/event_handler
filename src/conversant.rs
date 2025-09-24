use std::{fmt::Debug};
use crate::{emitter::DefEmitter, listener::DefListener, EHRc, Event, IEmitter, IListener, EMCOUNTER, LICOUNTER};

#[derive(Debug, Clone)]
pub struct DefConversant<Ev: Event> {
    em_id: usize,
    li_id: usize,
    parents: Vec<EHRc<Ev>>,
    triggers: Vec<Ev>,
}

impl<Ev: Event> DefConversant<Ev>  {
    pub fn new(parents: Vec<EHRc<Ev>>) -> Self {
        DefConversant { parents,
            triggers: Vec::new(),
            em_id: EMCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            li_id: LICOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

impl<Ev: Event> PartialEq for DefConversant<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.em_id == other.em_id && self.li_id == other.li_id
    }
}

impl<Ev: Event> PartialEq<DefListener<Ev>> for DefConversant<Ev> {
    fn eq(&self, other: &DefListener<Ev>) -> bool {
        self.li_id == other.get_id()
    }
}

impl<Ev: Event> PartialEq<DefEmitter<Ev>> for DefConversant<Ev> {
    fn eq(&self, other: &DefEmitter<Ev>) -> bool {
        self.em_id == other.get_id()
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