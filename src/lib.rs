#![feature(trait_alias)]

pub mod prelude;

pub mod event;
pub mod event_handler;
pub mod sub_event_handler;
pub mod eh_parent;
pub mod emitter;
pub mod listener;
pub mod em_li;

pub static IDCOUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        event::Event,
        event_handler::EventHandler as EH,
        emitter::DefEmitter as DEm,
        listener::DefListener as DLi,
        em_li::DefEmLi as DEmLi
    };

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
    fn empty_initializations() {
        use TestEvents::{self as TEv, *};

        let eh1 = EH::<TEv, usize>::new_ehrc();
        let eh2 = EH::<TEv, usize>::new_ehrc();
        let em1 = DEm::new(vec![eh1.clone()]);
        let em2 = DEm::new(vec![]);
        let li1 = DLi::new(vec![E1, E2, E3, E4(3), E5("Hi")]);
        let li2 = DLi::new(vec![]);
        let emli1 = DEmLi::<TEv>::new(vec![eh2.clone()], None);
        let emli2 = DEmLi::<TEv>::new(vec![], Some(vec![E1, E5("Bye")]));

        assert_ne!(eh1, eh2);
        assert_ne!(em1, em2);
        assert_ne!(li1, li2);
        assert_ne!(emli1, emli2);

        assert_eq!(eh1.borrow().get_stack_len(), 0);
        assert_eq!(eh1.borrow().get_listeners().len(), 0);

        assert_eq!(em1.get_handlers()[0], eh1);
        assert_eq!(emli1.get_handlers()[0], eh2);

        println!("{:?}", eh1.borrow());
        println!("{:?}", eh2.borrow());
        println!("{:?}", em1);
        println!("{:?}", em2);
        println!("{:?}", li1);
        println!("{:?}", li2);
        println!("{:?}", emli1);
        println!("{:?}", emli2);
    }

    #[test]
    fn stack_manipulation() {
        use TestEvents::{self as TEv, *};
        
        let eh = EH::<TEv, usize>::new_ehrc();
        let em = DEm::new_emrc(vec![eh.clone()]);

        println!("{:?}", eh.borrow());
        assert_eq!(eh.borrow().get_stack_len(), 0);

        eh.borrow_mut().push_event(Some(Event::new(em.clone(), Some(E1))));
        println!("{:?}", eh.borrow());

        assert_eq!(eh.borrow().get_stack_len(), 1);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag().unwrap(), E1);

        eh.borrow_mut().push_events(Some(vec![Event::new(em.clone(), Some(E3)), Event::new(em.clone(), Some(E5("A")))]));
        println!("{:?}", eh.borrow());

        assert_eq!(eh.borrow().get_stack_len(), 3);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag(), Some(E5("A")));

        let next = eh.borrow_mut().pop_next();
        println!("{:?}", eh.borrow());

        assert_eq!(next.as_ref().unwrap().get_emitter().borrow().get_id(), em.borrow().get_id());
        assert_eq!(next.as_ref().unwrap().get_tag(), Some(E5("A")));

        assert_eq!(eh.borrow().get_stack_len(), 2);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag(), Some(E3));
        assert_eq!(eh.borrow().get_prev_event(), next.as_ref());
    }

    #[test]
    fn emitter_creation_and_addition() {
        // todo
    }

    #[test]
    fn listener_creation_and_addition() {
        // todo
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
