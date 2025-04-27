use actix_web::{App, HttpServer, dev::Service, middleware, web};
use debugrs::debug;
use sysinfo::{Components, Disks, Networks, System};

use std::sync::{Arc, Mutex};

mod app;
mod env;
mod log;
mod models;
mod services;
mod ui;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Lood environment variables from .env into scope
    env::setup_env();

    let components = Components::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    let networks = Networks::new();
    let mut system = System::new_all();

    system.refresh_all();

    let app_state = app::AppState {
        components: Arc::new(Mutex::new(components)),
        disks: Arc::new(Mutex::new(disks)),
        networks: Arc::new(Mutex::new(networks)),
        system: Arc::new(Mutex::new(system)),
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
                web::scope("/health")
                    .wrap(middleware::Compress::default())
                    .configure(|config| {
                        services::health::configure_health_check_service(config);
                    }),
            )
            .service(
                web::scope("/")
                    .wrap(middleware::Compress::default())
                    .configure(|config| {
                        services::system::configure_system_monitor_service(config);
                    }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
