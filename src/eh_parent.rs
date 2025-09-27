use crate::prelude::*;

pub trait EHParent<Ev: Event, I: Id> { fn notify_parent(&self, event: &(EmRC<Ev, I>, Ev)); }