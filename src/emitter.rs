use crate::{prelude::*};
use crate::IDCOUNTER;

pub trait IEmitter<T: Tag, I: Id>: Debug {
    fn emit(&self) -> Result<(), &'static str>;
    fn emit_to_handler_by_id(&self, handler_id: usize) -> Result<(), &'static str>;
    fn add_handler(&mut self, handler: &EHRc<T, I>);
    fn get_handlers(&self) -> &Vec<EHRc<T, I>>;
    fn get_handler_by_id(&self, id: usize) -> Option<&EHRc<T, I>>;
    fn has_handler(&self, handler: &EHRc<T, I>)  -> bool;
    fn get_id(&self) -> I;
    fn into_emrc(self) -> EmRC<T, I>;
}

impl<T: Tag, I: Id> PartialEq for dyn IEmitter<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub type EmRC<T, I> = Rc<RefCell<dyn IEmitter<T, I>>>;

#[derive(Clone, PartialEq)]
pub struct DefEmitter<T: Tag> {
    id: usize,
    handlers: Vec<EHRc<T, usize>>,
    def_tag: Option<T>,
}

impl<T: Tag> Debug for DefEmitter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefEmitter")
            .field("id", &self.id)
            .field("handler ids", &self.handlers.iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .field("def_tag", &self.def_tag)
            .finish()
    }
}

impl<T: Tag> Into<EmRC<T, usize>> for DefEmitter<T> {
    fn into(self) -> EmRC<T, usize> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> DefEmitter<T> {
    pub fn new(handlers: Vec<EHRc<T, usize>>, def_tag: Option<T>) -> Self {
        Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag }
    }
    pub fn new_emrc(handlers: Vec<EHRc<T, usize>>, def_tag: Option<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { handlers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag}))
    }
}

impl<T: Tag> IEmitter<T, usize> for DefEmitter<T>  {
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
            Ok(())
        } else {
            Err("Emitter has no such handler")
        }
    }
    fn add_handler(&mut self, handler: &EHRc<T, usize>) {
        #[cfg(test)]
        println!("Emitter {:?} added handler {:?}", self.get_id(), handler.borrow().get_id());

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
        self.into()
    }
}