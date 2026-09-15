use gpui::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e3440))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xeceff4))
            .child("Hello, GPUI!")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld)
        })
        .unwrap();
    });
}
