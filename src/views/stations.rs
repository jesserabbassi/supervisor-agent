use super::*;

pub(super) struct StationView;
impl StationView {
    pub(super) fn render(app: &SupervisorApp, cx: &mut Context<SupervisorApp>) -> Div {
        let mut filters = div().flex().gap_2();
        for name in ["All", "Available", "In Session", "Locked", "Offline"] {
            let selected = app.vm.station.filter == name;
            filters = filters.child(
                div()
                    .id(format!("filter-{name}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.station.filter = name;
                        cx.notify();
                    }))
                    .child(badge(name, if selected { ORANGE } else { BORDER })),
            );
        }
        let mut cards = div().flex().flex_wrap().gap_3();
        for i in 0..app.vm.station.stations.len() {
            let s = &app.vm.station.stations[i];
            if app.vm.station.filter != "All"
                && s.status.label() != app.vm.station.filter.to_uppercase()
            {
                continue;
            }
            cards = cards.child(app.station_card(i, cx));
        }
        div()
            .flex()
            .gap_4()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(filters)
                    .child(cards),
            )
            .child(app.station_detail(cx))
    }
}
