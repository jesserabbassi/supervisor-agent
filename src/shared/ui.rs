use super::theme::*;
use gpui::{
    div, prelude::*, px, rgb, Div, FontWeight,
};

pub fn panel() -> Div {
    div()
        .bg(rgb(PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded_md()
}

pub fn label(text: impl ToString) -> Div {
    div()
        .text_color(rgb(MUTED))
        .text_sm()
        .child(text.to_string())
}

pub fn heading(text: impl ToString) -> Div {
    div()
        .text_color(rgb(TEXT))
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.to_string())
}

pub fn stat(title: &str, value: impl ToString, accent: u32, foot: &str) -> Div {
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

pub fn badge(text: impl ToString, color: u32) -> Div {
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

pub fn action(text: impl ToString, primary: bool) -> Div {
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

pub fn section(title: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(heading(title))
}

pub fn row() -> Div {
    div()
        .flex()
        .items_center()
        .gap_3()
        .px_3()
        .py_3()
        .border_b_1()
        .border_color(rgb(BORDER))
}

pub fn column(text: impl ToString, width: f32) -> Div {
    div()
        .w(px(width))
        .text_color(rgb(TEXT))
        .text_sm()
        .child(text.to_string())
}
