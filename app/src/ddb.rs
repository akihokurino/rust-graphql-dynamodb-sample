use std::marker::PhantomData;

pub mod stock;

#[derive(Clone)]
pub struct Dao<T> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub fn new() -> Self {
        Dao {
            _phantom: PhantomData,
        }
    }
}
