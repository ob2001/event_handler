use std::{fmt::Debug};
use crate::prelude::*;
use crate::{IDCOUNTER};

#[derive(Clone)]
pub struct DefEmLi<Ev: Event> {
    id: usize,
    handlers: Vec<EHRc<Ev, usize>>,
    triggers: Vec<Ev>,
}

impl<Ev: Event> Debug for DefEmLi<Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefConversant")
            .field("id", &self.id)
            .field("handlers", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("triggers", &self.triggers)
            .finish()
    }
}

impl<Ev: Event> DefEmLi<Ev>  {
    pub fn new(handlers: Vec<EHRc<Ev, usize>>, triggers: Option<Vec<Ev>>) -> Self {
        DefEmLi { handlers,
            triggers: if triggers.is_some() { triggers.unwrap() } else { vec![] },
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

impl<Ev: Event> PartialEq for DefEmLi<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Ev: Event> PartialEq<dyn IListener<Ev, usize>> for DefEmLi<Ev> {
    fn eq(&self, other: &dyn IListener<Ev, usize>) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event> PartialEq<dyn IEmitter<Ev, usize>> for DefEmLi<Ev> {
    fn eq(&self, other: &dyn IEmitter<Ev, usize>) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event> IEmitter<Ev, usize> for DefEmLi<Ev> {
    fn emit(&self) -> Option<Vec<(EmRC<Ev, usize>, Ev)>> {
        // todo
        None
    }
    fn add_handler(&mut self, handler: EHRc<Ev, usize>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<Ev, usize>> {
        self.handlers.clone()
    }
    fn get_id(&self) -> usize {
        self.id
    }
    fn into_emrc(self) -> EmRC<Ev, usize> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> IListener<Ev, usize> for DefEmLi<Ev>  {
    fn get_triggers(&self) -> Vec<&Ev> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
    fn on_triggers(&self, triggers: Vec<&(EmRC<Ev, usize>, Ev)>) {
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