use super::services::MonitoringService;
use crate::alerts::models::Alert;
use crate::stations::models::Station;

pub struct MonitoringViewModel {
    pub telemetry: Vec<Station>,
    pub alerts: Vec<Alert>,
    pub service: MonitoringService,
}

impl MonitoringViewModel {
    pub fn load_telemetry(service: MonitoringService, alerts: Vec<Alert>) -> Self {
        let telemetry = service.get_telemetry();
        Self {
            telemetry,
            alerts,
            service,
        }
    }

    pub fn update_realtime_data(&self) -> Result<(), String> {
        self.service.update_realtime_data()
    }
}
