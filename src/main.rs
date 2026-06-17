use gpui::*;
use gpui_component::Root;

// this is the only line changing between tests
use testing::test_1::main_window::MainWindow;

pub mod testing;

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                app_id: Some("TestApplication".into()),
                titlebar: if MainWindow::has_custom_titlebar() { None } else { Some(TitlebarOptions::default()) },
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(600.), px(400.))),
               ..Default::default() 
            };
            cx.open_window(window_options, |window, cx| {
                let main_window = cx.new(|cx| MainWindow::new(cx));
                cx.new(|cx| Root::new(main_window, window, cx))
            }).expect("Unable to open window");
        }).detach();
    });
}