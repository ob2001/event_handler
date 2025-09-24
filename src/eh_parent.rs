use crate::{Event, EmRC};

pub trait EHParent<Ev: Event> { fn notify(&self, event: &(EmRC<Ev>, Ev)); }