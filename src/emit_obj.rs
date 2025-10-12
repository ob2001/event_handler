use crate::prelude::*;

pub trait Id = PartialEq + Debug + Clone;

///
pub trait EmitObj<I: Id> {
    fn get_id(&self) -> I;
    fn as_uqrc(&self) -> UqRC<I>;
}

pub type UqRC<I> = Rc<RefCell<dyn EmitObj<I>>>;

impl<I: Id> PartialEq for dyn EmitObj<I> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<I: Id> Debug for dyn EmitObj<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmitObj id: {:?}", self.get_id())
    }
}