use crate::{EmCRc, LiCRc, Event};

pub struct EventHandler<Ev: Event> {
    stack: Vec<Ev>,
    prev_event: Option<Ev>,
    emitters: Vec<EmCRc<Ev>>,
    listeners: Vec<LiCRc<Ev>>,
}

impl<Ev: Event> EventHandler<Ev> {
    pub fn new(stack: Vec<Ev>, emitters: Vec<EmCRc<Ev>>, listeners: Vec<LiCRc<Ev>>) -> Self {
        EventHandler { 
            stack, 
            prev_event: None, 
            emitters,
            listeners, 
        }
    }

    pub fn register_emitter(&mut self, emitter: EmCRc<Ev>) {
        self.emitters.push(emitter);
    }

    pub fn register_listener(&mut self, listener: LiCRc<Ev>) {
        self.listeners.push(listener);
    }

    pub fn register_emitters(&mut self, emitters: Vec<EmCRc<Ev>>) {
        self.emitters.extend(emitters);
    }

    pub fn register_listeners(&mut self, listeners: Vec<LiCRc<Ev>>) {
        self.listeners.extend(listeners);
    }

    pub fn push_event(&mut self, event: Option<Ev>) {
        match event {
            Some(e) => self.stack.push(e),
            _ => {}
        }
    }

    pub fn push_events(&mut self, events: Option<Vec<Ev>>) {
        match events {
            None => {}
            Some(e) => {
                self.stack.extend(e);
            }
        }
    }

    pub fn get_stack(&self) -> &Vec<Ev> {
        &self.stack
    }

    pub fn get_emitters(&self) -> &Vec<EmCRc<Ev>> {
        &self.emitters
    }

    pub fn get_listeners(&self) -> &Vec<LiCRc<Ev>> {
        &self.listeners
    }

    pub fn peek_next_event(&self) -> Option<&Ev> {
        self.stack.first()
    }

    pub fn pop_next_event(&mut self) -> Option<Ev> {
        let ret = self.stack.pop();
        self.prev_event = ret.clone();
        ret
    }

    pub fn get_prev_event(&self) -> &Option<Ev> {
        &self.prev_event
    }

    pub fn broadcast(&mut self, events: Vec<Ev>) {
        for i in 0..self.listeners.len() {
            let intersection: Vec<&Ev> = events.iter().filter(|e| self.listeners[i].borrow().triggers().contains(e)).collect();
            let new_events = self.listeners[i].borrow().on_triggers(intersection);
            self.push_events(new_events);
        }
    }
}