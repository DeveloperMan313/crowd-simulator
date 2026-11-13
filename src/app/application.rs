use gpui_kit::*;

use crate::app::root::RootView;

pub fn run() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(RootView::new))
                .expect("failed to open window");
        });
}
