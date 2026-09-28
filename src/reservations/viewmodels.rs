use super::models::Reservation;
use super::services::ReservationService;

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
