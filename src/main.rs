use actix_web::{dev::Service, web, App, HttpServer, middleware};
use debugrs::debug;
use sysinfo::System;

use std::sync::{Arc, Mutex};

mod app;
mod env;
mod log;
mod models;
mod services;
mod ui;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::setup_env();

    let mut system = System::new_all();

    system.refresh_all();

    let app_state = app::AppState {
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
            .service(
                web::scope("")
                .wrap(middleware::Compress::default())
                .configure(|config| {
                    services::health::configure_health_check_service(config);
                })
            )
            .service(
                web::scope("")
                    .wrap(middleware::Compress::default())
                    .configure(|config| {
                        services::health::configure_health_check_service(config);
                    }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}