use crate::{prelude::*, event::Event};
use crate::{IDCOUNTER};

#[derive(Clone)]
pub struct DefEmLi<T: Tag> {
    id: usize,
    handlers: Vec<EHRc<T, usize>>,
    triggers: Vec<T>,
    def_tag: Option<T>,
}

impl<T: Tag> Debug for DefEmLi<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefConversant")
            .field("id", &self.id)
            .field("handlers", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("triggers", &self.triggers)
            .field("def_tag", &self.def_tag)
            .finish()
    }
}

impl<T: Tag> DefEmLi<T>  {
    pub fn new(handlers: Vec<EHRc<T, usize>>, triggers: Option<Vec<T>>, def_tag: Option<T>) -> Self {
        DefEmLi { handlers,
            triggers: if triggers.is_some() { triggers.unwrap() } else { vec![] },
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            def_tag
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
    fn emit(&self) -> Result<(), &'static str> {
        if self.handlers.len() > 0 {
            self.handlers[0].borrow_mut().receive(self, self.def_tag);
            Ok(())
        } else {
            Err("Emitter has no handlers")
        }
    }
    fn emit_to_handler_by_id(&self, handler_id: usize) -> Result<(), &'static str> {
        if let Some(h) = self.get_handler_by_id(handler_id) {
            h.borrow_mut().receive(self, self.def_tag);
            return Ok(())
        }
        Err("Emitter has no such handler")
    }
    fn add_handler(&mut self, handler: &EHRc<T, usize>) {
        self.handlers.push(handler.clone());
    }
    fn get_handlers(&self) -> &Vec<EHRc<T, usize>> {
        &self.handlers
    }
    fn get_handler_by_id(&self, id: usize) -> Option<&EHRc<T, usize>> {
        for h in self.get_handlers() {
            if h.borrow().get_id() == id {
                return Some(&h)
            }
        }
        None
    }
    fn has_handler(&self, handler: &EHRc<T, usize>)  -> bool {
        self.handlers.contains(handler)
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
    fn get_triggers(&self) -> Vec<&T> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
    fn has_trigger(&self, tag: &T) -> bool {
        self.triggers.contains(tag)
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