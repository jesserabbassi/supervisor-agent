use super::models::Reservation;
use super::viewmodels::ReservationViewModel;
use crate::shared::theme::*;
use crate::shared::ui::*;
use gpui::{div, prelude::*, px, Context, Div};

pub fn reservation_rows<T: 'static>(
    reservations: &[Reservation],
    limit: usize,
    cx: &mut Context<T>,
    on_select: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
) -> Div {
    let mut rows = div().flex().flex_col();
    for (i, r) in reservations.iter().take(limit).enumerate() {
        rows = rows.child(
            row()
                .id(format!("reservation-{i}"))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_select(this, i, cx);
                }))
                .child(column(&r.customer, 160.))
                .child(column(&r.pc, 70.))
                .child(column(&r.time, 120.))
                .child(badge(
                    &r.status,
                    if r.status == "Confirmed" {
                        GREEN
                    } else {
                        YELLOW
                    },
                )),
        );
    }
    rows
}

pub struct ReservationView;

impl ReservationView {
    pub fn render<T: 'static>(
        res_vm: &ReservationViewModel,
        cx: &mut Context<T>,
        on_select: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> Div {
        let r = &res_vm.reservations[res_vm.selected];
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
                                res_vm.reservations.iter().map(|r| {
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
                            .child(reservation_rows(&res_vm.reservations, 5, cx, on_select)),
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
