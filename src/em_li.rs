use std::{fmt::Debug};
use crate::{prelude::*, event::Event};
use crate::{IDCOUNTER};

#[derive(Clone)]
pub struct DefEmLi<T: Tag> {
    id: usize,
    handlers: Vec<EHRc<T, usize>>,
    triggers: Vec<T>,
}

impl<T: Tag> Debug for DefEmLi<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefConversant")
            .field("id", &self.id)
            .field("handlers", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("triggers", &self.triggers)
            .finish()
    }
}

impl<T: Tag> DefEmLi<T>  {
    pub fn new(handlers: Vec<EHRc<T, usize>>, triggers: Option<Vec<T>>) -> Self {
        DefEmLi { handlers,
            triggers: if triggers.is_some() { triggers.unwrap() } else { vec![] },
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

impl<T: Tag> PartialEq for DefEmLi<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Tag> PartialEq<dyn IListener<T, usize>> for DefEmLi<T> {
    fn eq(&self, other: &dyn IListener<T, usize>) -> bool {
        self.id == other.get_id()
    }
}

impl<T: Tag> PartialEq<dyn IEmitter<T, usize>> for DefEmLi<T> {
    fn eq(&self, other: &dyn IEmitter<T, usize>) -> bool {
        self.id == other.get_id()
    }
}

impl<T: Tag> IEmitter<T, usize> for DefEmLi<T> {
    fn add_handler(&mut self, handler: EHRc<T, usize>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> Vec<EHRc<T, usize>> {
        self.handlers.clone()
    }
    fn get_id(&self) -> usize {
        self.id
    }
    fn into_emrc(self) -> EmRC<T, usize> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> IListener<T, usize> for DefEmLi<T>  {
    fn get_trigger_tags(&self) -> Vec<&T> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
    fn on_triggers(&self, triggers: Vec<Event<T, usize>>) {
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