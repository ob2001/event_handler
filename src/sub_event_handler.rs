use crate::{prelude::*, event::Event};
use crate::{IDCOUNTER, event_handler::EventHandler};

// Event handler reporting to a parent object
#[derive(Clone)]
pub struct SubEventHandler<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> {
    id: usize,
    stack: Vec<Event<T, I>>,
    prev_event: Option<Event<T, I>>,
    listeners: Vec<LiRC<T, I>>,
    parents: Vec<&'a P>,
}

impl<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> Debug for SubEventHandler<'a, P, T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_event_str = if self.prev_event.is_some() { &Some((self.prev_event.as_ref().unwrap().get_emitter().borrow().get_id(), self.prev_event.as_ref().unwrap().get_tag())) } else { &None as &Option<(I, Option<T>)> };

        f.debug_struct("SubEventHandler")
            .field("id", &self.id)
            .field("stack", &self.stack.iter().map(|e| (e.get_emitter().borrow().get_id(), e.get_tag())).collect::<Vec<(I, Option<T>)>>())
            .field("prev_event", prev_event_str)
            .field("listeners", &self.listeners.iter().map(|l| l.borrow().get_id()).collect::<Vec<I>>())
            .finish()
    }
}

impl<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> PartialEq for SubEventHandler<'a, P, T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> PartialEq<EventHandler<T, I>> for SubEventHandler<'a, P, T, I> {
    fn eq(&self, other: &EventHandler<T, I>) -> bool {
        self.id == other.get_id()
    }
}

impl<'a, P: EHParent<T, I> + Debug, T: Tag, I: Id> SubEventHandler<'a, P, T, I> {
    pub fn new(parents: Vec<&'a P>) -> Self {
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
    pub fn push_event(&mut self, event: Option<Event<T, I>>) {
        match event {
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Event pushed to stack: {:?}", e);

                self.stack.push(e)
            },
            _ => {}
        }
    }
    pub fn push_events(&mut self, events: Option<Vec<Event<T, I>>>) {
        match events {
            None => {}
            Some(e) => {
                #[cfg(debug_assertions)]
                println!("Events pushed to stack: {:?}", e);

                self.stack.extend(e);
            }
        }
    }
    pub fn get_stack(&self) -> &Vec<Event<T, I>> {
        &self.stack
    }
    pub fn get_stack_events(&self) -> Vec<T> {
        self.get_stack().into_iter().map(|e| e.get_tag().expect("Untagged event")).collect()
    }
    pub fn get_stack_emitters(&self) -> Vec<UqRC<I>> {
        self.get_stack().into_iter().map(|e| e.get_emitter()).collect()
    }
    pub fn add_listener(&mut self, listener: LiRC<T, I>) {
        self.listeners.push(listener)
    }
    pub fn get_listeners(&self) -> &Vec<LiRC<T, I>> {
        &self.listeners
    }
    pub fn peek_next(&self) -> Option<&Event<T, I>> {
        #[cfg(debug_assertions)]
        println!("Event peeked: {:?}", self.stack.first());

        self.stack.first()
    }
    pub fn peek_next_tag(&self) -> Option<T> {
        if let Some(e) = self.peek_next() {
            e.get_tag()
        } else {
            None
        }
    }
    pub fn peek_next_emitter(&self) -> Option<UqRC<I>> {
        if let Some(e) = self.peek_next() {
            Some(e.get_emitter())
        } else {
            None
        }
    }
    pub fn pop_next(&mut self) -> Option<Event<T, I>> {
        let ret = self.stack.pop();
        #[cfg(debug_assertions)]
        println!("Event popped: {:?}", ret);

        self.prev_event = ret.clone();
        ret
    }
    pub fn get_prev_event(&self) -> &Option<Event<T, I>> {
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
    pub fn broadcast_event(&mut self, event: Event<T, I>) {
        #[cfg(debug_assertions)]
        println!("Broadcast event: {:?}", event);

        for li in self.get_listeners() {
            if li.borrow().get_triggers().contains(&&event.get_tag().expect("Untagged event")) {
                li.borrow().on_triggers(vec![event.clone()]);
            }
        }

        for &p in &self.parents {
            p.notify_parent(&event);
        }
    }
    pub fn broadcast_events(&mut self, events: Vec<Event<T, I>>) {
        for e in events {
            self.broadcast_event(e);
        }
    }
}