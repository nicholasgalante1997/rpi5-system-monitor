use sysinfo::{Components, Disks, Networks, System};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub components: Arc<Mutex<Components>>,
    pub disks: Arc<Mutex<Disks>>,
    pub networks: Arc<Mutex<Networks>>,
    pub system: Arc<Mutex<System>>,
}