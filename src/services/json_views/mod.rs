pub mod routes {

    use crate::{app::AppState, models::system_report::SystemReporter};
    use actix_web::{HttpResponse, Responder, get, web};

    #[get("/json-views/overview")]
    pub async fn render_overview_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(system_report)
    }

    #[get("/json-views/system")]
    pub async fn render_system_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(&system_report.system_info)
    }

    #[get("/json-views/cpus")]
    pub async fn render_cpus_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(&system_report.cpu_report_info)
    }

    #[get("/json-views/disks")]
    pub async fn render_disks_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(&system_report.disks_report_info)
    }

    #[get("/json-views/components")]
    pub async fn render_components_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(&system_report.components_report_info)
    }

    #[get("/json-views/networks")]
    pub async fn render_networks_as_json(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();
        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(&system_report.network_report_info)
    }
}
