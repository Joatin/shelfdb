use slog::Logger;
use shelf_database::Database;
use failure::Error;
use std::sync::Arc;
use crate::admin::context::Context;
use hyper::service::{make_service_fn, service_fn};
use crate::admin::schema::Schema;
use crate::admin::query::Query;
use crate::admin::mutation::Mutation;
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
use std::time::Instant;

pub fn start_admin_server(logger: &Logger, config: &Config, store: Arc<Database>) -> Result<impl FnOnce(), Error> {
    let logger = logger.clone();
    let config = config.clone();
    let stop_logger = logger.clone();
    let (tx, rx) = channel::<()>();

    debug!(logger, "Setting up admin server resources");

    let handle = thread::spawn(move || {
        let mut runtime = Runtime::new().unwrap();
        debug!(logger, "Admin server thread and thread pool created");

        let _res: Result<(), Error> = runtime.block_on(async move {
            info!(logger, "Starting admin web server");

            let addr = config.admin_host()?;

            let context = Context::new(&logger, (*store).clone());
            let make_svc = make_service_fn(move |_conn| {

                let root_node = Arc::new(Schema::new(Query::new(), Mutation::new()));
                let new_context = context.clone();
                async move {
                    // service_fn converts our function into a `Service`
                    Ok::<_, Infallible>(service_fn(move |r| {
                        handle_routes(Arc::clone(&root_node), new_context.new_request(), r)
                    }))
                }
            });


            let server = HyperServer::bind(&addr).serve(make_svc).with_graceful_shutdown(async {
                rx.await.ok();
            });

            info!(logger, "Admin server listening on {}", format!("http://{}", addr).underline().bold().blue());
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }

            Ok(())
        });

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
    context: Context,
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    let start_time = Instant::now();

    info!(context.logger, "Received request {}", format!("{} {}", req.method(), req.uri().path()).yellow());

    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>  playground("/graphql"),
        (&Method::GET, "/graphql") =>  graphql_get(Arc::clone(&root_node), context.clone()).await,
        (&Method::POST, "/graphql") =>  graphql_post(Arc::clone(&root_node), context.clone(), req).await,
        _ => Ok(Response::new("Hello, World".into()))
    };

    info!(context.logger, "Handled request in {}", format!("{:?}ms", Instant::now().duration_since(start_time).as_secs_f64() * 1000.0).yellow());

    res
}
