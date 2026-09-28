use super::models::Alert;
use super::services::AlertService;

pub struct AlertViewModel {
    pub alerts: Vec<Alert>,
    pub filter: &'static str,
    pub service: AlertService,
}

impl AlertViewModel {
    pub fn new(service: AlertService) -> Self {
        let alerts = service.get_alerts();
        Self {
            alerts,
            filter: "All",
            service,
        }
    }

    pub fn mark_as_read(&mut self, index: usize) {
        if let Some(alert) = self.alerts.get_mut(index) {
            alert.read = true;
        }
    }
}
