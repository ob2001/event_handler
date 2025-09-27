use std::{fmt::Debug};
use crate::prelude::*;
use crate::{IDCOUNTER, emitter::DefEmitter, listener::DefListener};

#[derive(Clone)]
pub struct DefConversant<Ev: Event> {
    em_id: usize,
    li_id: usize,
    handlers: Vec<EHRc<Ev>>,
    triggers: Vec<Ev>,
}

impl<Ev: Event> Debug for DefConversant<Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefConversant")
            .field("em_id", &self.em_id)
            .field("li_id", &self.li_id)
            .field("handlers", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("triggers", &self.triggers)
            .finish()
    }
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
    fn get_id(&self) -> usize {
        self.em_id
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
    fn get_id(&self) -> usize {
        self.li_id
    }
}