use crate::services::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Stations,
    Sessions,
    Reservations,
    Customers,
    Wallet,
    Monitoring,
    Alerts,
    Reports,
    Settings,
}
impl Page {
    pub const ALL: [Page; 10] = [
        Self::Dashboard,
        Self::Stations,
        Self::Sessions,
        Self::Reservations,
        Self::Customers,
        Self::Wallet,
        Self::Monitoring,
        Self::Alerts,
        Self::Reports,
        Self::Settings,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Stations => "Gaming Stations",
            Self::Sessions => "Sessions",
            Self::Reservations => "Reservations",
            Self::Customers => "Customers",
            Self::Wallet => "Wallet / Payments",
            Self::Monitoring => "Monitoring",
            Self::Alerts => "Alerts",
            Self::Reports => "Reports",
            Self::Settings => "Settings",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Self::Dashboard => "⌂",
            Self::Stations => "▣",
            Self::Sessions => "◷",
            Self::Reservations => "▦",
            Self::Customers => "♙",
            Self::Wallet => "▤",
            Self::Monitoring => "◎",
            Self::Alerts => "⚠",
            Self::Reports => "▥",
            Self::Settings => "⚙",
        }
    }
    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Dashboard => "Here's what's happening at your gaming house today.",
            Self::Stations => "Manage and monitor all gaming PCs in real time",
            Self::Sessions => "Monitor and control all active gaming sessions",
            Self::Reservations => "Manage customer reservations and bookings",
            Self::Customers => "Manage your gaming house customers",
            Self::Wallet => "Manage customer wallets, payments and transactions",
            Self::Monitoring => "Real-time overview of all systems and gaming stations",
            Self::Alerts => "Stay informed about important events and system notifications",
            Self::Reports => "Gain insights into your gaming house performance",
            Self::Settings => "Manage your account, preferences and system configuration",
        }
    }
}

pub struct DashboardViewModel {
    pub stations: Vec<Station>,
    pub reservations: Vec<Reservation>,
    pub alerts: Vec<Alert>,
    pub station_service: StationService,
}
impl DashboardViewModel {
    pub fn load_dashboard(
        station_service: StationService,
        reservations: &ReservationViewModel,
        monitoring: &MonitoringViewModel,
    ) -> Self {
        Self {
            stations: station_service.get_stations(),
            reservations: reservations.reservations.clone(),
            alerts: monitoring.alerts.clone(),
            station_service,
        }
    }
    pub fn refresh(
        &mut self,
        stations: &StationViewModel,
        reservations: &ReservationViewModel,
        monitoring: &MonitoringViewModel,
    ) {
        self.stations = stations.stations.clone();
        self.reservations = reservations.reservations.clone();
        self.alerts = monitoring.alerts.clone();
    }
}

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

pub struct ReservationViewModel {
    pub selected: usize,
    pub reservations: Vec<Reservation>,
    pub service: ReservationService,
}
impl ReservationViewModel {
    pub fn load_reservations(service: ReservationService) -> Self {
        let reservations = service.get_reservations();
        Self {
            selected: 0,
            reservations,
            service,
        }
    }
    pub fn create_reservation(&self, reservation: &Reservation) -> Result<(), String> {
        self.service.create_reservation(reservation)
    }
    pub fn cancel_reservation(&self) -> Result<(), String> {
        self.service
            .cancel_reservation(&self.reservations[self.selected].id)
    }
}

pub struct CustomerViewModel {
    pub selected: usize,
    pub customers: Vec<Customer>,
    pub service: CustomerService,
}
impl CustomerViewModel {
    pub fn load_customers(service: CustomerService) -> Self {
        let customers = service.get_customers();
        Self {
            selected: 0,
            customers,
            service,
        }
    }
    pub fn search_customer(&self, query: &str) -> Vec<Customer> {
        self.service.search(query)
    }
}

pub struct WalletViewModel {
    pub transactions: Vec<Transaction>,
    pub service: WalletService,
}
impl WalletViewModel {
    pub fn load_wallet(service: WalletService) -> Self {
        let transactions = service.get_transactions();
        Self {
            transactions,
            service,
        }
    }
    pub fn load_transactions(&mut self) {
        self.transactions = self.service.get_transactions();
    }
}

pub struct MonitoringViewModel {
    pub telemetry: Vec<Station>,
    pub alerts: Vec<Alert>,
    pub service: MonitoringService,
}
impl MonitoringViewModel {
    pub fn load_telemetry(service: MonitoringService) -> Self {
        let telemetry = service.get_telemetry();
        let alerts = service.get_alerts();
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

pub struct SupervisorViewModel {
    pub page: Page,
    pub notice: Option<String>,
    pub dashboard: DashboardViewModel,
    pub station: StationViewModel,
    pub session: SessionViewModel,
    pub reservation: ReservationViewModel,
    pub customer: CustomerViewModel,
    pub wallet: WalletViewModel,
    pub monitoring: MonitoringViewModel,
    pub alert_filter: &'static str,
}
impl SupervisorViewModel {
    pub fn new() -> Self {
        let demo = SupervisorServices::demo();
        let station = StationViewModel::load_stations(
            StationService::new(&demo),
            RemoteControlService::new(&demo),
        );
        let session = SessionViewModel::load_sessions(SessionService::new(&demo));
        let reservation = ReservationViewModel::load_reservations(ReservationService::new(&demo));
        let customer = CustomerViewModel::load_customers(CustomerService::new(&demo));
        let wallet = WalletViewModel::load_wallet(WalletService::new(&demo));
        let monitoring = MonitoringViewModel::load_telemetry(MonitoringService::new(&demo));
        let dashboard = DashboardViewModel::load_dashboard(
            StationService::new(&demo),
            &reservation,
            &monitoring,
        );
        Self {
            page: Page::Dashboard,
            notice: None,
            dashboard,
            station,
            session,
            reservation,
            customer,
            wallet,
            monitoring,
            alert_filter: "All",
        }
    }
    pub fn station_action(&mut self, action: &str) {
        self.notice = Some(self.station.action(action));
        self.dashboard
            .refresh(&self.station, &self.reservation, &self.monitoring);
    }
}
