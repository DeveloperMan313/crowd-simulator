use gpui_kit::*;

use crate::features::{simulation::view::simulation_view, welcome::view::welcome_view};

#[derive(Copy, Clone)]
pub enum AppView {
    Welcome,
    Simulation,
}

pub struct AppViewChangeEvent {
    pub view: AppView,
}

pub struct RootView {
    view: AppView,
    _subscription: Subscription,
}

impl EventEmitter<AppViewChangeEvent> for RootView {}

impl RootView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe_self(|this, event, cx| {
            this.view = event.view;
            cx.notify();
        });

        Self {
            view: AppView::Welcome,
            _subscription: subscription,
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match self.view {
            AppView::Welcome => welcome_view(cx).into_any_element(),
            AppView::Simulation => simulation_view(cx).into_any_element(),
        })
    }
}
