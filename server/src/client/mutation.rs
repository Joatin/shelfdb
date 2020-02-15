use crate::context::Context;
use std::marker::PhantomData;
use shelf_database::{Cache, Store};


pub struct Mutation<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>
}

impl<C: Cache, S: Store> Mutation<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData
        }
    }
}

#[juniper::object(Context = Context<C, S>)]
impl<C: Cache, S: Store> Mutation<C, S> {
    fn id() -> bool {
        true
    }
}