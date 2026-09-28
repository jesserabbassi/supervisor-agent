use super::models::StationStatus;
use crate::shared::theme::*;
use crate::shared::ui::*;
use gpui::{div, prelude::*, px, rgb, Context, Div, FontWeight, IntoElement};

pub fn mini_station(id: &str, status: StationStatus) -> Div {
    panel()
        .p_3()
        .w(px(126.))
        .h(px(70.))
        .flex()
        .flex_col()
        .justify_between()
        .border_color(rgb(status.color()))
        .child(
            div()
                .text_color(rgb(TEXT))
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .child(id.to_string()),
        )
        .child(
            div()
                .text_color(rgb(status.color()))
                .text_xs()
                .child(format!("● {}", status.label())),
        )
}

pub struct StationView;

impl StationView {
    pub fn card<T: 'static>(
        index: usize,
        station_vm: &super::viewmodels::StationViewModel,
        cx: &mut Context<T>,
        on_select: impl Fn(&mut T, usize, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> impl IntoElement {
        let s = &station_vm.stations[index];
        let selected = index == station_vm.selected;
        let c = s.status.color();
        panel()
            .id(format!("station-{}", index))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                on_select(this, index, cx);
            }))
            .w(px(218.))
            .h(px(198.))
            .p_3()
            .flex()
            .flex_col()
            .justify_between()
            .border_color(rgb(if selected { ORANGE } else { c }))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(heading(&s.id))
                    .child(div().text_color(rgb(MUTED)).child("›")),
            )
            .child(badge(s.status.label(), c))
            .child(if s.customer.is_empty() {
                label("Gaming station ready")
            } else {
                label(&s.customer)
            })
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(label(format!("CPU {}%", s.cpu)))
                    .child(label(format!("GPU {}%", s.gpu))),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(label(format!("RAM {}%", s.ram)))
                    .child(label(format!("{}°C", s.temp))),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(div().text_color(rgb(GREEN)).text_xs().child("● LAN"))
                    .child(
                        div()
                            .text_color(rgb(BLUE))
                            .text_xs()
                            .child("View details  →"),
                    ),
            )
    }

    pub fn detail<T: 'static>(
        station_vm: &super::viewmodels::StationViewModel,
        cx: &mut Context<T>,
        on_action: impl Fn(&mut T, &'static str, &mut Context<T>) + 'static + Send + Sync + Copy,
    ) -> Div {
        let s = &station_vm.stations[station_vm.selected];
        let mut box_ = panel()
            .w(px(300.))
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(heading(&s.id))
                    .child(badge(s.status.label(), s.status.color())),
            )
            .child(
                div()
                    .h(px(130.))
                    .rounded_md()
                    .bg(rgb(0x102c42))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(BLUE))
                    .text_3xl()
                    .child("▣  GAMING STATION"),
            )
            .child(section("Overview"))
            .child(label(if s.customer.is_empty() {
                "No active customer"
            } else {
                &s.customer
            }))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(stat("CPU", format!("{}%", s.cpu), BLUE, "usage"))
                    .child(stat("GPU", format!("{}%", s.gpu), ORANGE, "usage")),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(stat("RAM", format!("{}%", s.ram), BLUE, "usage"))
                    .child(stat("TEMP", format!("{}°", s.temp), YELLOW, "celsius")),
            )
            .child(section("Quick Actions"));
        for (name, primary) in [
            ("Lock", true),
            ("Unlock", false),
            ("Restart", false),
            ("Shutdown", false),
        ] {
            box_ = box_.child(
                div()
                    .id(format!("station-action-{name}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        on_action(this, name, cx);
                    }))
                    .child(action(name, primary)),
            );
        }
        box_
    }
}
