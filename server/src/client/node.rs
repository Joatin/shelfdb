use juniper::{GraphQLType, DefaultScalarValue, Registry};
use juniper::meta::MetaType;
use crate::context::Context;
use uuid::Uuid;
use shelf_database::{Cache, Store};
use failure::_core::marker::PhantomData;

pub struct Node<C: Cache, S: Store> {
    phantom_cache: PhantomData<C>,
    phantom_store: PhantomData<S>,
}

impl<C: Cache, S: Store> Node<C, S> {
    pub fn new() -> Self {
        Self {
            phantom_cache: PhantomData,
            phantom_store: PhantomData
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
        where DefaultScalarValue: 'r
    {
        let fields = vec![
            registry.field::<&Uuid>("id", &())
        ];


        registry.build_interface_type::<Node<C, S>>(&info, &fields).into_meta()
    }
}