use actix_web::{web, HttpResponse, Responder};
use futures::{future::ok, stream::once};
use crate::ui::Html;

pub async fn handler() -> impl Responder {
    let html = Html::new("<p>OK</p>".to_string());
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