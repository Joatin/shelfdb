use crate::admin::mutation::Mutation;
use crate::admin::query::Query;
use juniper::DefaultScalarValue;

pub type Schema<C, S> = juniper::RootNode<'static, Query<C, S>, Mutation<C, S>, DefaultScalarValue>;
