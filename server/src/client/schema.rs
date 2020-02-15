use crate::client::mutation::Mutation;
use crate::client::query::Query;
use juniper::RootNode;

pub type Schema<'a, C, S> = RootNode<'a, Query<C, S>, Mutation<C, S>>;