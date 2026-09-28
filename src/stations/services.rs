use super::models::{Station, StationStatus};
use crate::infrastructure::{ApiClient, SignalRClient};
use crate::shared::services::SupervisorServices;

#[derive(Clone)]
pub struct StationService {
    api: ApiClient,
    demo_mode: bool,
}

impl StationService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo_mode: demo.demo_mode,
        }
    }

    pub fn get_stations(&self) -> Vec<Station> {
        let statuses = [
            StationStatus::Available,
            StationStatus::InSession,
            StationStatus::InSession,
            StationStatus::Offline,
            StationStatus::Locked,
            StationStatus::Available,
            StationStatus::InSession,
            StationStatus::Available,
            StationStatus::InSession,
            StationStatus::Available,
            StationStatus::Locked,
            StationStatus::Offline,
            StationStatus::Available,
            StationStatus::InSession,
            StationStatus::Available,
            StationStatus::Maintenance,
        ];
        let names = [
            "",
            "Ahmed Ben Ali",
            "Yassine Trabelsi",
            "",
            "",
            "",
            "Sara Mohamed",
            "",
            "Omar Khaled",
            "",
            "",
            "",
            "",
            "Maya Cherif",
            "",
            "",
        ];
        statuses
            .iter()
            .enumerate()
            .map(|(i, status)| Station {
                id: format!("PC-{:02}", i + 1),
                status: *status,
                customer: names[i].into(),
                cpu: [12, 47, 61, 0, 5, 17, 56, 9, 49, 11, 6, 0, 14, 52, 8, 0][i],
                gpu: [18, 72, 68, 0, 8, 21, 71, 14, 66, 16, 9, 0, 20, 65, 11, 0][i],
                ram: [28, 63, 55, 0, 12, 34, 62, 26, 58, 29, 18, 0, 31, 60, 20, 0][i],
                temp: [42, 58, 54, 0, 38, 43, 60, 41, 57, 40, 39, 0, 44, 56, 42, 0][i],
            })
            .collect()
    }

    pub fn get_station(&self, id: &str) -> Option<Station> {
        self.get_stations().into_iter().find(|s| s.id == id)
    }

    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct RemoteControlService {
    signalr: SignalRClient,
    demo_mode: bool,
}

impl RemoteControlService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            signalr: demo.signalr.clone(),
            demo_mode: demo.demo_mode,
        }
    }

    pub fn lock_station(&self, id: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            self.signalr.send("LockStation", id)
        }
    }

    pub fn unlock_station(&self, id: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            self.signalr.send("UnlockStation", id)
        }
    }

    pub fn restart_station(&self, id: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            self.signalr.send("RestartStation", id)
        }
    }

    pub fn shutdown_station(&self, id: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            self.signalr.send("ShutdownStation", id)
        }
    }
}
