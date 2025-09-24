use std::{fmt::Debug};
use crate::prelude::*;
use crate::{emitter::DefEmitter, listener::DefListener};

#[derive(Debug, Clone)]
pub struct DefConversant<Ev: Event> {
    em_id: usize,
    li_id: usize,
    handlers: Vec<EHRc<Ev>>,
    triggers: Vec<Ev>,
}

impl<Ev: Event> DefConversant<Ev>  {
    pub fn new(handlers: Vec<EHRc<Ev>>) -> Self {
        DefConversant { handlers,
            triggers: Vec::new(),
            em_id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            li_id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
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
    fn add_handler(&mut self, handler: EHRc<Ev>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<Ev>> {
        self.handlers.clone()
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