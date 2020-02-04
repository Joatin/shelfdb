use slog::Logger;
use shelf_database::Database;
use failure::Error;
use std::sync::Arc;
use crate::client::context::Context;
use hyper::service::{make_service_fn, service_fn};
use crate::client::schema::Schema;
use crate::client::query::Query;
use crate::client::mutation::Mutation;
use std::convert::Infallible;
use hyper::{Request, Response, Body, Method, Server as HyperServer };
use std::thread;
use tokio::runtime::Runtime;
use crate::util::graphql_post;
use crate::util::graphql_get;
use crate::util::playground;
use tokio::sync::oneshot::channel;
use shelf_config::Config;
use colored::*;

pub fn start_client_server(logger: &Logger, config: &Config, store: Arc<Database>) -> Result<impl FnOnce(), Error> {
    let logger = logger.clone();
    let config = config.clone();
    let stop_logger = logger.clone();
    let (tx, rx) = channel::<()>();

    debug!(logger, "Setting up client server resources");

    let handle = thread::spawn(move || {
        let mut runtime = Runtime::new().unwrap();
        debug!(logger, "Client server thread and thread pool created");

        let _res: Result<(), Error> = runtime.block_on(async move {
            info!(logger, "Starting client web server");

            let addr = config.client_host()?;

            let context = Arc::new(Context::new(&logger, store));
            let make_svc = make_service_fn(move |_conn| {

                let root_node = Arc::new(Schema::new(Query::new(), Mutation::new()));
                let new_context = Arc::clone(&context);
                async move {
                    // service_fn converts our function into a `Service`
                    Ok::<_, Infallible>(service_fn(move |r| {
                        handle_routes(Arc::clone(&root_node), Arc::clone(&new_context), r)
                    }))
                }
            });


            let server = HyperServer::bind(&addr).serve(make_svc).with_graceful_shutdown(async {
                rx.await.ok();
            });

            info!(logger, "Client server listening on {}", format!("http://{}", addr).underline().bold().blue());
            if let Err(e) = server.await {
                eprintln!("Client server error: {}", e);
            }

            Ok(())
        });

    });

    Ok(move || {
        info!(stop_logger, "Shutting down client server");
        match tx.send(()) {
            Ok(_) => {
                handle.join().expect("Failed to wait for client server to stop");
                info!(stop_logger, "Client server closed");
            },
            Err(_) => {
                warn!(stop_logger, "Weird, client server was already closed...");
            }
        }
    })
}

async fn handle_routes(
    root_node: Arc<Schema>,
    context: Arc<Context>,
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>  playground("/graphql"),
        (&Method::GET, "/graphql") =>  graphql_get(Arc::clone(&root_node), context).await,
        (&Method::POST, "/graphql") =>  graphql_post(Arc::clone(&root_node), context, req).await,
        _ => Ok(Response::new("Hello, World".into()))
    }
}