use crate::server::client::query::Query;
use juniper::{DefaultScalarValue};
use crate::server::client::mutation::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation, DefaultScalarValue>;