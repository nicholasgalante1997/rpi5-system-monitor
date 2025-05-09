use crate::ui::Html;
use actix_web::{HttpResponse, web};

pub async fn handler() -> HttpResponse {
    let html = Html::new("<p>OK</p>".to_string());
    HttpResponse::Ok().body(html.into_page())
}

pub fn configure_health_check_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(handler))
            .route(web::post().to(HttpResponse::MethodNotAllowed))
            .route(web::put().to(HttpResponse::MethodNotAllowed))
            .route(web::patch().to(HttpResponse::MethodNotAllowed))
            .route(web::delete().to(HttpResponse::MethodNotAllowed)),
    );
}
