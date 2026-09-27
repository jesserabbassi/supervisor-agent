use super::*;

pub(super) struct SessionView;
impl SessionView {
    pub(super) fn render(app: &SupervisorApp, cx: &mut Context<SupervisorApp>) -> Div {
        let mut table = panel().flex_1().child(
            row()
                .child(column("Customer", 170.))
                .child(column("PC", 65.))
                .child(column("Start", 65.))
                .child(column("End", 65.))
                .child(column("Remaining", 100.))
                .child(column("Cost", 80.))
                .child(column("Status", 90.)),
        );
        for (i, s) in app.vm.session.sessions.iter().enumerate() {
            table = table.child(
                row()
                    .id(format!("session-{i}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.session.selected = i;
                        cx.notify();
                    }))
                    .bg(rgb(if app.vm.session.selected == i {
                        PANEL2
                    } else {
                        PANEL
                    }))
                    .child(column(&s.customer, 170.))
                    .child(column(&s.pc, 65.))
                    .child(column(&s.start, 65.))
                    .child(column(&s.end, 65.))
                    .child(column(&s.remaining, 100.))
                    .child(column(format!("{:.2} €", s.cost), 80.))
                    .child(badge(
                        &s.status,
                        if s.status == "Running" { YELLOW } else { GREEN },
                    )),
            );
        }
        let s = &app.vm.session.sessions[app.vm.session.selected];
        div().flex().gap_4().child(table).child(
            panel()
                .w(px(290.))
                .p_4()
                .flex()
                .flex_col()
                .gap_4()
                .child(section("Session Details"))
                .child(heading(format!("{} · {}", s.pc, s.status)))
                .child(label(&s.customer))
                .child(stat("Remaining", &s.remaining, ORANGE, "session time"))
                .child(stat(
                    "Current cost",
                    format!("{:.2} €", s.cost),
                    TEXT,
                    "2.00 € / hour",
                ))
                .child(section("Session Controls"))
                .child(app.session_action("Extend", cx))
                .child(app.session_action("Pause / Resume", cx))
                .child(app.session_action("Stop", cx)),
        )
    }
}
