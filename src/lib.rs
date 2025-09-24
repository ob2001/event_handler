#![feature(trait_alias, type_alias_impl_trait)]

use std::{cell::RefCell, fmt::Debug, rc::Rc, sync::atomic::AtomicUsize};
use crate::{event_handler::EventHandler};

// pub type Emitter<Ev: Event> = impl IEmitter<Ev>;
// pub type Listener<Ev: Event> = impl IListener<Ev>;

pub type Emitter<Ev> = dyn IEmitter<Ev>;
pub type Listener<Ev> = dyn IListener<Ev>;

pub type EHRc<Ev> = Rc<RefCell<EventHandler<Ev>>>;
pub type EmRC<Ev> = Rc<RefCell<Emitter<Ev>>>;
pub type LiRC<Ev> = Rc<RefCell<Listener<Ev>>>;

pub mod event_handler;
pub mod sub_event_handler;
pub mod eh_parent;
pub mod emitter;
pub mod listener;
pub mod conversant;

pub trait Event = Debug + PartialEq + Copy;

pub trait IEmitter<Ev: Event>: Debug {
    // Cause emitter to emit events without regard
    // for context.
    // Implementation specific to each emitter.
    // May return any number of events in reaction.
    fn emit(&self) -> Option<Vec<Ev>>;
    fn add_handler(&mut self, parent: EHRc<Ev>);
    fn get_handlers(&self) -> Vec<EHRc<Ev>>;
}

pub trait IListener<Ev: Event>: Debug {
    // Return a view of of all events this listener
    // can be triggered by
    fn get_triggers(&self) -> Vec<&Ev>;

    // Contains logic on how to behave when any trigger/s
    // are broadcast to this listener.
    // May return any number of events in reaction.
    fn on_triggers(&self, triggers: Vec<&Ev>);
}

pub static EHCOUNTER: AtomicUsize = AtomicUsize::new(0);
pub static EMCOUNTER: AtomicUsize = AtomicUsize::new(0);
pub static LICOUNTER: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use crate::{emitter::DefEmitter, event_handler::*, listener::DefListener, IEmitter};

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum TestEvents {
        E1,
        E2,
        E3,
        E4(i32),
        E5(&'static str),
    }

    // *** Tests start here *** //
    #[test]
    fn empty_initialization() {
        use TestEvents::*;
        let eh1 = EventHandler::<TestEvents>::new_ehrc();
        let eh2 = EventHandler::<TestEvents>::new_ehrc();
        let em1 = DefEmitter::new(vec![eh1.clone()]);
        let em2 = DefEmitter::new(vec![]);
        let li1 = DefListener::new(vec![E1, E2, E3, E4(3), E5("Hi")]);
        let li2 = DefListener::new(vec![]);

        assert_ne!(eh1, eh2);
        assert_ne!(em1, em2);
        assert_ne!(li1, li2);

        assert_eq!(eh1.borrow().get_stack().len(), 0);

        assert_eq!(em1.get_handlers()[0], eh1);
    }

    #[test]
    fn stack_manipulation() {
        // todo: Finish test
    }

    #[test]
    fn emitter_creation_and_addition() {
        // todo: Finish test
    }

    #[test]
    fn listener_creation_and_addition() {
        // todo: Finish Test
    }

    #[test]
    fn emit() {
        // todo
    }

    #[test]
    fn listen() {
        // todo
    }

    #[test]
    fn broadcast() {
        // todo
    }
}
