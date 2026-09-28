use super::viewmodels::CustomerViewModel;
use crate::shared::theme::*;
use crate::shared::ui::*;
use gpui::{div, prelude::*, px, rgb, Context, Div};

pub struct CustomerView;

impl CustomerView {
    pub fn render<T: 'static>(
        customer_vm: &CustomerViewModel,
        cx: &mut Context<T>,
        on_select: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> Div {
        let mut table = panel().flex_1().child(
            row()
                .child(column("ID", 70.))
                .child(column("Customer", 150.))
                .child(column("Membership", 100.))
                .child(column("Wallet", 80.))
                .child(column("Status", 90.)),
        );
        for (i, c) in customer_vm.customers.iter().enumerate() {
            table = table.child(
                row()
                    .id(format!("customer-{i}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        on_select(this, i, cx);
                    }))
                    .bg(rgb(if customer_vm.selected == i {
                        PANEL2
                    } else {
                        PANEL
                    }))
                    .child(column(&c.id, 70.))
                    .child(column(&c.name, 150.))
                    .child(column(&c.membership, 100.))
                    .child(column(format!("{:.2} €", c.balance), 80.))
                    .child(badge(
                        if c.online { "Online" } else { "Offline" },
                        if c.online { GREEN } else { MUTED },
                    )),
            );
        }
        let c = &customer_vm.customers[customer_vm.selected];
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(stat("Total Customers", "428", BLUE, "all branches"))
                    .child(stat("Active Customers", "56", GREEN, "currently online"))
                    .child(stat("New This Month", "12", BLUE, "recent signups"))
                    .child(stat("VIP Members", "18", YELLOW, "premium members")),
            )
            .child(
                div().flex().gap_4().child(table).child(
                    panel()
                        .w(px(300.))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(section("Customer Details"))
                        .child(heading(&c.name))
                        .child(label(&c.email))
                        .child(badge(&c.membership, YELLOW))
                        .child(stat(
                            "Wallet Balance",
                            format!("{:.2} €", c.balance),
                            YELLOW,
                            "available",
                        ))
                        .child(label(format!("Member ID: {}", c.id)))
                        .child(action("Add Funds", true))
                        .child(action("Make Reservation", false)),
                ),
            )
    }
}
