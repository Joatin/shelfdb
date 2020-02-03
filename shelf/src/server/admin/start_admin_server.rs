use slog::Logger;
use crate::database::Database;
use failure::Error;
use std::sync::Arc;
use crate::server::admin::context::Context;
use hyper::service::{make_service_fn, service_fn};
use crate::server::admin::schema::Schema;
use crate::server::admin::query::Query;
use crate::server::admin::mutation::Mutation;
use std::convert::Infallible;
use hyper::{Request, Response, Body, Method, Server as HyperServer };
use std::thread;
use tokio::runtime::Runtime;
use crate::server::util::graphql_post;
use crate::server::util::graphql_get;
use crate::server::util::playground;
use tokio::sync::oneshot::channel;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;

pub fn start_admin_server(logger: &Logger, store: Arc<Database>, port: u16) -> Result<impl FnOnce(), Error> {
    let logger = logger.clone();
    let stop_logger = logger.clone();
    let (tx, rx) = channel::<()>();

    debug!(logger, "Setting up admin server resources");

    let handle = thread::spawn(move || {
        let mut runtime = Runtime::new().unwrap();
        debug!(logger, "Admin server thread and thread pool created");

        runtime.block_on(async move {
            info!(logger, "Starting admin web server");

            let addr = ([127, 0, 0, 1], port).into();

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
                println!("STOPPING");
            });

            info!(logger, "Admin server listening on http://{}", addr);
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        })

    });

    Ok(move || {
        info!(stop_logger, "Shutting down admin server");
        match tx.send(()) {
            Ok(_) => {
                handle.join().expect("Failed to wait for admin server to stop");
                info!(stop_logger, "Admin server closed");
            },
            Err(_) => {
                warn!(stop_logger, "Weird, admin server was already closed...");
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
