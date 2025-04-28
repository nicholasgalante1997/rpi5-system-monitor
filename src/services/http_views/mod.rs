pub mod routes {

    use actix_web::{get, web, HttpResponse, Responder};
    use anyhow::Error;
    use futures::{future::ok, stream::once};
    use crate::{app::AppState, models::system_report::SystemReporter, ui::HttpViews};

    #[get("/http-views/pages/overview")]
    pub async fn render_overview_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
    
        let mut system_reporter = SystemReporter::new(&mut components, &mut disks, &networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_overview_view(&system_report.system_info, &system_report.disks_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));

        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/cpu")]
    pub async fn render_cpu_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
    
        let mut system_reporter = SystemReporter::new(&mut components, &mut disks, &networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_cpu_view(&system_report.system_info, &system_report.cpu_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));

        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/memory")]
    pub async fn render_memory_as_html(app_state: web::Data<AppState>) -> impl Responder {

        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter = SystemReporter::new(&mut components, &mut disks, &networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_memory_view(&system_report.system_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }


    #[get("/http-views/pages/disks")]
    pub async fn render_disks_as_html(app_state: web::Data<AppState>) -> impl Responder {

        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter = SystemReporter::new(&mut components, &mut disks, &networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_disks_view(&system_report.disks_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

}