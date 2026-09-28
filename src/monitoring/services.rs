use crate::infrastructure::SignalRClient;
use crate::shared::services::SupervisorServices;
use crate::stations::models::Station;
use crate::stations::services::StationService;

#[derive(Clone)]
pub struct MonitoringService {
    signalr: SignalRClient,
    station_service: StationService,
}

impl MonitoringService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            signalr: demo.signalr.clone(),
            station_service: StationService::new(demo),
        }
    }

    pub fn get_telemetry(&self) -> Vec<Station> {
        self.station_service.get_stations()
    }

    pub fn update_realtime_data(&self) -> Result<(), String> {
        self.signalr.connect()
    }
}
