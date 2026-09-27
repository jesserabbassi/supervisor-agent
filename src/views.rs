use crate::services::StationStatus;
mod dashboard;
use dashboard::DashboardView;
mod stations;
use stations::StationView;
mod sessions;
use sessions::SessionView;
mod reservations;
use reservations::ReservationView;
mod customers;
use customers::CustomerView;
mod wallet;
use wallet::WalletView;
mod monitoring;
use monitoring::MonitoringView;
mod alerts;
use alerts::AlertView;
mod reports;
use reports::ReportView;
mod settings;
use crate::viewmodels::{Page, SupervisorViewModel};
use gpui::{
    div, prelude::*, px, rgb, Context, Div, FontWeight, InteractiveElement, IntoElement, Render,
    Window,
};
use settings::SettingsView;

const BG: u32 = 0x031522;
const PANEL: u32 = 0x071e30;
const PANEL2: u32 = 0x0a293e;
const BORDER: u32 = 0x164660;
const TEXT: u32 = 0xf2f7ff;
const MUTED: u32 = 0x9bb2c5;
const BLUE: u32 = 0x16b9f5;
const ORANGE: u32 = 0xff5a24;
const GREEN: u32 = 0x00d69b;
const RED: u32 = 0xff4a55;
const YELLOW: u32 = 0xffa229;

fn panel() -> Div {
    div()
        .bg(rgb(PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded_md()
}
fn label(text: impl ToString) -> Div {
    div()
        .text_color(rgb(MUTED))
        .text_sm()
        .child(text.to_string())
}
fn heading(text: impl ToString) -> Div {
    div()
        .text_color(rgb(TEXT))
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.to_string())
}
fn stat(title: &str, value: impl ToString, accent: u32, foot: &str) -> Div {
    panel()
        .flex_1()
        .min_w(px(160.))
        .p_4()
        .flex()
        .flex_col()
        .gap_2()
        .child(label(title))
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(accent))
                .child(value.to_string()),
        )
        .child(label(foot))
}
fn badge(text: impl ToString, color: u32) -> Div {
    div()
        .px_2()
        .py_1()
        .rounded_sm()
        .bg(rgb(PANEL2))
        .border_1()
        .border_color(rgb(color))
        .text_color(rgb(color))
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.to_string())
}
fn action(text: impl ToString, primary: bool) -> Div {
    div()
        .px_3()
        .py_2()
        .rounded_sm()
        .border_1()
        .border_color(rgb(if primary { ORANGE } else { BORDER }))
        .bg(rgb(if primary { ORANGE } else { PANEL2 }))
        .text_color(rgb(TEXT))
        .text_sm()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.to_string())
}
fn section(title: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(heading(title))
}
fn status_color(status: StationStatus) -> u32 {
    match status {
        StationStatus::Available => GREEN,
        StationStatus::InSession => YELLOW,
        StationStatus::Locked => RED,
        StationStatus::Offline => MUTED,
        StationStatus::Maintenance => YELLOW,
    }
}
fn row() -> Div {
    div()
        .flex()
        .items_center()
        .gap_3()
        .px_3()
        .py_3()
        .border_b_1()
        .border_color(rgb(BORDER))
}
fn column(text: impl ToString, width: f32) -> Div {
    div()
        .w(px(width))
        .text_color(rgb(TEXT))
        .text_sm()
        .child(text.to_string())
}
fn mini_station(id: &str, status: StationStatus) -> Div {
    panel()
        .p_3()
        .w(px(126.))
        .h(px(70.))
        .flex()
        .flex_col()
        .justify_between()
        .border_color(rgb(status_color(status)))
        .child(
            div()
                .text_color(rgb(TEXT))
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .child(id.to_string()),
        )
        .child(
            div()
                .text_color(rgb(status_color(status)))
                .text_xs()
                .child(format!("● {}", status.label())),
        )
}

