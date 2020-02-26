use crate::context::Context;
use shelf_database::{
    Cache,
    Store,
};
use std::marker::PhantomData;

pub struct Mutation<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Mutation<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData,
        }
    }
}

#[juniper::graphql_object(Context = Context<C, S>)]
impl<C: Cache, S: Store> Mutation<C, S> {
    fn id() -> bool {
        true
    }
}
