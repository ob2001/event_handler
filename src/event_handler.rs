use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;
use crate::sub_event_handler::SubEventHandler;

pub trait Event = Debug + PartialEq + Copy;

#[derive(Debug, Clone)]
pub struct EventHandler<Ev: Event> {
    id: usize,
    stack: Vec<(EmRC<Ev>, Ev)>,
    prev_event: Option<(EmRC<Ev>, Ev)>,
    listeners: Vec<LiRC<Ev>>,
}

pub type EHRc<Ev> = Rc<RefCell<EventHandler<Ev>>>;

impl<Ev: Event> PartialEq for EventHandler<Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, T: EHParent<Ev>, Ev: Event> PartialEq<SubEventHandler<'a, T, Ev>> for EventHandler<Ev> {
    fn eq(&self, other: &SubEventHandler<'a, T, Ev>) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event> Into<EHRc<Ev>> for EventHandler<Ev> {
    fn into(self) -> EHRc<Ev> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event> EventHandler<Ev> {
    pub fn new() -> Self {
        EventHandler { 
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            stack: Vec::new(),
            prev_event: None,
            listeners: Vec::new()
        }
    }

    pub fn new_ehrc() -> Rc<RefCell<Self>> {
        EventHandler::new().into()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn push_event(&mut self, event: Option<(EmRC<Ev>, Ev)>) {
        match event {
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Event pushed to stack: {:?}", e);

                self.stack.push(e)
            },
            _ => {}
        }
    }

    pub fn push_events(&mut self, events: Option<Vec<(EmRC<Ev>, Ev)>>) {
        match events {
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Events pushed to stack: {:?}", e);
                
                self.stack.extend(e);
            }
            None => {}
        }
    }

    pub fn get_stack(&self) -> &Vec<(EmRC<Ev>, Ev)> {
        &self.stack
    }

    pub fn get_stack_events(&self) -> Vec<&Ev> {
        self.get_stack().into_iter().map(|e| &e.1).collect()
    }

    pub fn get_stack_emitters(&self) -> Vec<EmRC<Ev>> {
        self.get_stack().into_iter().map(|e| e.0.clone()).collect()
    }

    pub fn add_listener(&mut self, listener: LiRC<Ev>) {
        self.listeners.push(listener)
    }

    pub fn get_listeners(&self) -> &Vec<LiRC<Ev>> {
        &self.listeners
    }

    pub fn peek_next(&self) -> Option<&(EmRC<Ev>, Ev)> {
        #[cfg(debug_assertions)]
        println!("Event peeked: {:?}", self.stack.first());

        self.stack.first()
    }

    pub fn peek_next_event(&self) -> Option<&Ev> {
        if let Some((_, e)) = self.peek_next() {
            Some(e)
        } else {
            None
        }
    }

    pub fn peek_next_emitter(&self) -> Option<&EmRC<Ev>> {
        if let Some((em, _)) = self.peek_next() {
            Some(em)
        } else {
            None
        }
    }

    pub fn pop_next(&mut self) -> Option<(EmRC<Ev>, Ev)> {
        let ret = self.stack.pop();
        #[cfg(debug_assertions)]
        println!("Event popped: {:?}", ret);

        self.prev_event = ret.clone();
        ret
    }

    pub fn get_prev_event(&self) -> &Option<(EmRC<Ev>, Ev)> {
        &self.prev_event
    }

    pub fn consume_next_event(&mut self) {
        let next = self.pop_next();

        match next {
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Consumed event: {:?}", e);

                self.broadcast_event(e);
            }
            None => ()
        }
    }

    pub fn broadcast_event(&mut self, event: (EmRC<Ev>, Ev)) {
        #[cfg(debug_assertions)]
        println!("Broadcast event: {:?}", event);

        for li in self.get_listeners() {
            if li.borrow().get_triggers().contains(&&event.1) {
                li.borrow().on_triggers(vec![&&event.1]);
            }
        }
    }

    pub fn broadcast_events(&mut self, events: Vec<(EmRC<Ev>, Ev)>) {
        for e in events {
            self.broadcast_event(e);
        }
    }
}