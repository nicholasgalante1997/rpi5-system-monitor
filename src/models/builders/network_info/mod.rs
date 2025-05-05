use crate::models::data_objects::network_info::NetworkReportInfo;

pub struct NetworkReportInfoBuilder {
    interface_name: Option<String>,
    rx_bytes: Option<u64>,
    tx_bytes: Option<u64>,
    rx_packets: Option<u64>,
    tx_packets: Option<u64>,
    rx_errors: Option<u64>,
    tx_errors: Option<u64>,
}

impl NetworkReportInfoBuilder {
    pub fn new() -> Self {
        NetworkReportInfoBuilder {
            interface_name: None,
            rx_bytes: None,
            tx_bytes: None,
            rx_packets: None,
            tx_packets: None,
            rx_errors: None,
            tx_errors: None,
        }
    }

    pub fn set_interface_name(mut self, interface_name: String) -> Self {
        self.interface_name = Some(interface_name);
        self
    }

    pub fn set_rx_bytes(mut self, rx_bytes: u64) -> Self {
        self.rx_bytes = Some(rx_bytes);
        self
    }

    pub fn set_tx_bytes(mut self, tx_bytes: u64) -> Self {
        self.tx_bytes = Some(tx_bytes);
        self
    }

    pub fn set_rx_packets(mut self, rx_packets: u64) -> Self {
        self.rx_packets = Some(rx_packets);
        self
    }

    pub fn set_tx_packets(mut self, tx_packets: u64) -> Self {
        self.tx_packets = Some(tx_packets);
        self
    }

    pub fn set_rx_errors(mut self, rx_errors: u64) -> Self {
        self.rx_errors = Some(rx_errors);
        self
    }

    pub fn set_tx_errors(mut self, tx_errors: u64) -> Self {
        self.tx_errors = Some(tx_errors);
        self
    }

    pub fn build(&self) -> NetworkReportInfo {
        NetworkReportInfo {
            interface_name: self.interface_name.clone().unwrap(),
            rx_bytes: self.rx_bytes.clone().unwrap(),
            tx_bytes: self.tx_bytes.clone().unwrap(),
            rx_packets: self.rx_packets.clone().unwrap(),
            tx_packets: self.tx_packets.clone().unwrap(),
            rx_errors: self.rx_errors.clone().unwrap(),
            tx_errors: self.tx_errors.clone().unwrap(), // Assuming we have a default value for tx_errors if not provided
        }
    }
}
