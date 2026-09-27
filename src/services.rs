use crate::infrastructure::{ApiClient, SignalRClient};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StationStatus {
    Available,
    InSession,
    Locked,
    Offline,
    Maintenance,
}
impl StationStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Available => "AVAILABLE",
            Self::InSession => "IN SESSION",
            Self::Locked => "LOCKED",
            Self::Offline => "OFFLINE",
            Self::Maintenance => "MAINTENANCE",
        }
    }
}

#[derive(Clone)]
pub struct Station {
    pub id: String,
    pub status: StationStatus,
    pub customer: String,
    pub cpu: u8,
    pub gpu: u8,
    pub ram: u8,
    pub temp: u8,
}
#[derive(Clone)]
pub struct Session {
    pub customer: String,
    pub pc: String,
    pub start: String,
    pub end: String,
    pub remaining: String,
    pub cost: f32,
    pub status: String,
}
#[derive(Clone)]
pub struct Reservation {
    pub id: String,
    pub customer: String,
    pub pc: String,
    pub time: String,
    pub duration: String,
    pub status: String,
    pub payment: String,
}
#[derive(Clone)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub membership: String,
    pub balance: f32,
    pub online: bool,
}
#[derive(Clone)]
pub struct Transaction {
    pub id: String,
    pub customer: String,
    pub kind: String,
    pub amount: f32,
    pub method: String,
}
#[derive(Clone)]
pub struct Alert {
    pub severity: String,
    pub title: String,
    pub detail: String,
    pub source: String,
    pub time: String,
    pub read: bool,
}

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
    pub fn stations(&self) -> Vec<Station> {
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
    pub fn sessions(&self) -> Vec<Session> {
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
    pub fn reservations(&self) -> Vec<Reservation> {
        [
            (
                "RES-0187",
                "Ahmed Ben Ali",
                "PC-01",
                "08:00 – 12:00",
                "4h",
                "Confirmed",
                "Paid",
            ),
            (
                "RES-0188",
                "Yassine Trabelsi",
                "PC-02",
                "10:00 – 14:00",
                "4h",
                "Pending",
                "Unpaid",
            ),
            (
                "RES-0189",
                "Sara Mohamed",
                "PC-03",
                "15:00 – 20:00",
                "5h",
                "Confirmed",
                "Paid",
            ),
            (
                "RES-0190",
                "Maya Cherif",
                "PC-01",
                "14:00 – 18:00",
                "4h",
                "Confirmed",
                "Paid",
            ),
            (
                "RES-0191",
                "Tarek Charbi",
                "PC-08",
                "12:00 – 18:00",
                "6h",
                "Pending",
                "Unpaid",
            ),
        ]
        .into_iter()
        .map(
            |(id, customer, pc, time, duration, status, payment)| Reservation {
                id: id.into(),
                customer: customer.into(),
                pc: pc.into(),
                time: time.into(),
                duration: duration.into(),
                status: status.into(),
                payment: payment.into(),
            },
        )
        .collect()
    }
    pub fn customers(&self) -> Vec<Customer> {
        [
            (
                "C-1042",
                "Ahmed Ben Ali",
                "ahmed.benali@email.com",
                "VIP Gold",
                45.5,
                true,
            ),
            (
                "C-1041",
                "Yassine Trabelsi",
                "yassine.t@email.com",
                "Regular",
                12.0,
                true,
            ),
            (
                "C-1040",
                "Sara Mohamed",
                "sara.m@email.com",
                "Student",
                8.5,
                false,
            ),
            (
                "C-1039",
                "Maya Cherif",
                "maya.c@email.com",
                "VIP Gold",
                32.0,
                true,
            ),
            (
                "C-1038",
                "Omar Khaled",
                "omar.k@email.com",
                "Regular",
                5.0,
                false,
            ),
            (
                "C-1037",
                "Karim Ben Salem",
                "karim.b@email.com",
                "VIP Silver",
                18.0,
                true,
            ),
            (
                "C-1036",
                "Amine Haddad",
                "amine.h@email.com",
                "Regular",
                0.0,
                false,
            ),
            (
                "C-1035",
                "Leila Ghorbel",
                "leila.g@email.com",
                "Student",
                6.5,
                true,
            ),
        ]
        .into_iter()
        .map(|(id, name, email, membership, balance, online)| Customer {
            id: id.into(),
            name: name.into(),
            email: email.into(),
            membership: membership.into(),
            balance,
            online,
        })
        .collect()
    }
    pub fn transactions(&self) -> Vec<Transaction> {
        [
            ("TX-2841", "Ahmed Ben Ali", "Top Up", 20.0, "Credit Card"),
            ("TX-2840", "Maya Cherif", "Payment", -5.5, "Wallet Balance"),
            ("TX-2839", "Yassine Trabelsi", "Top Up", 15.0, "Cash"),
            (
                "TX-2838",
                "PC-03 Session",
                "Payment",
                -6.0,
                "Wallet Balance",
            ),
            ("TX-2837", "Omar Khaled", "Top Up", 10.0, "Mobile Payment"),
        ]
        .into_iter()
        .map(|(id, customer, kind, amount, method)| Transaction {
            id: id.into(),
            customer: customer.into(),
            kind: kind.into(),
            amount,
            method: method.into(),
        })
        .collect()
    }
    pub fn alerts(&self) -> Vec<Alert> {
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

// Each feature owns its backend boundary. The fixture provider above is used
// only while the server team's contracts are unavailable.
#[derive(Clone)]
pub struct StationService {
    api: ApiClient,
    demo: SupervisorServices,
}
impl StationService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_stations(&self) -> Vec<Station> {
        self.demo.stations()
    }
    pub fn get_station(&self, id: &str) -> Option<Station> {
        self.get_stations().into_iter().find(|s| s.id == id)
    }
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct SessionService {
    api: ApiClient,
    demo: SupervisorServices,
}
impl SessionService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_sessions(&self) -> Vec<Session> {
        self.demo.sessions()
    }
    pub fn start_session(&self, _station: &str, _customer: &str) -> Result<(), String> {
        if self.demo.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }
    pub fn stop_session(&self, _station: &str) -> Result<(), String> {
        if self.demo.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }
    pub fn extend_session(&self, _station: &str, _minutes: u32) -> Result<(), String> {
        if self.demo.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }
    pub fn pause_session(&self, _station: &str) -> Result<(), String> {
        if self.demo.demo_mode {
            Ok(())
        } else {
            Err("Session API contract pending".into())
        }
    }
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct ReservationService {
    api: ApiClient,
    demo: SupervisorServices,
}
impl ReservationService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_reservations(&self) -> Vec<Reservation> {
        self.demo.reservations()
    }
    pub fn check_availability(&self, station: &str) -> bool {
        self.demo
            .stations()
            .iter()
            .any(|s| s.id == station && s.status == StationStatus::Available)
    }
    pub fn create_reservation(&self, _reservation: &Reservation) -> Result<(), String> {
        Err("Reservation API contract pending".into())
    }
    pub fn cancel_reservation(&self, _id: &str) -> Result<(), String> {
        Err("Reservation API contract pending".into())
    }
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct CustomerService {
    api: ApiClient,
    demo: SupervisorServices,
}
impl CustomerService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_customers(&self) -> Vec<Customer> {
        self.demo.customers()
    }
    pub fn search(&self, query: &str) -> Vec<Customer> {
        self.get_customers()
            .into_iter()
            .filter(|c| c.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct WalletService {
    api: ApiClient,
    demo: SupervisorServices,
}
impl WalletService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_balance(&self, customer: &str) -> Option<f32> {
        self.demo
            .customers()
            .into_iter()
            .find(|c| c.id == customer)
            .map(|c| c.balance)
    }
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.demo.transactions()
    }
    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

#[derive(Clone)]
pub struct MonitoringService {
    signalr: SignalRClient,
    demo: SupervisorServices,
}
impl MonitoringService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            signalr: demo.signalr.clone(),
            demo: demo.clone(),
        }
    }
    pub fn get_telemetry(&self) -> Vec<Station> {
        self.demo.stations()
    }
    pub fn get_alerts(&self) -> Vec<Alert> {
        self.demo.alerts()
    }
    pub fn update_realtime_data(&self) -> Result<(), String> {
        self.signalr.connect()
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
