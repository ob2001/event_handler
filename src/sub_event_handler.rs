use std::fmt::Debug;
use crate::prelude::*;
use crate::{IDCOUNTER, event_handler::EventHandler};

// Event handler reporting to a parent object
#[derive(Clone)]
pub struct SubEventHandler<'a, Id: PartialEq + Debug, T: EHParent<Ev> + Debug, Ev: Event> {
    id: usize,
    stack: Vec<(EmRC<Ev, Id>, Ev)>,
    prev_event: Option<(EmRC<Ev, Id>, Ev)>,
    listeners: Vec<LiRC<Ev, Id>>,
    parents: Vec<&'a T>,
}

impl<'a, Id: PartialEq + Debug, T: EHParent<Ev> + Debug, Ev: Event> Debug for SubEventHandler<'a, Id, T, Ev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_event_str = if self.prev_event.is_some() { &Some((self.prev_event.as_ref().unwrap().0.borrow().get_id(), self.prev_event.as_ref().unwrap().1)) } else { &None as &Option<(Id, Ev)> };

        f.debug_struct("SubEventHandler")
            .field("id", &self.id)
            .field("stack", &self.stack.iter().map(|e| (e.0.borrow().get_id(), e.1)).collect::<Vec<(Id, Ev)>>())
            .field("prev_event", prev_event_str)
            .field("listeners", &self.listeners.iter().map(|l| l.borrow().get_id()).collect::<Vec<Id>>())
            .finish()
    }
}

impl<'a, Id: PartialEq + Debug, T: EHParent<Ev> + Debug, Ev: Event> PartialEq for SubEventHandler<'a, Id, T, Ev> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, Id: PartialEq + Debug, T: EHParent<Ev> + Debug, Ev: Event> PartialEq<EventHandler<Ev>> for SubEventHandler<'a, Id, T, Ev> {
    fn eq(&self, other: &EventHandler<Ev>) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, Id: PartialEq + Debug, T: EHParent<Ev> + Debug, Ev: Event> SubEventHandler<'a, Id, T, Ev> {
    pub fn new(parents: Vec<&'a T>) -> Self {
        SubEventHandler {
            id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            stack: Vec::new(),
            prev_event: None,
            listeners: Vec::new(),
            parents
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn push_event(&mut self, event: Option<(EmRC<Ev, Id>, Ev)>) {
        match event {
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Event pushed to stack: {:?}", e);

                self.stack.push(e)
            },
            _ => {}
        }
    }

    pub fn push_events(&mut self, events: Option<Vec<(EmRC<Ev, Id>, Ev)>>) {
        match events {
            None => {}
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Events pushed to stack: {:?}", e);

                self.stack.extend(e);
            }
        }
    }

    pub fn get_stack(&self) -> &Vec<(EmRC<Ev, Id>, Ev)> {
        &self.stack
    }

    pub fn get_stack_events(&self) -> Vec<&Ev> {
        self.get_stack().into_iter().map(|e| &e.1).collect()
    }

    pub fn get_stack_emitters(&self) -> Vec<EmRC<Ev, Id>> {
        self.get_stack().into_iter().map(|e| e.0.clone()).collect()
    }

    pub fn add_listener(&mut self, listener: LiRC<Ev, Id>) {
        self.listeners.push(listener)
    }

    pub fn get_listeners(&self) -> &Vec<LiRC<Ev, Id>> {
        &self.listeners
    }

    pub fn peek_next(&self) -> Option<&(EmRC<Ev, Id>, Ev)> {
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

    pub fn peek_next_emitter(&self) -> Option<&EmRC<Ev, Id>> {
        if let Some((em, _)) = self.peek_next() {
            Some(em)
        } else {
            None
        }
    }

    pub fn pop_next(&mut self) -> Option<(EmRC<Ev, Id>, Ev)> {
        let ret = self.stack.pop();
        #[cfg(debug_assertions)]
        println!("Event popped: {:?}", ret);

        self.prev_event = ret.clone();
        ret
    }

    pub fn get_prev_event(&self) -> &Option<(EmRC<Ev, Id>, Ev)> {
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

    pub fn broadcast_event(&mut self, event: (EmRC<Ev, Id>, Ev)) {
        #[cfg(debug_assertions)]
        println!("Broadcast event: {:?}", event);

        for li in self.get_listeners() {
            if li.borrow().get_triggers().contains(&&event.1) {
                li.borrow().on_triggers(vec![&&event.1]);
            }
        }

        for &p in &self.parents {
            p.notify_parent(&event);
        }
    }

    pub fn broadcast_events(&mut self, events: Vec<(EmRC<Ev, Id>, Ev)>) {
        for e in events {
            self.broadcast_event(e);
        }
    }
}