use gpui_kit::base::v_flex;
use gpui_kit::{IntoElement, *};
use gpui_kit::{
    base::{StyledExt, h_flex},
    component::{
        ActiveTheme, IconName,
        button::{Button, ButtonVariants},
        label::Label,
    },
};

use crate::app::root::{AppView::Welcome, AppViewChangeEvent, RootView};

pub fn simulation_panel(cx: &mut Context<RootView>) -> impl IntoElement {
    let simulation_time_s = 15.5;

    div()
        .w_64()
        .h_full()
        .p(cx.theme().spacing_tokens().md)
        .flex()
        .flex_col()
        .gap_4()
        .bg(cx.theme().sidebar)
        .border_1()
        .border_color(cx.theme().border)
        .child(
            div()
                .h_flex()
                .gap_2()
                .child(
                    Button::new("to-welcome-view")
                        .ghost()
                        .icon(IconName::ArrowLeft)
                        .accessibility_label("Back to welcome screen")
                        .tooltip("Back to welcome screen")
                        .cursor_pointer()
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(AppViewChangeEvent { view: Welcome });
                        })),
                )
                .child(Label::new("Simulation").text_lg()),
        )
        .child(
            h_flex()
                .gap_1()
                .child(Label::new("Time:").text_color(cx.theme().muted_foreground))
                .child(Label::new(format!("{simulation_time_s}")))
                .child(Label::new("s").text_color(cx.theme().muted_foreground)),
        )
        .child(
            v_flex()
                .gap_2()
                .child(
                    Button::new("start-simulation")
                        .primary()
                        .icon(IconName::Play)
                        .label("Start")
                        .tooltip("Start simulation")
                        .cursor_pointer(),
                )
                .child(
                    Button::new("pause-simulation")
                        .secondary()
                        .icon(IconName::Pause)
                        .label("Pause")
                        .tooltip("Pause simulation")
                        .cursor_pointer(),
                ),
        )
}
