use super::models::Session;
use crate::infrastructure::ApiClient;
use crate::shared::services::SupervisorServices;

#[derive(Clone)]
pub struct SessionService {
    api: ApiClient,
    demo_mode: bool,
}

impl SessionService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo_mode: demo.demo_mode,
        }
    }

    pub fn get_sessions(&self) -> Vec<Session> {
        [
            (
                "Ahmed Ben Ali",
                "PC-02",
                "14:20",
                "16:20",
                "01:24:32",
                4.5,
                "Running",
            ),
            (
                "Yassine Trabelsi",
                "PC-03",
                "13:45",
                "16:45",
                "02:12:18",
                6.0,
                "Running",
            ),
            (
                "Sara Mohamed",
                "PC-07",
                "15:10",
                "17:10",
                "01:50:05",
                4.0,
                "Running",
            ),
            (
                "Maya Cherif",
                "PC-06",
                "14:55",
                "16:25",
                "01:27:40",
                3.0,
                "Running",
            ),
            (
                "Omar Khaled",
                "PC-09",
                "14:05",
                "15:35",
                "00:42:11",
                3.0,
                "Running",
            ),
            (
                "Karim Ben Salem",
                "PC-11",
                "13:20",
                "17:20",
                "03:10:22",
                8.0,
                "Paused",
            ),
            (
                "Amine Haddad",
                "PC-01",
                "12:10",
                "14:10",
                "—",
                4.0,
                "Completed",
            ),
            (
                "Leila Ghorbel",
                "PC-08",
                "11:50",
                "13:20",
                "—",
                3.0,
                "Completed",
            ),
        ]
        .into_iter()
        .map(
            |(customer, pc, start, end, remaining, cost, status)| Session {
                customer: customer.into(),
                pc: pc.into(),
                start: start.into(),
                end: end.into(),
                remaining: remaining.into(),
                cost,
                status: status.into(),
            },
        )
        .collect()
    }

    pub fn start_session(&self, _station: &str, _customer: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }

    pub fn stop_session(&self, _station: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }

    pub fn extend_session(&self, _station: &str, _minutes: u32) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }

    pub fn pause_session(&self, _station: &str) -> Result<(), String> {
        if self.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }

    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}
