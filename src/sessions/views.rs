use super::viewmodels::SessionViewModel;
use crate::shared::theme::*;
use crate::shared::ui::*;
use gpui::{div, prelude::*, px, rgb, Context, Div, Stateful};

pub struct SessionView;

impl SessionView {
    pub fn render<T: 'static>(
        session_vm: &SessionViewModel,
        cx: &mut Context<T>,
        on_select: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
        on_action: impl Fn(&mut T, &'static str, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> Div {
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
        for (i, s) in session_vm.sessions.iter().enumerate() {
            table = table.child(
                row()
                    .id(format!("session-{i}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        on_select(this, i, cx);
                    }))
                    .bg(rgb(if session_vm.selected == i {
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
        let s = &session_vm.sessions[session_vm.selected];
        let session_action_button = |name: &'static str, cx: &mut Context<T>| -> Stateful<Div> {
            div()
                .id(format!("session-action-{name}"))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_action(this, name, cx);
                }))
                .child(action(name, name == "Extend"))
        };

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
                .child(session_action_button("Extend", cx))
                .child(session_action_button("Pause / Resume", cx))
                .child(session_action_button("Stop", cx)),
        )
    }
}
