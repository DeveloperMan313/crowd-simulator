use gpui_kit::{
    base::plot::label::TEXT_SIZE,
    component::{
        button::{Button, ButtonVariants},
        label::Label,
    },
    *,
};

use crate::app::root::{AppView::Welcome, AppViewChangeEvent, RootView};

pub fn simulation_view(cx: &mut Context<RootView>) -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .justify_center()
        .items_center()
        .gap_4()
        .child(Label::new("TODO simulation").text_size(px(TEXT_SIZE * 2.0)))
        .child(
            Button::new("to-welcome-view")
                .primary()
                .label("Back")
                .cursor_pointer()
                .on_click(cx.listener(|_, _, _, cx| {
                    cx.emit(AppViewChangeEvent { view: Welcome });
                })),
        )
}
