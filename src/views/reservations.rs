use super::*;

pub(super) struct ReservationView;
impl ReservationView {
    pub(super) fn render(app: &SupervisorApp, cx: &mut Context<SupervisorApp>) -> Div {
        let r = &app.vm.reservation.reservations[app.vm.reservation.selected];
        div()
            .flex()
            .gap_4()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        panel()
                            .p_4()
                            .child(section("April 18, 2024  ·  Booking timeline"))
                            .child(div().mt_4().flex().flex_wrap().gap_2().children(
                                app.vm.reservation.reservations.iter().map(|r| {
                                    badge(
                                        format!("{}  {}  {}", r.pc, r.time, r.customer),
                                        if r.status == "Confirmed" {
                                            BLUE
                                        } else {
                                            YELLOW
                                        },
                                    )
                                }),
                            )),
                    )
                    .child(
                        panel()
                            .p_4()
                            .child(section("Reservations"))
                            .child(app.reservation_rows(5, cx)),
                    ),
            )
            .child(
                panel()
                    .w(px(300.))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(section("Reservation Details"))
                    .child(badge(
                        &r.status,
                        if r.status == "Confirmed" {
                            GREEN
                        } else {
                            YELLOW
                        },
                    ))
                    .child(heading(&r.customer))
                    .child(label(format!("{} · {}", r.id, r.pc)))
                    .child(label(format!("April 18, 2024 · {}", r.time)))
                    .child(label(format!("Duration: {}", r.duration)))
                    .child(badge(
                        &r.payment,
                        if r.payment == "Paid" { BLUE } else { RED },
                    ))
                    .child(action("Edit Reservation", false))
                    .child(action("Reschedule", false)),
            )
    }
}
