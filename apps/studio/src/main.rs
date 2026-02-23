use gpui::*;

struct StudioApp;

impl Render for StudioApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .child("Hello from GPUI Workbench")
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |_window, cx| {
                cx.new(|_cx| StudioApp)
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
