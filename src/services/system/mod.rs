use actix_web::{web, HttpResponse, Responder};
use futures::{future::ok, stream::once};
use crate::{app::AppState, models::system_report::SystemReporter, ui::Html};

pub async fn handler(app_state: web::Data<AppState>) -> impl Responder {
    let components_guard = app_state.components.lock().unwrap();
    let components = components_guard.as_ref();

    let disks_guard = app_state.disks.lock().unwrap();
    let disks = disks_guard.as_ref();
    
    let networks = app_state.networks.get_mut().unwrap();
    let system = app_state.system.get_mut().unwrap();
    let mut system_reporter = SystemReporter::new(components, disks, networks, system);
    let system_report = system_reporter.build_report();
    let html = Html::new(system_report.into_html());
    HttpResponse::Ok().body(html.into_page())
}


pub fn configure_system_monitor_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(handler))
            .route(web::post().to(HttpResponse::MethodNotAllowed))
            .route(web::put().to(HttpResponse::MethodNotAllowed))
            .route(web::patch().to(HttpResponse::MethodNotAllowed))
            .route(web::delete().to(HttpResponse::MethodNotAllowed)),
    );
}