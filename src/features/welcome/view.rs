use gpui_kit::{
    base::plot::label::TEXT_SIZE,
    component::{
        button::{Button, ButtonVariants},
        label::Label,
    },
    *,
};

use crate::app::root::{AppView::Simulation, AppViewChangeEvent, RootView};

pub fn welcome_view(cx: &mut Context<RootView>) -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .justify_center()
        .items_center()
        .gap_4()
        .child(Label::new("Crowd simulator").text_size(px(TEXT_SIZE * 2.0)))
        .child(
            Button::new("new-simulation")
                .primary()
                .label("New simulation")
                .cursor_pointer()
                .on_click(cx.listener(|_, _, _, cx| {
                    cx.emit(AppViewChangeEvent { view: Simulation });
                })),
        )
}
