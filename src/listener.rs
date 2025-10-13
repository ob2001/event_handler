use crate::def_emitter::DefEmitter;
use crate::{prelude::*, event::Event};
use crate::IDCOUNTER;

/// High-level trait to be implemented by all objects
/// to be added as listeners to an event handler
pub trait IListener<T: Tag, I: Id>: EmitObj<I> {
    fn get_triggers(&self) -> Vec<&T>;
    fn has_trigger(&self, tag: &T) -> bool;
    fn on_triggers(&self, triggers: Vec<Event<T, I>>);
    fn as_lirc(&self) -> LiRC<T, I>;
    fn into_lirc(self) -> Result<LiRC<T, I>, &'static str>;
    fn try_into_lirc(self) -> Option<LiRC<T, I>>;
    fn into_emrc(self) -> EmRC<I>;
    fn as_emrc(&self) -> EmRC<I>;
}

impl<T: Tag, I: Id> Debug for dyn IListener<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefListener")
            .field("id", &self.get_id())
            .field("triggers", &self.get_triggers())
            .finish()
    }
}

impl<T: Tag, I: Id> PartialEq for dyn IListener<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<T: Tag, I: Id> PartialEq<dyn EmitObj<I>> for dyn IListener<T, I> {
    fn eq(&self, other: &dyn EmitObj<I>) -> bool {
        self.get_id() == other.get_id()
    }
}

///
#[derive(Clone, Debug)]
pub struct LiRC<T: Tag, I: Id>(Rc<RefCell<dyn IListener<T, I>>>);

impl<T: Tag, I: Id> Deref for LiRC<T, I> {
    type Target = Rc<RefCell<dyn IListener<T, I>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Tag, I: Id> PartialEq for LiRC<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().get_id() == other.0.borrow().get_id()
    }
}

impl<T: Tag, I: Id> PartialEq<EmRC<I>> for LiRC<T, I> {
    fn eq(&self, other: &EmRC<I>) -> bool {
        *self.borrow() == *other.borrow()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefListener<T: Tag> {
    id: usize,
    triggers: Vec<T>,
}

impl<T: Tag> EmitObj<usize> for DefListener<T> {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl<T: Tag> Into<LiRC<T, usize>> for DefListener<T> {
    fn into(self) -> LiRC<T, usize> {
        LiRC(Rc::new(RefCell::new(self)))
    }
}

impl<T: Tag> DefListener<T> {
    pub fn new(triggers: Vec<T>) -> Self {
        Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn new_lirc(triggers: Vec<T>) -> LiRC<T, usize> {
        LiRC(Rc::new(RefCell::new(Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) })))
    }
}

impl<T: Tag> IListener<T, usize> for DefListener<T> {
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
    fn as_lirc(&self) -> LiRC<T, usize> {
        LiRC(Rc::new(RefCell::new(self.clone())))
    }
    fn into_lirc(self) -> Result<LiRC<T, usize>, &'static str> {
        Ok(self.into())
    }
    fn try_into_lirc(self) -> Option<LiRC<T, usize>> {
        Some(self.into())
    }
    fn as_emrc(&self) -> EmRC<usize> {
        EmRC(Rc::new(RefCell::new(self.clone())))
    }
    fn into_emrc(self) -> EmRC<usize> {
        EmRC(Rc::new(RefCell::new(self)))
    }
}

impl<T: Tag> PartialEq<DefEmitter<T>> for DefListener<T> {
    fn eq(&self, other: &DefEmitter<T>) -> bool {
        self.get_id() == other.get_id()
    }
}