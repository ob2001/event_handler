use crate::prelude::*;

pub trait EHParent<Ev: Event> { fn notify_parent(&self, event: &(EmRC<Ev>, Ev)); }