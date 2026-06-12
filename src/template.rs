#![allow(unused_variables, dead_code, unused_imports)]
use gpui::*;
use gpui_component::*;

// div()
//     // 1. layout role
//     .flex()
//     .flex_col()
//     // 2. sizing
//     .w_full()
//     .h(px(48.))
//     // 3. spacing
//     .px_3()
//     .py_2()
//     .gap_2()
//     // 4. visual
//     .bg(cx.theme().colors().surface)
//     .border_b_1()
//     .border_color(cx.theme().colors().border)
//     // 5. interaction
//     .on_click(cx.listener(...))
//     // 6. children last
//     .child(...)

pub struct MainWindow {
    
}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            
        }
    }

    pub fn has_custom_titlebar() -> bool {
        false
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main-window")
            .bg(rgb(0x999999))
            .size_full()
    }
}

