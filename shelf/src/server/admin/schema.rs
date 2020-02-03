use crate::server::admin::query::Query;
use juniper::{DefaultScalarValue};
use crate::server::admin::mutation::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation, DefaultScalarValue>;