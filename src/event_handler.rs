use crate::{Emitter, Listener};

pub struct EventHandler<Ev: PartialEq + Copy + 'static> {
    stack: Vec<Ev>,
    emitters: Vec<Box<dyn Emitter<Ev>>>,
    listeners: Vec<Box<dyn Listener<Ev>>>,
}

impl<Ev: PartialEq + Copy> EventHandler<Ev> {
    pub fn new(stack: Option<Vec<Ev>>, emitters: Option<Vec<Box<dyn Emitter<Ev>>>>, listeners: Option<Vec<Box<dyn Listener<Ev>>>>) -> Self {
        EventHandler { stack: stack.unwrap_or_default(), emitters: emitters.unwrap_or_default(), listeners: listeners.unwrap_or_default() }
    }

    pub fn register_emitter(&mut self, emitter: Box<dyn Emitter<Ev> + 'static>) {
        self.emitters.push(emitter);
    }

    pub fn register_listener(&mut self, listener: Box<dyn Listener<Ev> + 'static>) {
        self.listeners.push(listener);
    }

    pub fn register_emitters(&mut self, emitters: Vec<Box<dyn Emitter<Ev>>>) {
        self.emitters.extend(emitters);
    }

    pub fn register_listeners(&mut self, listeners: Vec<Box<dyn Listener<Ev>>>) {
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

    pub fn get_emitters(&self) -> &Vec<Box<dyn Emitter<Ev>>> {
        &self.emitters
    }

    pub fn get_listeners(&self) -> &Vec<Box<dyn Listener<Ev>>> {
        &self.listeners
    }

    pub fn peek_next_event(&self) -> Option<&Ev> {
        self.stack.first()
    }

    pub fn pop_next_event(&mut self) -> Option<Ev> {
        self.stack.pop()
    }

    pub fn broadcast(&mut self, events: Vec<Ev>) {
        for i in 0..self.listeners.len() {
            let intersection: Vec<&Ev> = events.iter().filter(|e| self.listeners[i].triggers().contains(e)).collect();
            self.push_events(self.listeners[i].on_triggers(intersection).clone());
        }
    }
}