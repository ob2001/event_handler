use std::{fmt::Debug};
use crate::prelude::*;
use crate::{IDCOUNTER};

#[derive(Clone)]
pub struct DefConversant<Ev: Event> {
    id: usize,
    handlers: Vec<EHRc<Ev>>,
    triggers: Vec<Ev>,
}

impl<Ev: Event> Debug for DefConversant<Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefConversant")
            .field("id", &self.id)
            .field("handlers", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("triggers", &self.triggers)
            .finish()
    }
}

impl<Ev: Event> DefConversant<Ev>  {
    pub fn new(handlers: Vec<EHRc<Ev>>, triggers: Option<Vec<Ev>>) -> Self {
        DefConversant { handlers,
            triggers: if triggers.is_some() { triggers.unwrap() } else { vec![] },
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

impl<Ev: Event> PartialEq for DefConversant<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> PartialEq<dyn IListener<Ev>> for DefConversant<Ev> {
    fn eq(&self, other: &dyn IListener<Ev>) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event> PartialEq<dyn IEmitter<Ev>> for DefConversant<Ev> {
    fn eq(&self, other: &dyn IEmitter<Ev>) -> bool {
        self.id == other.get_id()
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
        self.id
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
        self.id
    }
}