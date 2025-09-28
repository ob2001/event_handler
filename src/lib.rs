#![feature(trait_alias, type_alias_impl_trait)]

pub mod prelude;

pub mod event_handler;
pub mod sub_event_handler;
pub mod eh_parent;
pub mod emitter;
pub mod listener;
pub mod conversant;

pub static IDCOUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        event_handler::EventHandler as EH,
        emitter::DefEmitter as DEm,
        listener::DefListener as DLi,
        conversant::DefConversant as DCo
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
        let co1 = DCo::<TEv>::new(vec![eh2.clone()], None);
        let co2 = DCo::<TEv>::new(vec![], Some(vec![E1, E5("Bye")]));

        assert_ne!(eh1, eh2);
        assert_ne!(em1, em2);
        assert_ne!(li1, li2);
        assert_ne!(co1, co2);

        assert_eq!(eh1.borrow().get_stack_len(), 0);
        assert_eq!(eh1.borrow().get_listeners().len(), 0);

        assert_eq!(em1.get_handlers()[0], eh1);
        assert_eq!(co1.get_handlers()[0], eh2);

        println!("{:?}", eh1.borrow());
        println!("{:?}", eh2.borrow());
        println!("{:?}", em1);
        println!("{:?}", em2);
        println!("{:?}", li1);
        println!("{:?}", li2);
        println!("{:?}", co1);
        println!("{:?}", co2);
    }

    #[test]
    fn stack_manipulation() {
        use TestEvents::{self as TEv, *};
        
        let eh = EH::<TEv, usize>::new_ehrc();
        let em = DEm::new_emrc(vec![eh.clone()]);

        println!("Event Handler: {:?}", eh);
        assert_eq!(eh.borrow().get_stack_len(), 0);

        eh.borrow_mut().push_event(Some((em.clone(), E1)));
        println!("Event Handler: {:?}", eh);

        assert_eq!(eh.borrow().get_stack_len(), 1);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_event().unwrap(), &E1);

        eh.borrow_mut().push_events(Some(vec![(em.clone(), E3), (em.clone(), E5("A"))]));
        println!("Event Handler: {:?}", eh);

        assert_eq!(eh.borrow().get_stack_len(), 3);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_event().unwrap(), &E5("A"));

        let next = eh.borrow_mut().pop_next();
        println!("Event Handler: {:?}", eh);

        assert_eq!(next.as_ref().unwrap().0.borrow().get_id(), em.borrow().get_id());
        assert_eq!(next.as_ref().unwrap().1, E5("A"));

        assert_eq!(eh.borrow().get_stack_len(), 2);
        assert_eq!(eh.borrow().peek_next_emitter().unwrap().borrow().get_id(), em.borrow().get_id());
        assert_eq!(eh.borrow().peek_next_event().unwrap(), &E3);
        assert_eq!(eh.borrow().get_prev_event(), next.as_ref());
        println!("Hello");
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
