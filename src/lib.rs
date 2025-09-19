#![feature(trait_alias)]

use std::{rc::Rc, cell::RefCell};

pub mod event_handler;

pub type EmCRc<Ev> = Rc<RefCell<dyn Emitter<Ev>>>;
pub type LiCRc<Ev> = Rc<RefCell<dyn Listener<Ev>>>;

pub trait Event = PartialEq + Copy + 'static;

pub trait Emitter<Ev: Event> {
    // Cause emitter to emit events without regard
    // for context.
    // Implementation specific to each emitter.
    // May return any number of events in reaction.
    fn emit(&self) -> &Option<Vec<Ev>>;
}

pub trait Listener<Ev: Event> {
    // Return a view of of all events this listener
    // can be triggered by
    fn triggers(&self) -> &Vec<Ev>;

    // Contains logic on how to behave when any trigger/s
    // are broadcast to this listener.
    // May return any number of events in reaction.
    fn on_triggers(&self, triggers: Vec<&Ev>) -> Option<Vec<Ev>>;

    // Handles dispatching individual triggers to specific
    // functionality.
    // May return any number of events in reaction.
    fn dispatch(&self, trigger: &Ev) -> Option<Vec<Ev>>;
}

#[cfg(test)]
mod tests {
    use std::{any::Any, cell::RefCell, rc::Rc};
    use crate::{EmCRc, LiCRc};

    use super::{event_handler::*, Emitter, Listener};

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum TestEvents {
        E1,
        E2,
        E3,
        E4(i32),
        E5(&'static str),
    }

    #[derive(Debug, Clone)]
    struct TestEmitter<TestEvents> { 
        emissions: Option<Vec<TestEvents>>,
    }

    impl Emitter<TestEvents> for TestEmitter<TestEvents> {
        fn emit(&self) -> &Option<Vec<TestEvents>> {
            &self.emissions
        }
    }

    #[derive(Debug, Clone)]
    pub struct TestListener<TestEvents> {
        pub triggers: Vec<TestEvents>,
    }

    impl Listener<TestEvents> for TestListener<TestEvents> {
        fn on_triggers(&self, triggers: Vec<&TestEvents>) -> Option<Vec<TestEvents>> {
            let mut ret = vec![];
            for &t in triggers {
                ret.extend(self.dispatch(&t).unwrap());
            }
            Some(ret)
        }

        fn dispatch(&self, trigger: &TestEvents) -> Option<Vec<TestEvents>> {
            match trigger {
                _ => None
            }
        }

        fn triggers(&self) -> &Vec<TestEvents> {
            &self.triggers
        }
    }

    // *** Tests start here *** //
    #[test]
    fn empty_initialization() {
        let eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);
        let e_v: Vec<TestEvents> = Vec::new();
        let em_v: Vec<EmCRc<TestEvents>> = Vec::new();
        let li_v: Vec<LiCRc<TestEvents>> = Vec::new();

        assert_eq!(eh.get_stack().len(), 0);
        assert_eq!(eh.get_stack().type_id(), e_v.type_id());
        
        assert_eq!(eh.get_emitters().len(), 0);
        assert_eq!(eh.get_emitters().type_id(), em_v.type_id());

        assert_eq!(eh.get_listeners().len(), 0);
        assert_eq!(eh.get_listeners().type_id(), li_v.type_id());
    }

    #[test]
    fn stack_manipulation() {
        let mut eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);
        let ev: Vec<TestEvents> = Vec::new();

        eh.push_event(Some(TestEvents::E1));
        
        assert_eq!(eh.get_stack().len(), 1);
        assert_eq!(eh.peek_next_event(), Some(&TestEvents::E1));

        assert_eq!(eh.get_stack().len(), 1);

        assert_eq!(eh.pop_next_event(), Some(TestEvents::E1));

        assert_eq!(eh.get_stack().len(), 0);
        assert_eq!(eh.get_stack().type_id(), ev.type_id());

        eh.push_events(Some(vec![TestEvents::E1, TestEvents::E2]));

        assert_eq!(eh.get_stack().len(), 2);
        assert_eq!(eh.get_stack(), &vec![TestEvents::E1, TestEvents::E2]);

        eh.push_events(Some(vec![TestEvents::E4(0), TestEvents::E5("a")]));

        assert_eq!(eh.get_stack().len(), 4);
        assert_eq!(eh.get_stack(), &vec![TestEvents::E1, TestEvents::E2, TestEvents::E4(0), TestEvents::E5("a")]);
    }

    #[test]
    fn emitter_creation_and_addition() {
        let mut eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);
        let e_v: Vec<EmCRc<TestEvents>> = Vec::new();
        let em = TestEmitter { emissions: Some(vec![TestEvents::E4(3)]) };

        assert_eq!(eh.get_emitters().len(), 0);
        assert_eq!(eh.get_emitters().type_id(), e_v.type_id());

        eh.register_emitter(Rc::new(RefCell::new(em)));

        assert_eq!(eh.get_emitters().len(), 1);
        assert_eq!(eh.get_emitters()[0].borrow().emit(), &Some(vec![TestEvents::E4(3)]));

        let ems: Vec<EmCRc<TestEvents>> = vec![Rc::new(RefCell::new(TestEmitter {emissions: Some(vec![TestEvents::E2])}))];
        eh.register_emitters(ems);

        // todo: Finish test
    }

    #[test]
    fn listener_creation_and_addition() {
        let mut eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);
        let e_v: Vec<LiCRc<TestEvents>> = Vec::new();
        let li = TestListener { triggers: vec![TestEvents::E2] };

        assert_eq!(eh.get_listeners().len(), 0);
        assert_eq!(eh.get_listeners().type_id(), e_v.type_id());

        eh.register_listener(Rc::new(RefCell::new(li)));

        assert_eq!(eh.get_listeners().len(), 1);
        assert!(eh.get_listeners()[0].borrow().triggers().contains(&TestEvents::E2));

        let lis: Vec<LiCRc<TestEvents>> = vec![Rc::new(RefCell::new(TestListener {triggers: vec![TestEvents::E3]}))];
        eh.register_listeners(lis);

        // todo: Finish test
    }

    #[test]
    fn emission() {
        // todo
    }

    #[test]
    fn listening() {
        // todo
    }
}
