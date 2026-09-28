use crate::shared::theme::*;
use crate::shared::ui::*;
use gpui::{div, prelude::*, px, Div};

pub struct SettingsView;

impl SettingsView {
    pub fn render() -> Div {
        div()
            .flex()
            .gap_4()
            .child(
                panel()
                    .w(px(200.))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(badge("General", ORANGE))
                    .child(label("Profile"))
                    .child(label("Notifications"))
                    .child(label("Security"))
                    .child(label("Appearance"))
                    .child(label("System"))
                    .child(label("Backup & Data"))
                    .child(label("About")),
            )
            .child(
                panel()
                    .flex_1()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(section("General Settings"))
                    .child(label("Gaming House Name"))
                    .child(badge("Ninety Gaming House", BLUE))
                    .child(label("Branch Location"))
                    .child(badge("Downtown Branch", BLUE))
                    .child(label("Timezone"))
                    .child(badge("(UTC+01:00) Tunis", BLUE))
                    .child(label("Currency"))
                    .child(badge("EUR (€)", BLUE))
                    .child(action("Save Changes", true)),
            )
            .child(
                panel()
                    .w(px(290.))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(section("Application Preferences"))
                    .child(label("● Auto-refresh data"))
                    .child(label("● Show online status"))
                    .child(label("● Enable sound notifications"))
                    .child(label("● Show station thumbnails"))
                    .child(section("Data Management"))
                    .child(action("Export All Data", false))
                    .child(action("Clear Cache", false)),
            )
    }
}
