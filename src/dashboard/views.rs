use super::viewmodels::DashboardViewModel;
use crate::reservations::models::Reservation;
use crate::reservations::views::reservation_rows;
use crate::shared::navigation::Page;
use crate::shared::theme::*;
use crate::shared::ui::*;
use crate::stations::views::mini_station;
use gpui::{div, prelude::*, px, Context, Div, Stateful};

pub struct DashboardView;

impl DashboardView {
    pub fn render<T: 'static>(
        dash_vm: &DashboardViewModel,
        reservations: &[Reservation],
        cx: &mut Context<T>,
        on_navigate: impl Fn(&mut T, Page, &mut Context<T>) + 'static + Send + Sync + Copy,
        on_select_reservation: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> Div {
        let mut grid = div().flex().flex_wrap().gap_2();
        for s in dash_vm.stations.iter().take(12) {
            grid = grid.child(mini_station(&s.id, s.status));
        }
        let mut alerts = panel().p_4().flex_1().child(section("Recent Alerts"));
        for a in dash_vm.alerts.iter().take(4) {
            alerts = alerts.child(
                row()
                    .child(badge(
                        &a.severity,
                        if a.severity == "Critical" {
                            RED
                        } else {
                            YELLOW
                        },
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(column(&a.title, 200.))
                            .child(label(&a.detail)),
                    ),
            );
        }

        let nav_action = move |text: &'static str, target_page: Page, cx: &mut Context<T>| -> Stateful<Div> {
            div()
                .id(format!("quick-{text}"))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_navigate(this, target_page, cx);
                }))
                .child(action(text, false))
        };

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(stat("Total PCs", "16", BLUE, "12 connected"))
                    .child(stat("Active Customers", "56", GREEN, "↑ 12% today"))
                    .child(stat("Active Sessions", "8", YELLOW, "↑ 14% today"))
                    .child(stat("Today's Reservations", "18", BLUE, "4 upcoming"))
                    .child(stat("Today's Revenue", "315.50 €", RED, "↑ 12% today")),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        panel()
                            .p_4()
                            .w(px(590.))
                            .child(section("Station Overview"))
                            .child(div().mt_4().flex().flex_wrap().gap_2().child(grid))
                            .child(
                                div()
                                    .id("all-stations")
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        on_navigate(this, Page::Stations, cx);
                                    }))
                                    .mt_3()
                                    .child(action("View All Stations  →", false)),
                            ),
                    )
                    .child(alerts),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        panel()
                            .p_4()
                            .flex_1()
                            .child(section("Upcoming Reservations"))
                            .child(reservation_rows(reservations, 3, cx, on_select_reservation)),
                    )
                    .child(
                        panel()
                            .p_4()
                            .flex_1()
                            .child(section("Quick Actions"))
                            .child(
                                div()
                                    .mt_3()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(nav_action(
                                        "New Reservation",
                                        Page::Reservations,
                                        cx,
                                    ))
                                    .child(nav_action("Add Customer", Page::Customers, cx))
                                    .child(nav_action("Station Management", Page::Stations, cx))
                                    .child(nav_action("View Reports", Page::Reports, cx)),
                            ),
                    ),
            )
    }
}
