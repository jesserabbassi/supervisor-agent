use super::app_viewmodel::SupervisorViewModel;
use crate::alerts::views::AlertView;
use crate::customers::views::CustomerView;
use crate::dashboard::views::DashboardView;
use crate::monitoring::views::MonitoringView;
use crate::reports::views::ReportView;
use crate::reservations::views::ReservationView;
use crate::sessions::views::SessionView;
use crate::settings::views::SettingsView;
use crate::shared::navigation::Page;
use crate::shared::theme::*;
use crate::shared::ui::*;
use crate::stations::views::StationView;
use crate::wallet::views::WalletView;
use gpui::{
    div, prelude::*, px, rgb, Context, Div, FontWeight, InteractiveElement, IntoElement, Render,
    Window,
};

pub struct SupervisorApp {
    pub vm: SupervisorViewModel,
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
}

impl Render for SupervisorApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.vm.page {
            Page::Dashboard => DashboardView::render(
                &self.vm.dashboard,
                &self.vm.reservation.reservations,
                cx,
                |this, page, cx| {
                    this.vm.page = page;
                    cx.notify();
                },
                |this, idx, cx| {
                    this.vm.reservation.selected = idx;
                    this.vm.page = Page::Reservations;
                    cx.notify();
                },
            ),
            Page::Stations => {
                let mut filters = div().flex().gap_2();
                for name in ["All", "Available", "In Session", "Locked", "Offline"] {
                    let selected = self.vm.station.filter == name;
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
                for i in 0..self.vm.station.stations.len() {
                    let s = &self.vm.station.stations[i];
                    if self.vm.station.filter != "All"
                        && s.status.label() != self.vm.station.filter.to_uppercase()
                    {
                        continue;
                    }
                    cards = cards.child(StationView::card(
                        i,
                        &self.vm.station,
                        cx,
                        |this, idx, cx| {
                            this.vm.station.select_station(idx);
                            cx.notify();
                        },
                    ));
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
                    .child(StationView::detail(
                        &self.vm.station,
                        cx,
                        |this, action_name, cx| {
                            this.vm.station_action(action_name);
                            cx.notify();
                        },
                    ))
            }
            Page::Sessions => SessionView::render(
                &self.vm.session,
                cx,
                |this, idx, cx| {
                    this.vm.session.selected = idx;
                    cx.notify();
                },
                |this, action_name, cx| {
                    this.vm.notice = Some(match this.vm.session.control(action_name) {
                        Ok(()) => format!("{action_name} applied in demo mode"),
                        Err(error) => error,
                    });
                    cx.notify();
                },
            ),
            Page::Reservations => ReservationView::render(
                &self.vm.reservation,
                cx,
                |this, idx, cx| {
                    this.vm.reservation.selected = idx;
                    this.vm.page = Page::Reservations;
                    cx.notify();
                },
            ),
            Page::Customers => CustomerView::render(
                &self.vm.customer,
                cx,
                |this, idx, cx| {
                    this.vm.customer.selected = idx;
                    cx.notify();
                },
            ),
            Page::Wallet => WalletView::render(&self.vm.wallet),
            Page::Monitoring => MonitoringView::render(&self.vm.monitoring),
            Page::Alerts => AlertView::render(
                &self.vm.alerts,
                cx,
                |this, filter_name, cx| {
                    this.vm.alerts.filter = filter_name;
                    cx.notify();
                },
                |this, idx, cx| {
                    this.vm.alerts.mark_as_read(idx);
                    cx.notify();
                },
            ),
            Page::Reports => ReportView::render(),
            Page::Settings => SettingsView::render(),
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
