use std::{fmt::Debug, rc::Rc, cell::RefCell};
use crate::prelude::*;
use crate::{IDCOUNTER, event::Event, sub_event_handler::SubEventHandler};

pub trait Id = PartialEq + Debug + Copy + Clone;

#[derive(Clone)]
pub struct EventHandler<T: Tag, I: Id> {
    id: usize,
    stack: Vec<Event<T, I>>,
    prev_event: Option<Event<T, I>>,
    listeners: Vec<LiRC<T, I>>,
}

impl<T: Tag, I: Id> Debug for EventHandler<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_event_str = if self.prev_event.is_some() { &Some((self.prev_event.as_ref().unwrap().get_emitter().borrow().get_id(), self.prev_event.as_ref().unwrap().get_tag())) } else { &None as &Option<(I, Option<T>)> };
        f.debug_struct("EventHandler")
            .field("id", &self.id)
            .field("stack", &self.stack.iter().map(|e| (e.get_emitter().borrow().get_id(), e.get_tag())).collect::<Vec<(I, Option<T>)>>())
            .field("prev_event", prev_event_str)
            .field("listeners", &self.listeners.iter().map(|l| l.borrow().get_id()).collect::<Vec<I>>())
            .finish()
    }
}

pub type EHRc<T, I> = Rc<RefCell<EventHandler<T, I>>>;

impl<T: Tag, I: Id> PartialEq for EventHandler<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> PartialEq<SubEventHandler<'a, P, T, I>> for EventHandler<T, I> {
    fn eq(&self, other: &SubEventHandler<P, T, I>) -> bool {
        self.id == other.get_id()
    }
}

impl<T: Tag, I: Id> Into<EHRc<T, I>> for EventHandler<T, I> {
    fn into(self) -> EHRc<T, I> {
        use std::{rc::Rc, cell::RefCell};
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag, I: Id> EventHandler<T, I> {
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
    pub fn push_event(&mut self, event: Option<Event<T, I>>) {
        if let Some(e) = event {
            #[cfg(test)]
            println!("Event pushed: (Emitter id: {:?}, Event: {:?})", e.get_emitter().borrow().get_id(), e.get_tag());
    
            self.stack.push(e);
        }
    }
    pub fn push_events(&mut self, events: Option<Vec<Event<T, I>>>) {
        if let Some(events) = events {
            for event in events {
                self.push_event(Some(event));
            }
        }
    }
    pub fn get_stack(&self) -> &Vec<Event<T, I>> {
        &self.stack
    }
    pub fn get_stack_len(&self) -> usize {
        self.stack.len()
    }
    pub fn get_stack_tags(&self) -> Vec<Option<T>> {
        self.get_stack().iter().map(|e| e.get_tag()).collect()
    }
    pub fn get_stack_emitters(&self) -> Vec<EmRC<T, I>> {
        self.get_stack().iter().map(|e| e.get_emitter().clone()).collect()
    }
    pub fn add_listener(&mut self, listener: LiRC<T, I>) {
        self.listeners.push(listener)
    }
    pub fn get_listeners(&self) -> &Vec<LiRC<T, I>> {
        &self.listeners
    }
    pub fn peek_next(&self) -> Option<&Event<T, I>> {
        #[cfg(test)]
        {
            if let Some(e) = self.stack.last() {
                println!("Event peeked: (Emitter id: {:?}, Event: {:?})", e.get_emitter().borrow().get_id(), e.get_tag());
            } else {
                println!("Event peeked: None");
            }
        }

        self.stack.last()
    }
    pub fn peek_next_tag(&self) -> Option<T> {
        if let Some(e) = self.peek_next() {
            e.get_tag()
        } else {
            None
        }
    }
    pub fn peek_next_emitter(&self) -> Option<EmRC<T, I>> {
        if let Some(e) = self.peek_next() {
            Some(e.get_emitter())
        } else {
            None
        }
    }
    pub fn pop_next(&mut self) -> Option<Event<T, I>> {
        if let Some(ret) = self.stack.pop() {
            self.prev_event = Some(ret.clone());

            #[cfg(test)]
            println!("Event popped: (Emitter: {:?}, Event: {:?})", ret.get_emitter().borrow().get_id(), ret.get_tag());

            Some(ret)
        } else {
            None
        }
    }
    pub fn get_prev_event(&self) -> Option<&Event<T, I>> {
        self.prev_event.as_ref()
    }
    pub fn emit(&mut self, emitter: EmRC<T, I>, tag: T) {
        self.push_event(Some(Event::new(emitter.clone(), Some(tag))));
    }
    pub fn consume_next_event(&mut self) {
        if let Some(next) = self.pop_next() {        
            #[cfg(test)]
            println!("Consumed event: {:?}", next);

            self.broadcast_event(next);
        }
    }
    pub fn broadcast_event(&mut self, event: Event<T, I>) {
        #[cfg(test)]
        println!("Broadcast event: {:?}", event);

        for li in self.get_listeners() {
            if li.borrow().get_trigger_tags().contains(&&event.get_tag().expect("Untagged")) {
                li.borrow().on_triggers(vec![event.clone()]);
            }
        }
    }
    pub fn broadcast_events(&mut self, events: Vec<Event<T, I>>) {
        for e in events {
            self.broadcast_event(e);
        }
    }
}