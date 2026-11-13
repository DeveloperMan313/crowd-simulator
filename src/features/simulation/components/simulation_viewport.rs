use gpui_kit::*;

pub fn simulation_viewport() -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .justify_center()
        .items_center()
        .text_lg()
        .child("TODO simulation viewport")
}
