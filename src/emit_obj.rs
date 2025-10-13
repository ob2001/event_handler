use crate::prelude::*;

pub trait Id = PartialEq + Debug + Clone;

///
pub trait EmitObj<I: Id> {
    fn get_id(&self) -> I;
}

impl<I: Id> PartialEq for dyn EmitObj<I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<T: Tag, I: Id> PartialEq<dyn IListener<T, I>> for dyn EmitObj<I> {
    fn eq(&self, other: &dyn IListener<T, I>) -> bool {
        self.get_id() == other.get_id()
    }
}

#[derive(Clone, Debug)]
pub struct EmRC<I: Id>(pub(crate) Rc<RefCell<dyn EmitObj<I>>>);

impl<I: Id> Deref for EmRC<I> {
    type Target = Rc<RefCell<dyn EmitObj<I>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: Id> PartialEq for EmRC<I> {
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl<T: Tag, I: Id> PartialEq<LiRC<T, I>> for EmRC<I> {
    fn eq(&self, other: &LiRC<T, I>) -> bool {
        self.borrow().get_id() == other.borrow().get_id()
    }
}

impl<I: Id> Debug for dyn EmitObj<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmitObj id: {:?}", self.get_id())
    }
}