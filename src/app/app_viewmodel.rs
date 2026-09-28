use crate::alerts::services::AlertService;
use crate::alerts::viewmodels::AlertViewModel;
use crate::customers::services::CustomerService;
use crate::customers::viewmodels::CustomerViewModel;
use crate::dashboard::viewmodels::DashboardViewModel;
use crate::monitoring::services::MonitoringService;
use crate::monitoring::viewmodels::MonitoringViewModel;
use crate::reservations::services::ReservationService;
use crate::reservations::viewmodels::ReservationViewModel;
use crate::sessions::services::SessionService;
use crate::sessions::viewmodels::SessionViewModel;
use crate::shared::navigation::Page;
use crate::shared::services::SupervisorServices;
use crate::stations::services::{RemoteControlService, StationService};
use crate::stations::viewmodels::StationViewModel;
use crate::wallet::services::WalletService;
use crate::wallet::viewmodels::WalletViewModel;

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
    pub alerts: AlertViewModel,
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
        let alert_service = AlertService::new();
        let alerts = AlertViewModel::new(alert_service);
        let monitoring = MonitoringViewModel::load_telemetry(
            MonitoringService::new(&demo),
            alerts.alerts.clone(),
        );
        let dashboard = DashboardViewModel::load_dashboard(
            StationService::new(&demo),
            &reservation,
            &alerts,
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
            alerts,
        }
    }

    pub fn station_action(&mut self, action: &str) {
        self.notice = Some(self.station.action(action));
        self.dashboard
            .refresh(&self.station, &self.reservation, &self.alerts);
    }
}
