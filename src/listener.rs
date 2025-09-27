use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;
use crate::IDCOUNTER;

pub trait IListener<Ev: Event>: Debug {
    // Return a view of of all events this listener
    // can be triggered by
    fn get_triggers(&self) -> Vec<&Ev>;

    // Contains logic on how to behave when any trigger/s
    // are broadcast to this listener.
    // May return any number of events in reaction.
    fn on_triggers(&self, triggers: Vec<&Ev>);
    fn get_id(&self) -> usize;
}

impl<Ev: Event> PartialEq for dyn IListener<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

pub type LiRC<Ev> = Rc<RefCell<dyn IListener<Ev>>>;

#[derive(Clone, PartialEq)]
pub struct DefListener<Ev: Event> {
    id: usize,
    triggers: Vec<Ev>,
}

impl<Ev: Event> Debug for DefListener<Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefListener")
            .field("id", &self.id)
            .field("triggers", &self.triggers)
            .finish()
    }
}

impl<Ev: Event + 'static> Into<LiRC<Ev>> for DefListener<Ev> {
    fn into(self) -> LiRC<Ev> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> DefListener<Ev> {
    pub fn new(triggers: Vec<Ev>) -> Self {
        Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn new_lirc(triggers: Vec<Ev>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { triggers, id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }))
    }
}

impl<Ev: Event + 'static> DefListener<Ev> {
    pub fn into_emli(self) -> LiRC<Ev> {
        self.into()
    }
}

impl<Ev: Event> IListener<Ev> for DefListener<Ev> {
    fn on_triggers(&self, triggers: Vec<&Ev>) {
        for t in triggers {
            match t {
                _ => {}
            }
        }
    }
    fn get_triggers(&self) -> Vec<&Ev> {
        let mut ret = vec![];
        for t in &self.triggers {
            ret.push(t);
        }
        ret
    }
    fn get_id(&self) -> usize {
        self.id
    }
}