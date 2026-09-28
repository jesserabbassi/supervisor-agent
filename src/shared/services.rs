use crate::infrastructure::{ApiClient, SignalRClient};

#[derive(Clone)]
pub struct SupervisorServices {
    pub api: ApiClient,
    pub signalr: SignalRClient,
    pub demo_mode: bool,
}

impl SupervisorServices {
    pub fn demo() -> Self {
        Self {
            api: ApiClient::new("http://localhost:5000"),
            signalr: SignalRClient::new("http://localhost:5000/hubs/supervisor"),
            demo_mode: true,
        }
    }
}
