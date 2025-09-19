#![feature(trait_alias)]

use std::fmt::Debug;

pub mod event_handler;

pub type EmMutRef<'a, Ev> = &'a mut dyn Emitter<'a, Ev>;
pub type LiMutRef<'a, Ev> = &'a mut dyn Listener<'a, Ev>;

pub trait Event<'a> = Debug + PartialEq + Copy + 'a;

// DO NOT change types of function arguments or returns
// I have thought them through!
pub trait Emitter<'a, Ev: Event<'a>> {
    // Cause emitter to emit events without regard
    // for context.
    // Implementation specific to each emitter.
    // May return any number of events in reaction.
    fn emit(&self) -> Option<Vec<Ev>>;
}

// DO NOT change types of function arguments or returns
// I have thought them through!
pub trait Listener<'a, Ev: Event<'a>> {
    // Return a view of of all events this listener
    // can be triggered by
    fn triggers(&self) -> Vec<&Ev>;

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
    use crate::{EmMutRef, LiMutRef};

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
    pub struct TestEmitter<TestEvents> { 
        emissions: Option<Vec<TestEvents>>,
    }

    impl<'a> Emitter<'a, TestEvents> for TestEmitter<TestEvents> {
        fn emit(&self) -> Option<Vec<TestEvents>> {
            self.emissions.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TestListener<'a, TestEvents> {
        pub triggers: Vec<&'a TestEvents>,
    }

    impl<'a> Listener<'a, TestEvents> for TestListener<'a, TestEvents> {
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

        fn triggers(&self) -> Vec<&TestEvents> {
            self.triggers.clone()
        }
    }

    // *** Tests start here *** //
    #[test]
    fn empty_initialization() {
        let eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);

        assert_eq!(eh.get_stack().len(), 0);
        assert_eq!(eh.get_emitters().len(), 0);
        assert_eq!(eh.get_listeners().len(), 0);
    }

    #[test]
    fn stack_manipulation() {
        let mut eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);

        eh.push_event(Some(TestEvents::E1));
        
        assert_eq!(eh.get_stack().len(), 1);
        assert_eq!(eh.peek_next_event(), Some(&TestEvents::E1));

        assert_eq!(eh.get_stack().len(), 1);

        assert_eq!(eh.pop_next_event(), Some(TestEvents::E1));

        assert_eq!(eh.get_stack().len(), 0);

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
        let mut em = TestEmitter { emissions: Some(vec![TestEvents::E4(3)]) };
        let mut ems = vec![
            TestEmitter { emissions: Some(vec![TestEvents::E1]) },
            TestEmitter { emissions: Some(vec![TestEvents::E2]) },
            TestEmitter { emissions: Some(vec![TestEvents::E4(5)]) },
            TestEmitter { emissions: Some(vec![TestEvents::E1, TestEvents::E3, TestEvents::E5("Hi")]) },
        ];

        assert_eq!(eh.get_emitters().len(), 0);

        eh.register_emitter(&mut em);

        assert_eq!(eh.get_emitters().len(), 1);
        assert_eq!(eh.get_emitters()[0].emit(), Some(vec![TestEvents::E4(3)]));

        eh.register_emitters(&mut ems.iter_mut().map(|e| e as EmMutRef<TestEvents>).collect());

        // todo: Finish test
    }

    #[test]
    fn listener_creation_and_addition() {
        let mut eh: EventHandler<TestEvents> = EventHandler::new(vec![], vec![], vec![]);
        let mut li = TestListener { triggers: vec![&TestEvents::E1] };
        let mut lis = vec![
            TestListener { triggers: vec![&TestEvents::E1] },
            TestListener { triggers: vec![&TestEvents::E2] },
            TestListener { triggers: vec![&TestEvents::E4(3)] },
            TestListener { triggers: vec![&TestEvents::E3, &TestEvents::E2, &TestEvents::E5("Hi")] },
        ];

        assert_eq!(eh.get_listeners().len(), 0);

        eh.register_listener(&mut li);

        assert_eq!(eh.get_listeners().len(), 1);
        assert!(eh.get_listeners()[0].triggers().contains(&&TestEvents::E1));

        eh.register_listeners(&mut lis.iter_mut().map(|e| e as LiMutRef<TestEvents>).collect());

        assert_eq!(eh.get_listeners().len(), 5);

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
