use super::*;

pub(super) struct AlertView;
impl AlertView {
    pub(super) fn render(app: &SupervisorApp, cx: &mut Context<SupervisorApp>) -> Div {
        let mut filters = div().flex().gap_2();
        for name in ["All", "Critical", "Warning", "Info"] {
            filters = filters.child(
                div()
                    .id(format!("alert-filter-{name}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.alert_filter = name;
                        cx.notify();
                    }))
                    .child(badge(
                        name,
                        if app.vm.alert_filter == name {
                            ORANGE
                        } else {
                            BORDER
                        },
                    )),
            );
        }
        let mut table = panel().flex_1().child(
            row()
                .child(column("Type", 90.))
                .child(column("Message", 280.))
                .child(column("Source", 90.))
                .child(column("Time", 65.))
                .child(column("Status", 70.)),
        );
        for (i, a) in app.vm.monitoring.alerts.iter().enumerate() {
            if app.vm.alert_filter != "All" && app.vm.alert_filter != a.severity {
                continue;
            }
            table = table.child(
                row()
                    .id(format!("alert-{i}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.monitoring.alerts[i].read = true;
                        cx.notify();
                    }))
                    .child(badge(
                        &a.severity,
                        if a.severity == "Critical" {
                            RED
                        } else if a.severity == "Warning" {
                            YELLOW
                        } else {
                            BLUE
                        },
                    ))
                    .child(
                        div()
                            .w(px(280.))
                            .flex()
                            .flex_col()
                            .child(column(&a.title, 280.))
                            .child(label(&a.detail)),
                    )
                    .child(column(&a.source, 90.))
                    .child(column(&a.time, 65.))
                    .child(label(if a.read { "Read" } else { "Unread" })),
            );
        }
        div().flex().flex_col().gap_4().child(filters).child(
            div().flex().gap_4().child(table).child(
                panel()
                    .w(px(245.))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(section("Alert Statistics"))
                    .child(stat("Critical", "2", RED, "needs attention"))
                    .child(stat("Warnings", "2", YELLOW, "review soon"))
                    .child(stat("Info", "2", BLUE, "updates")),
            ),
        )
    }
}
