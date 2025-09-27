use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;
use crate::{IDCOUNTER, sub_event_handler::SubEventHandler};

pub trait Event = Debug + PartialEq + Copy;
pub trait Id = PartialEq + Debug + Clone;

#[derive(Clone)]
pub struct EventHandler<Ev: Event, I: Id> {
    id: usize,
    stack: Vec<(EmRC<Ev, I>, Ev)>,
    prev_event: Option<(EmRC<Ev, I>, Ev)>,
    listeners: Vec<LiRC<Ev, I>>,
}

impl<Ev: Event, I: Id> Debug for EventHandler<Ev, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_event_str = if self.prev_event.is_some() { &Some((self.prev_event.as_ref().unwrap().0.borrow().get_id(), self.prev_event.as_ref().unwrap().1)) } else { &None as &Option<(I, Ev)> };
        f.debug_struct("EventHandler")
            .field("id", &self.id)
            .field("stack", &self.stack.iter().map(|e| (e.0.borrow().get_id(), e.1)).collect::<Vec<(I, Ev)>>())
            .field("prev_event", prev_event_str)
            .field("listeners", &self.listeners.iter().map(|l| l.borrow().get_id()).collect::<Vec<I>>())
            .finish()
    }
}

pub type EHRc<Ev, I> = Rc<RefCell<EventHandler<Ev, I>>>;

impl<Ev: Event, I: Id> PartialEq for EventHandler<Ev, I> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, T: EHParent<Ev, I> + Debug, Ev: Event, I: Id> PartialEq<SubEventHandler<'a, T, Ev, I>> for EventHandler<Ev, I> {
    fn eq(&self, other: &SubEventHandler<'a, T, Ev, I>) -> bool {
        self.id == other.get_id()
    }
}

impl<Ev: Event, I: Id> Into<EHRc<Ev, I>> for EventHandler<Ev, I> {
    fn into(self) -> EHRc<Ev, I> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<Ev: Event, I: Id> EventHandler<Ev, I> {
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

    pub fn push_event(&mut self, event: Option<(EmRC<Ev, I>, Ev)>) {
        match event {
            Some(e) => {
                #[cfg(test)]
                println!("Event pushed to stack: (Emitter id: {:?}, Event: {:?})", e.0.borrow().get_id(), e.1);

                self.stack.push(e)
            },
            _ => {}
        }
    }

    pub fn push_events(&mut self, events: Option<Vec<(EmRC<Ev, I>, Ev)>>) {
        if let Some(events) = events {
            for event in events {
                self.push_event(Some(event));
            }
        }
    }

    pub fn get_stack(&self) -> &Vec<(EmRC<Ev, I>, Ev)> {
        &self.stack
    }

    pub fn get_stack_events(&self) -> Vec<&Ev> {
        self.get_stack().into_iter().map(|e| &e.1).collect()
    }

    pub fn get_stack_emitters(&self) -> Vec<EmRC<Ev, I>> {
        self.get_stack().into_iter().map(|e| e.0.clone()).collect()
    }

    pub fn add_listener(&mut self, listener: LiRC<Ev, I>) {
        self.listeners.push(listener)
    }

    pub fn get_listeners(&self) -> &Vec<LiRC<Ev, I>> {
        &self.listeners
    }

    pub fn peek_next(&self) -> Option<&(EmRC<Ev, I>, Ev)> {
        #[cfg(test)]
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

    pub fn peek_next_emitter(&self) -> Option<&EmRC<Ev, I>> {
        if let Some((em, _)) = self.peek_next() {
            Some(em)
        } else {
            None
        }
    }

    pub fn pop_next(&mut self) -> Option<(EmRC<Ev, I>, Ev)> {
        let ret = self.stack.pop();
        #[cfg(test)]
        println!("Event popped: {:?}", ret);

        self.prev_event = ret.clone();
        ret
    }

    pub fn get_prev_event(&self) -> &Option<(EmRC<Ev, I>, Ev)> {
        &self.prev_event
    }

    pub fn consume_next_event(&mut self) {
        let next = self.pop_next();

        match next {
            Some(e) => {
                #[cfg(test)]
                println!("Consumed event: {:?}", e);

                self.broadcast_event(e);
            }
            None => ()
        }
    }

    pub fn broadcast_event(&mut self, event: (EmRC<Ev, I>, Ev)) {
        #[cfg(test)]
        println!("Broadcast event: {:?}", event);

        for li in self.get_listeners() {
            if li.borrow().get_triggers().contains(&&event.1) {
                li.borrow().on_triggers(vec![&&event.1]);
            }
        }
    }

    pub fn broadcast_events(&mut self, events: Vec<(EmRC<Ev, I>, Ev)>) {
        for e in events {
            self.broadcast_event(e);
        }
    }
}