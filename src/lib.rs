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
    enum TestTags {
        T1,
        T2,
        T3,
        T4(i32),
        T5(&'static str),
    }

    // *** Tests start here *** //
    #[test]
    fn empty_initializations() {
        use TestTags::{self, *};

        let eh1 = EH::<TestTags, usize>::new_ehrc();
        let eh2 = EH::<TestTags, usize>::new_ehrc();
        let em1 = DEm::new_emrc(vec![eh1.clone()], None);
        let em2 = DEm::new_emrc(vec![], None);
        let li1 = DLi::new_lirc(vec![T1, T2, T3, T4(3), T5("Hi")]);
        let li2 = DLi::new_lirc(vec![]);
        let emli1 = DEmLi::<TestTags>::new(vec![eh2.clone()], None, None);
        let emli2 = DEmLi::<TestTags>::new(vec![], Some(vec![T1, T5("Bye")]), None);

        assert_ne!(eh1, eh2);
        assert_ne!(em1, em2);
        assert_ne!(li1, li2);
        assert_ne!(emli1, emli2);

        assert_eq!(eh1.borrow().get_stack_len(), 0);
        assert_eq!(eh1.borrow().get_listeners().len(), 0);

        assert!(em1.borrow().has_handler(&eh1));
        assert!(emli1.has_handler(&eh2));
        assert!(emli2.has_trigger(&T1));

        println!("{:?}", eh1);
        println!("{:?}", eh2);
        println!("{:?}", em1);
        println!("{:?}", em2);
        println!("{:?}", li1);
        println!("{:?}", li2);
        println!("{:?}", emli1);
        println!("{:?}", emli2);
    }

    #[test]
    fn stack_manipulation() {
        use TestTags::{self, *};
        
        let eh = EH::<TestTags, usize>::new_ehrc();
        let em = DEm::new_emrc(vec![eh.clone()], None);

        println!("{:?}", eh.borrow());
        assert_eq!(eh.borrow().get_stack_len(), 0);

        eh.borrow_mut().push_event(Some(Event::new(em.clone(), Some(T1))));
        println!("{:?}", eh.borrow());

        assert_eq!(eh.borrow().get_stack_len(), 1);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag().unwrap(), T1);

        eh.borrow_mut().push_events(Some(vec![Event::new(em.clone(), Some(T3)), Event::new(em.clone(), Some(T5("A")))]));
        println!("{:?}", eh.borrow());

        assert_eq!(eh.borrow().get_stack_len(), 3);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag(), Some(T5("A")));

        let next = eh.borrow_mut().pop_next();
        println!("{:?}", eh.borrow());

        assert_eq!(next.as_ref().unwrap().get_emitter().borrow().get_id(), em.borrow().get_id());
        assert_eq!(next.as_ref().unwrap().get_tag(), Some(T5("A")));

        assert_eq!(eh.borrow().get_stack_len(), 2);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_tag(), Some(T3));
        assert_eq!(eh.borrow().get_prev_event(), next.as_ref());
    }

    #[test]
    fn emitter_creation_and_addition() {
        use crate::event_handler::register_emitter;
        use TestTags::{self, *};
        let eh = EH::<TestTags, usize>::new_ehrc();
        let em = DEm::<TestTags>::new_emrc(vec![], Some(T1));

        println!("{:?}", eh.borrow());
        println!("{:?}", em.borrow());

        register_emitter(&eh, &(em.clone() as EmRC<TestTags, usize>));

        println!("{:?}", eh.borrow());
        println!("{:?}", em.borrow());
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
