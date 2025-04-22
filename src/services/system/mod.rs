use actix_web::{Error, HttpResponse, Responder, web};
use futures::{future::ok, stream::once};
use std::sync::Arc;

use crate::{app::AppState, models::system_report::SystemReporter, ui::Html};

pub async fn handler(app_state: web::Data<AppState>) -> impl Responder {
    let components = app_state.components.lock().unwrap();
    let disks = app_state.disks.lock().unwrap();
    let networks = app_state.networks.lock().unwrap();
    let mut system = app_state.system.lock().unwrap();

    let mut system_reporter = SystemReporter::new(&components, &disks, &networks, &mut system);
    let system_report = system_reporter.build_report();
    let html = Html::new(system_report.into_html());
    let page = html.into_page();

    let bytes = web::Bytes::from(page);
    let body = once(ok::<_, Error>(bytes));

    HttpResponse::Ok().content_type("text/html").streaming(body)
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
