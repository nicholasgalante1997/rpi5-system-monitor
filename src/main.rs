use actix_web::{dev::Service, web, App, HttpServer};
use debugrs::debug;
use sysinfo::System;

use std::sync::{Arc, Mutex};

mod env;
mod log;
mod models;
mod services;

#[derive(Clone)]
struct AppState {
    system: Arc<Mutex<System>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::setup_env();

    let mut system = System::new_all();

    system.refresh_all();

    let app_state = AppState {
        system: Arc::new(Mutex::new(system))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap_fn(|req, srv| {
                let logline = format!("{} {}", req.method(), req.path());
                debug!(log::logger(), logline);
                
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    Ok(res)
                }
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}