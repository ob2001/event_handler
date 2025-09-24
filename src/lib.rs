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
    use crate::event_handler::*;

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
        let eh: EventHandler<TestEvents> = EventHandler::new();

        assert_eq!(eh.get_stack().len(), 0);
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
