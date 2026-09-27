use super::*;

pub(super) struct DashboardView;
impl DashboardView {
    pub(super) fn render(app: &SupervisorApp, cx: &mut Context<SupervisorApp>) -> Div {
        let mut grid = div().flex().flex_wrap().gap_2();
        for s in app.vm.dashboard.stations.iter().take(12) {
            grid = grid.child(mini_station(&s.id, s.status));
        }
        let mut alerts = panel().p_4().flex_1().child(section("Recent Alerts"));
        for a in app.vm.dashboard.alerts.iter().take(4) {
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
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.vm.page = Page::Stations;
                                        cx.notify();
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
                            .child(app.reservation_rows(3, cx)),
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
                                    .child(app.nav_action(
                                        "New Reservation",
                                        Page::Reservations,
                                        cx,
                                    ))
                                    .child(app.nav_action("Add Customer", Page::Customers, cx))
                                    .child(app.nav_action("Station Management", Page::Stations, cx))
                                    .child(app.nav_action("View Reports", Page::Reports, cx)),
                            ),
                    ),
            )
    }
}
