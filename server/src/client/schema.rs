use crate::client::query::Query;
use juniper::{DefaultScalarValue};
use crate::client::mutation::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation, DefaultScalarValue>;