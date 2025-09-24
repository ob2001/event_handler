use std::{fmt::Debug};
use crate::{Event, IListener, LiRC, LICOUNTER};

#[derive(Debug, Clone)]
pub struct DefListener<Ev: Event> {
    id: usize,
    triggers: Vec<Ev>,
}

impl<Ev: Event> PartialEq for DefListener<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
        Self { triggers, id: LICOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
    pub fn get_id(&self) -> usize {
        self.id
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
}