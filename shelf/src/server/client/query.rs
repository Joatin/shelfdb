use crate::server::client::context::Context;
use std::sync::Arc;


pub struct Query;

impl Query {
    pub fn new() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Arc<Context>)]
impl Query {

}