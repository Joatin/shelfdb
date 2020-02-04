use crate::admin::query::Query;
use juniper::{DefaultScalarValue};
use crate::admin::mutation::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation, DefaultScalarValue>;