use gpui_kit::*;

use crate::{
    app::root::RootView,
    features::simulation::components::{
        simulation_panel::simulation_panel, simulation_viewport::simulation_viewport,
    },
};

pub fn simulation_view(cx: &mut Context<RootView>) -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .child(simulation_panel(cx))
        .child(simulation_viewport())
}
