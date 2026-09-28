use super::models::Session;
use super::services::SessionService;

pub struct SessionViewModel {
    pub selected: usize,
    pub sessions: Vec<Session>,
    pub service: SessionService,
}

impl SessionViewModel {
    pub fn load_sessions(service: SessionService) -> Self {
        let sessions = service.get_sessions();
        Self {
            selected: 0,
            sessions,
            service,
        }
    }
    pub fn start_session(&self, station: &str, customer: &str) -> Result<(), String> {
        self.service.start_session(station, customer)
    }
    pub fn extend_session(&self, minutes: u32) -> Result<(), String> {
        self.service
            .extend_session(&self.sessions[self.selected].pc, minutes)
    }
    pub fn stop_session(&self) -> Result<(), String> {
        self.service.stop_session(&self.sessions[self.selected].pc)
    }
    pub fn control(&mut self, action: &str) -> Result<(), String> {
        let pc = self.sessions[self.selected].pc.clone();
        match action {
            "Extend" => self.service.extend_session(&pc, 30)?,
            "Pause / Resume" => self.service.pause_session(&pc)?,
            "Stop" => self.service.stop_session(&pc)?,
            _ => return Err("Unknown session action".into()),
        }
        let session = &mut self.sessions[self.selected];
        match action {
            "Pause / Resume" => {
                session.status = if session.status == "Paused" {
                    "Running".into()
                } else {
                    "Paused".into()
                }
            }
            "Stop" => session.status = "Completed".into(),
            _ => {}
        }
        Ok(())
    }
}
