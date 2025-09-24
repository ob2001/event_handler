#![feature(trait_alias, type_alias_impl_trait)]

use std::sync::atomic::AtomicUsize;

pub mod prelude;

pub mod event_handler;
pub mod sub_event_handler;
pub mod eh_parent;
pub mod emitter;
pub mod listener;
pub mod conversant;

pub static IDCOUNTER: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{event_handler::EventHandler, emitter::DefEmitter, listener::DefListener};

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
