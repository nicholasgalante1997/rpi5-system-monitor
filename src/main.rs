use actix_cors::Cors;
use actix_files;
use actix_web::{
    App, HttpServer,
    dev::{Service, ServiceRequest, ServiceResponse, fn_service},
    http, middleware, web,
};
use debugrs::debug;
use sysinfo::{Components, Disks, Networks, System};

use std::sync::{Arc, Mutex};

mod app;
mod env;
mod log;
mod models;
mod rdf;
mod services;
mod ui;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::setup_env();

    let components = Components::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    let mut system = System::new_all();

    system.refresh_all();
    networks.refresh(true);

    let app_state = app::AppState {
        components: Arc::new(Mutex::new(components)),
        disks: Arc::new(Mutex::new(disks)),
        networks: Arc::new(Mutex::new(networks)),
        system: Arc::new(Mutex::new(system)),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["HEAD", "OPTIONS", "GET"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::ACCEPT_ENCODING])
            .max_age(30);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
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
                web::scope("/api")
                    .wrap(middleware::Compress::default())
                    .configure(|config| {
                        // HTML Routes
                        config.service(services::http_views::routes::render_overview_as_html);
                        config.service(services::http_views::routes::render_cpu_as_html);
                        config.service(services::http_views::routes::render_memory_as_html);
                        config.service(services::http_views::routes::render_disks_as_html);
                        config.service(services::http_views::routes::render_components_as_html);
                        config.service(services::http_views::routes::render_networks_as_html);

                        // JSON Routes
                        config.service(services::json_views::routes::render_overview_as_json);
                        config.service(services::json_views::routes::render_system_as_json);
                        config.service(services::json_views::routes::render_cpus_as_json);
                        config.service(services::json_views::routes::render_disks_as_json);
                        config.service(services::json_views::routes::render_components_as_json);
                        config.service(services::json_views::routes::render_networks_as_json);

                        // RDF Routes
                    }),
            )
            .service(
                web::scope("")
                    .wrap(middleware::Compress::default())
                    .configure(|config| {
                        config.service(
                            actix_files::Files::new("", "./public")
                                .use_last_modified(true)
                                .use_etag(true)
                                .prefer_utf8(true)
                                .index_file("index.html")
                                .redirect_to_slash_directory()
                                .show_files_listing()
                                .default_handler(fn_service(|req: ServiceRequest| async {
                                    let (req, _) = req.into_parts();
                                    let file = actix_files::NamedFile::open("public/index.html")?;
                                    let res = file.into_response(&req);
                                    Ok(ServiceResponse::new(req, res))
                                })),
                        );
                    }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
