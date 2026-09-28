use super::models::{Station, StationStatus};
use super::services::{RemoteControlService, StationService};

pub struct StationViewModel {
    pub selected: usize,
    pub filter: &'static str,
    pub stations: Vec<Station>,
    pub service: StationService,
    pub remote_control: RemoteControlService,
}

impl StationViewModel {
    pub fn load_stations(service: StationService, remote_control: RemoteControlService) -> Self {
        let stations = service.get_stations();
        Self {
            selected: 6,
            filter: "All",
            stations,
            service,
            remote_control,
        }
    }
    pub fn select_station(&mut self, index: usize) {
        if index < self.stations.len() {
            self.selected = index;
        }
    }
    pub fn action(&mut self, action: &str) -> String {
        let station = &mut self.stations[self.selected];
        let result = match action {
            "Lock" => self.remote_control.lock_station(&station.id),
            "Unlock" => self.remote_control.unlock_station(&station.id),
            "Restart" => self.remote_control.restart_station(&station.id),
            "Shutdown" => self.remote_control.shutdown_station(&station.id),
            _ => Err("Unknown station action".into()),
        };
        match result {
            Ok(()) => {
                match action {
                    "Lock" => station.status = StationStatus::Locked,
                    "Unlock" => station.status = StationStatus::Available,
                    _ => {}
                }
                format!("{}: {} applied in demo mode", station.id, action)
            }
            Err(error) => error,
        }
    }
}
