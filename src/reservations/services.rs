use super::models::Reservation;
use crate::infrastructure::ApiClient;
use crate::shared::services::SupervisorServices;
use crate::stations::models::StationStatus;
use crate::stations::services::StationService;

#[derive(Clone)]
pub struct ReservationService {
    api: ApiClient,
    station_service: StationService,
}

impl ReservationService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            station_service: StationService::new(demo),
        }
    }

    pub fn get_reservations(&self) -> Vec<Reservation> {
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

    pub fn check_availability(&self, station: &str) -> bool {
        self.station_service
            .get_stations()
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
