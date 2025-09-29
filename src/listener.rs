use crate::{prelude::*, event::Event};
use crate::IDCOUNTER;

pub trait IListener<T: Tag, I: Id> {
    fn get_triggers(&self) -> Vec<&T>;
    fn has_trigger(&self, tag: &T) -> bool;
    fn on_triggers(&self, triggers: Vec<Event<T, I>>);
    fn get_id(&self) -> I;
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

pub type LiRC<T, I> = Rc<RefCell<dyn IListener<T, I>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct DefListener<T: Tag> {
    id: usize,
    triggers: Vec<T>,
}

impl<T: Tag> Into<LiRC<T, usize>> for DefListener<T> {
    fn into(self) -> LiRC<T, usize> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> DefListener<T> {
    pub fn new(triggers: Vec<T>) -> Self {
        Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn new_lirc(triggers: Vec<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }))
    }
}

impl<T: Tag> DefListener<T> {
    pub fn into_emli(self) -> LiRC<T, usize> {
        self.into()
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
    fn get_id(&self) -> usize {
        self.id
    }
}