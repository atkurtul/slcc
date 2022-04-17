pub use std::{cell::RefCell, rc::Rc};
pub type Map<K, V> = fxhash::FxHashMap<K, V>;
pub type Set<K> = fxhash::FxHashSet<K>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Mrc<T>(RefCell<T>);

impl<T> Mrc<T> {
    pub fn new(t: T) -> Self {
        Self(RefCell::new(t))
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Mrc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl<T> std::ops::Deref for Mrc<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
