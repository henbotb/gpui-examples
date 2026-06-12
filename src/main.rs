use gpui::*;
use gpui_component::Root;

// this is the only line changing between tests
use testing::test_1::main::MainWindow;

pub mod testing;

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    let (sx, sy) = (1920., 1080.);
    let (wx_d, wy_d) = (1200., 800.);
    let (wx_m, wy_m) = (600., 400.);

    app.run(move |cx| {
        gpui_component::init(cx);
        cx.spawn(async move |cx| {
            let window_bounds = WindowBounds::Windowed(Bounds::new(Point::new(px(sx / 2. - wx_d / 2.), px(sy / 2. - wy_d / 2.)), Size::new(px(wx_d), px(wy_d))));
            let window_min_size = Size::new(px(wx_m), px(wy_m));
            let window_options = WindowOptions {
                app_id: Some("TestApplication".into()),
                titlebar: if MainWindow::has_custom_titlebar() { None } else { Some(TitlebarOptions::default()) },
                window_bounds: Some(window_bounds),
                window_min_size: Some(window_min_size),
               ..Default::default() 
            };
            cx.open_window(window_options, |window, cx| {
                let main_window = cx.new(|cx| MainWindow::new(cx));
                cx.new(|cx| Root::new(main_window, window, cx))
            }).expect("Unable to open window");
        }).detach();
    });
}