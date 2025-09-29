use crate::{prelude::*};
use crate::IDCOUNTER;

pub trait IEmitter<T: Tag, I: Id>: Unique<I> {
    fn emit(&self) -> Result<(), String>;
    fn emit_to_handler_by_id(&self, handler_id: usize) -> Result<(), String>;
    fn add_handler(&mut self, handler: &EHRc<T, I>) -> Result<(), String>;
    fn get_handlers(&self) -> &Vec<EHRc<T, I>>;
    fn get_handler_by_id(&self, id: usize) -> Option<&EHRc<T, I>>;
    fn has_handler(&self, handler: &EHRc<T, I>)  -> bool;
    fn into_emrc(self) -> EmRC<T, I>;
}

impl<T: Tag, I: Id> Debug for dyn IEmitter<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emitter")
            .field("id", &self.get_id())
            .field("handler ids", &self.get_handlers().iter().map(|h| h.borrow().get_id()).collect::<Vec<usize>>())
            .finish()
    }
}

impl<T: Tag, I: Id> PartialEq for dyn IEmitter<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self as &dyn Unique<I> == other as &dyn Unique<I>
    }
}

pub type EmRC<T, I> = Rc<RefCell<dyn IEmitter<T, I>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct DefEmitter<T: Tag> {
    id: usize,
    handlers: Vec<EHRc<T, usize>>,
    def_tag: Option<T>,
}

impl<T: Tag> Into<EmRC<T, usize>> for DefEmitter<T> {
    fn into(self) -> EmRC<T, usize> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> Unique<usize> for DefEmitter<T> {
    fn get_id(&self) -> usize {
        self.id
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
    fn emit(&self) -> Result<(), String> {
        if self.handlers.len() > 0 {
            #[cfg(test)]
            println!("Emitter_{} emitted {:?} to {:?}", self.get_id(), self.def_tag, self.handlers[0].borrow());

            self.handlers[0].borrow_mut().receive(self, self.def_tag);
            Ok(())
        } else {
            Err(format!("Emitter_{} has no handlers", self.get_id()))
        }
    }
    fn emit_to_handler_by_id(&self, handler_id: usize) -> Result<(), String> {
        if let Some(h) = self.get_handler_by_id(handler_id) {
            #[cfg(test)]
            println!("Emitter_{} emitted {:?} to {:?}", self.get_id(), self.def_tag, h.borrow());

            h.borrow_mut().receive(self, self.def_tag);
            Ok(())
        } else {
            Err(format!("Emitter_{} has no handler with id {}", self.get_id(), handler_id))
        }
    }
    fn add_handler(&mut self, handler: &EHRc<T, usize>) -> Result<(), String> {
        if !self.has_handler(&handler) {
            #[cfg(test)]
            println!("{:?} added EventHandler_{}", self, handler.borrow());

            self.handlers.push(handler.clone());
            Ok(())
        } else {
            Err(format!("Emitter_{:?} already has {:?}", self.id, handler.borrow()))
        }
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
    fn into_emrc(self) -> EmRC<T, usize> {
        self.into()
    }
}