use crate::context::Context;
use failure::_core::marker::PhantomData;
use juniper::meta::MetaType;
use juniper::{DefaultScalarValue, GraphQLType, GraphQLTypeAsync, Registry};
use shelf_database::{Cache, Store};
use uuid::Uuid;

pub struct Node<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Node<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData,
        }
    }
}

impl<C: Cache, S: Store> GraphQLType for Node<C, S> {
    type Context = Context<C, S>;
    type TypeInfo = ();

    fn name(_info: &Self::TypeInfo) -> Option<&str> {
        Some("Node")
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r>
    where
        DefaultScalarValue: 'r,
    {
        let fields = vec![registry.field::<&Uuid>("id", &())];

        registry
            .build_interface_type::<Node<C, S>>(&info, &fields)
            .into_meta()
    }
}

impl<C: Cache, S: Store> GraphQLTypeAsync<DefaultScalarValue> for Node<C, S> {}
