use super::models::Alert;

#[derive(Clone, Default)]
pub struct AlertService;

impl AlertService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        [
            (
                "Critical",
                "PC-04 is offline",
                "No response for 5 minutes",
                "PC-04",
                "14:28",
                false,
            ),
            (
                "Warning",
                "High CPU usage detected",
                "CPU usage above 90%",
                "PC-07",
                "14:15",
                false,
            ),
            (
                "Warning",
                "Low disk space",
                "Less than 10% remaining",
                "PC-12",
                "13:47",
                true,
            ),
            (
                "Info",
                "Scheduled maintenance",
                "Payment system maintenance at 02:00",
                "System",
                "12:30",
                true,
            ),
            (
                "Critical",
                "Network latency high",
                "Latency above 200ms",
                "Network",
                "11:22",
                false,
            ),
            (
                "Info",
                "New customer registered",
                "Welcome Ahmed Ben Ali",
                "Customer",
                "10:15",
                true,
            ),
        ]
        .into_iter()
        .map(|(severity, title, detail, source, time, read)| Alert {
            severity: severity.into(),
            title: title.into(),
            detail: detail.into(),
            source: source.into(),
            time: time.into(),
            read,
        })
        .collect()
    }
}
