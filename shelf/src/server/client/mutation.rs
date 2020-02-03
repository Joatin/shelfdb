use crate::server::client::context::Context;
use std::sync::Arc;


pub struct Mutation;

impl Mutation {
    pub fn new() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Arc<Context>)]
impl Mutation {

}