pub struct SupervisorApp {
    vm: SupervisorViewModel,
}
impl SupervisorApp {
    pub fn new() -> Self {
        Self {
            vm: SupervisorViewModel::new(),
        }
    }
    fn sidebar(&self, cx: &mut Context<Self>) -> Div {
        let mut nav = div().flex().flex_col().gap_1();
        for page in Page::ALL {
            let active = self.vm.page == page;
            nav = nav.child(
                div()
                    .id(format!("nav-{}", page.label()))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.page = page;
                        this.vm.notice = None;
                        cx.notify();
                    }))
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_3()
                    .bg(rgb(if active { 0x222029 } else { BG }))
                    .border_l_4()
                    .border_color(rgb(if active { ORANGE } else { BG }))
                    .text_color(rgb(if active { ORANGE } else { TEXT }))
                    .child(div().text_lg().child(page.icon()))
                    .child(div().text_sm().child(page.label())),
            );
        }
        div()
            .w(px(218.))
            .h_full()
            .bg(rgb(0x041320))
            .flex()
            .flex_col()
            .justify_between()
            .border_r_1()
            .border_color(rgb(BORDER))
            .child(nav)
            .child(
                div()
                    .p_6()
                    .text_color(rgb(TEXT))
                    .text_lg()
                    .child("MORE\nTHAN A\nGAME"),
            )
    }
    fn topbar(&self) -> Div {
        div()
            .h(px(72.))
            .w_full()
            .px_6()
            .flex()
            .items_center()
            .justify_between()
            .bg(rgb(0x041a2b))
            .border_b_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(TEXT))
                            .child("◈  NINETY"),
                    )
                    .child(
                        div()
                            .border_l_1()
                            .border_color(rgb(BORDER))
                            .pl_4()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(TEXT))
                                    .child("SUPERVISOR APPLICATION"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(MUTED))
                                    .child("MANAGE  •  PLAY  •  BELONG"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(badge("⌖  Downtown Branch", BLUE))
                    .child(badge("●  Demo mode", YELLOW))
                    .child(badge("◯  Admin · Supervisor", BLUE)),
            )
    }
    fn title(&self) -> Div {
        div()
            .mb_5()
            .flex()
            .justify_between()
            .items_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(TEXT))
                            .child(self.vm.page.label()),
                    )
                    .child(label(self.vm.page.subtitle())),
            )
            .child(
                div()
                    .text_color(rgb(MUTED))
                    .text_sm()
                    .child("NINETY GAMING HOUSE  /  SUPERVISOR"),
            )
    }
    fn station_card(&self, index: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let s = &self.vm.station.stations[index];
        let selected = index == self.vm.station.selected;
        let c = status_color(s.status);
        panel()
            .id(format!("station-{}", index))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.vm.station.select_station(index);
                cx.notify();
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
    fn station_detail(&self, cx: &mut Context<Self>) -> Div {
        let s = &self.vm.station.stations[self.vm.station.selected];
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
                    .child(badge(s.status.label(), status_color(s.status))),
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
                        this.vm.station_action(name);
                        cx.notify();
                    }))
                    .child(action(name, primary)),
            );
        }
        box_
    }

    fn nav_action(
        &self,
        text: &'static str,
        page: Page,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(format!("quick-{text}"))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.vm.page = page;
                cx.notify();
            }))
            .child(action(text, false))
    }

    fn session_action(&self, name: &'static str, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(format!("session-action-{name}"))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.vm.notice = Some(match this.vm.session.control(name) {
                    Ok(()) => format!("{name} applied in demo mode"),
                    Err(error) => error,
                });
                cx.notify();
            }))
            .child(action(name, name == "Extend"))
    }
    fn reservation_rows(&self, limit: usize, cx: &mut Context<Self>) -> Div {
        let mut rows = div().flex().flex_col();
        for (i, r) in self
            .vm
            .reservation
            .reservations
            .iter()
            .take(limit)
            .enumerate()
        {
            rows = rows.child(
                row()
                    .id(format!("reservation-{i}"))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.vm.reservation.selected = i;
                        this.vm.page = Page::Reservations;
                        cx.notify();
                    }))
                    .child(column(&r.customer, 160.))
                    .child(column(&r.pc, 70.))
                    .child(column(&r.time, 120.))
                    .child(badge(
                        &r.status,
                        if r.status == "Confirmed" {
                            GREEN
                        } else {
                            YELLOW
                        },
                    )),
            );
        }
        rows
    }
}

impl Render for SupervisorApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.vm.page {
            Page::Dashboard => DashboardView::render(self, cx),
            Page::Stations => StationView::render(self, cx),
            Page::Sessions => SessionView::render(self, cx),
            Page::Reservations => ReservationView::render(self, cx),
            Page::Customers => CustomerView::render(self, cx),
            Page::Wallet => WalletView::render(self),
            Page::Monitoring => MonitoringView::render(self),
            Page::Alerts => AlertView::render(self, cx),
            Page::Reports => ReportView::render(self),
            Page::Settings => SettingsView::render(self),
        };
        div()
            .size_full()
            .font_family("IBM Plex Sans")
            .bg(rgb(BG))
            .text_color(rgb(TEXT))
            .flex()
            .flex_col()
            .child(self.topbar())
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .child(self.sidebar(cx))
                    .child(
                        div()
                            .id("content-scroll")
                            .flex_1()
                            .min_w_0()
                            .overflow_y_scroll()
                            .p_6()
                            .child(self.title())
                            .child(if let Some(notice) = &self.vm.notice {
                                badge(notice, BLUE)
                            } else {
                                div()
                            })
                            .child(content),
                    ),
            )
    }
}
