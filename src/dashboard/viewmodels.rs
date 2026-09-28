use crate::alerts::models::Alert;
use crate::alerts::viewmodels::AlertViewModel;
use crate::reservations::models::Reservation;
use crate::reservations::viewmodels::ReservationViewModel;
use crate::stations::models::Station;
use crate::stations::services::StationService;
use crate::stations::viewmodels::StationViewModel;

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
        alerts: &AlertViewModel,
    ) -> Self {
        Self {
            stations: station_service.get_stations(),
            reservations: reservations.reservations.clone(),
            alerts: alerts.alerts.clone(),
            station_service,
        }
    }

    pub fn refresh(
        &mut self,
        stations: &StationViewModel,
        reservations: &ReservationViewModel,
        alerts: &AlertViewModel,
    ) {
        self.stations = stations.stations.clone();
        self.reservations = reservations.reservations.clone();
        self.alerts = alerts.alerts.clone();
    }
}
