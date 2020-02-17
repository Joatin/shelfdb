use failure::Error;
use slog::Logger;
use shelf_database::{Database, Cache, Store};
use crate::client::{ Schema as ClientSchema };
use crate::admin::{ Schema as AdminSchema, Query as AdminQuery, Mutation as AdminMutation };
use std::sync::{Arc, RwLock};
use shelf_config::Config;
use std::collections::HashMap;
use crate::context::Context;
use hyper::{Request, Body, Response, Method, Server as HyperServer};
use std::convert::Infallible;
use std::time::Instant;
use hyper::service::{make_service_fn, service_fn};
use colored::*;
use crate::util::graphql_post;
use crate::util::graphql_get;
use crate::util::playground;
use crate::client::build_root_node_from_schemas;
use futures::channel::oneshot::channel;


pub struct Server {
    handle: Box<dyn FnOnce()>
}

impl Server {
    pub async fn start<C: Cache, S: Store>(logger: &Logger, config: &Config, db: Database<C, S>) -> Result<Self, Error> {
        let logger = logger.clone();
        let stop_logger = logger.clone();
        let config = config.clone();

        let db = Arc::new(RwLock::new(db));

        let (tx, rx) = channel::<()>();

        tokio::spawn(async move {
            info!(logger, "Starting web server 🥳");
            let addr = config.client_host()?;
            let other_logger = logger.clone();

            let admin_root_node = Self::build_admin_root_node();
            let client_root_nodes = Self::build_client_root_nodes(&db);

            let db = Arc::clone(&db);

            let make_svc = make_service_fn(move |_conn| {
                let client_root_nodes = Arc::clone(&client_root_nodes);
                let admin_root_node = Arc::clone(&admin_root_node);
                let context = Context::<C, S>::new(&other_logger, Arc::clone(&db));
                async move {
                    Ok::<_, Infallible>(service_fn(move |r| {
                        let lock = client_root_nodes.read().unwrap();
                        Self::map_route(lock.clone(), Arc::clone(&admin_root_node), context.new_request(), r)
                    }))
                }
            });


            let server = HyperServer::bind(&addr).serve(make_svc).with_graceful_shutdown(async {
                rx.await.ok();
            });

            info!(logger, "Server listening on {} 🔗", format!("http://{}", addr).underline().bold().blue());
            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }

            Result::<(), Error>::Ok(())
        });

        Ok(Self {
            handle: Box::new(move || {
                info!(stop_logger, "Shutting down client server");
                match tx.send(()) {
                    Ok(_) => {
                        info!(stop_logger, "Server closed");
                    },
                    Err(_) => {
                        warn!(stop_logger, "Weird, server was already closed...");
                    }
                }
            })
        })
    }

    fn build_admin_root_node<C: Cache, S: Store>() -> Arc<AdminSchema<C, S>> {
        Arc::new(AdminSchema::new(AdminQuery::new(), AdminMutation::new()))
    }

    fn build_client_root_nodes<C: Cache, S: Store>(db: &Arc<RwLock<Database<C, S>>>) -> Arc<RwLock<HashMap<String, Arc<ClientSchema<'static, C, S>>>>> {
        let other_db2 = Arc::clone(&db);
        let nodes = {
            let lock = other_db2.read().unwrap();
            build_root_node_from_schemas(lock.schemas())
        };

        let rw_nodes = Arc::new(RwLock::new(nodes));
        let root_nodes = Arc::clone(&rw_nodes);
        let other_db = Arc::clone(&db);

        tokio::spawn(async move {
            let mut recv = {
                let lock = other_db.read().unwrap();
                lock.on_schema_updates()
            };
            while let Ok(_) = recv.recv().await {
                let mut lock = root_nodes.write().unwrap();
                let db_lock = other_db.read().unwrap();
                *lock = build_root_node_from_schemas(db_lock.schemas());
            }
        });

        rw_nodes
    }

    async fn map_route<C: Cache, S: Store>(
        client_root_nodes: HashMap<String, Arc<ClientSchema<'_, C, S>>>,
        admin_root_node: Arc<AdminSchema<C, S>>,
        context: Context<C, S>,
        req: Request<Body>
    ) -> Result<Response<Body>, Infallible> {
        let start_time = Instant::now();

        let logger = context.logger.clone();
        let method_and_uri = format!("{} {}", req.method(), req.uri().path()).yellow();

        debug!(logger, "Received request {}", method_and_uri);

        if req.uri().path().starts_with(&"/admin") {
            return match (req.method(), req.uri().path()) {
                (&Method::GET, "/") =>  playground("/graphql"),
                (&Method::GET, "/graphql") =>  graphql_get(Arc::clone(&admin_root_node), context).await,
                (&Method::POST, "/graphql") =>  graphql_post(Arc::clone(&admin_root_node), context, req).await,
                _ => Ok(Response::new("Hello, World".into()))
            };
        }

        let res = match client_root_nodes.iter().find(|(key, _)| req.uri().path().starts_with(&format!("/{}", key))) {
            Some((key, node)) => {
                let prefix = format!("/{}", key);

                match (req.method(), &*req.uri().path().replace(&prefix, "")) {
                    (&Method::GET, "") =>  playground(&format!("/{}/graphql", key)),
                    (&Method::GET, "/") =>  playground(&format!("/{}/graphql", key)),
                    (&Method::GET, "/graphql") =>  graphql_get(Arc::clone(&node), context).await,
                    (&Method::POST, "/graphql") =>  graphql_post(Arc::clone(&node), context, req).await,
                    _ => Ok(Response::new("Hello, World".into()))
                }
            },
            None => {
                match client_root_nodes.get("shelf") {
                    Some(node) => {
                        match (req.method(), req.uri().path()) {
                            (&Method::GET, "") =>  playground("/graphql"),
                            (&Method::GET, "/") =>  playground("/graphql"),
                            (&Method::GET, "/graphql") =>  graphql_get(Arc::clone(&node), context).await,
                            (&Method::POST, "/graphql") =>  graphql_post(Arc::clone(&node), context, req).await,
                            _ => Ok(Response::new("Hello, World".into()))
                        }
                    },
                    None => Ok(Response::new("Hello, World".into())),
                }
            },
        };

        info!(logger, "Handled request {} in {}", method_and_uri, format!("{:?}ms", Instant::now().duration_since(start_time).as_secs_f64() * 1000.0).yellow());

        res
    }

    pub fn stop(self) {
        (self.handle)();
    }
}




