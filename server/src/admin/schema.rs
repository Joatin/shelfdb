use crate::admin::query::Query;
use juniper::{DefaultScalarValue};
use crate::admin::mutation::Mutation;

pub type Schema<C, S> = juniper::RootNode<'static, Query<C, S>, Mutation<C, S>, DefaultScalarValue>;