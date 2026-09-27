use super::*;

pub(super) struct WalletView;
impl WalletView {
    pub(super) fn render(app: &SupervisorApp) -> Div {
        let mut table = panel().p_4().child(section("Transactions")).child(
            row()
                .child(column("ID", 95.))
                .child(column("Customer", 190.))
                .child(column("Type", 90.))
                .child(column("Amount", 90.))
                .child(column("Method", 140.)),
        );
        for t in &app.vm.wallet.transactions {
            table = table.child(
                row()
                    .child(column(&t.id, 95.))
                    .child(column(&t.customer, 190.))
                    .child(column(&t.kind, 90.))
                    .child(
                        div()
                            .w(px(90.))
                            .text_color(rgb(if t.amount >= 0. { GREEN } else { RED }))
                            .child(format!("{:+.2} €", t.amount)),
                    )
                    .child(column(&t.method, 140.)),
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
                    .child(stat(
                        "Total Wallet Balance",
                        "2,145.50 €",
                        BLUE,
                        "all customers",
                    ))
                    .child(stat("Top Up This Week", "315.50 €", GREEN, "↑ 12%"))
                    .child(stat("Transactions", "128", BLUE, "↑ 8%"))
                    .child(stat("Active Wallets", "428", YELLOW, "↑ 5%")),
            )
            .child(
                div().flex().gap_4().child(table).child(
                    panel()
                        .w(px(280.))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(section("Wallet Actions"))
                        .child(action("Add Funds to Customer", true))
                        .child(action("Deduct Amount", false))
                        .child(action("Transfer Between Customers", false))
                        .child(action("View Transaction History", false))
                        .child(section("Payment Methods"))
                        .child(label("Credit / Debit Card  ·  Cash"))
                        .child(label("Mobile Payment  ·  Gift Card")),
                ),
            )
    }
}
