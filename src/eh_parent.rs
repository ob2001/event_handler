use crate::{prelude::*, event::Event};

pub trait EHParent<T: Tag, I: Id> { fn notify_parent(&self, event: Event<T, I>); }