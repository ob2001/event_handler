use crate::{prelude::*};
use crate::IDCOUNTER;

#[derive(Debug, Clone, PartialEq)]
pub struct DefEmitter<T: Tag> {
    id: usize,
    def_tag: Option<T>,
}

impl<T: Tag> Into<UqRC<usize>> for DefEmitter<T> {
    fn into(self) -> UqRC<usize> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Tag> EmitObj<usize> for DefEmitter<T> {
    fn get_id(&self) -> usize {
        self.id
    }
    fn as_uqrc(&self) -> UqRC<usize> {
        self.clone().into()
    }
}

impl<T: Tag> DefEmitter<T> {
    pub fn new(def_tag: Option<T>) -> Self {
        Self { id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag }
    }
    pub fn new_emrc(def_tag: Option<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { id: IDCOUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst), def_tag}))
    }
}