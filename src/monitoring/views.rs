use super::viewmodels::MonitoringViewModel;
use crate::shared::theme::*;
use crate::shared::ui::*;
use crate::stations::views::mini_station;
use gpui::{div, prelude::*, px, Div};

pub struct MonitoringView;

impl MonitoringView {
    pub fn render(monitoring_vm: &MonitoringViewModel) -> Div {
        let mut grid = div().flex().flex_wrap().gap_2();
        for s in &monitoring_vm.telemetry {
            grid = grid.child(mini_station(&s.id, s.status));
        }
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(stat("Total PCs", "16", BLUE, "12 online"))
                    .child(stat("Active Customers", "56", GREEN, "↑ 12%"))
                    .child(stat("Active Sessions", "8", ORANGE, "↑ 14%"))
                    .child(stat("System Uptime", "99.8%", BLUE, "this month")),
            )
            .child(
                panel()
                    .p_4()
                    .child(section("Station Status Overview"))
                    .child(div().mt_4().flex().flex_wrap().gap_2().child(grid)),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        panel()
                            .p_4()
                            .flex_1()
                            .child(section("Resource Usage"))
                            .child(
                                div()
                                    .mt_4()
                                    .flex()
                                    .gap_3()
                                    .child(stat("CPU", "42%", BLUE, "average"))
                                    .child(stat("GPU", "68%", ORANGE, "average"))
                                    .child(stat("RAM", "56%", GREEN, "average"))
                                    .child(stat("Storage", "37%", YELLOW, "used")),
                            ),
                    )
                    .child(
                        panel()
                            .p_4()
                            .w(px(290.))
                            .child(section("System Health"))
                            .child(label("● Network                 Healthy"))
                            .child(label("● Monitoring services    Healthy"))
                            .child(label("● Booking system         Healthy")),
                    ),
            )
    }
}
