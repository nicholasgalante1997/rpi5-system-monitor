use sysinfo::{Components, Disks, Networks, System};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub components: Arc<Mutix<Components>,
    pub system: Arc<Mutex<System>>,
}