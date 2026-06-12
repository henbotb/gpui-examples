#![allow(unused_variables, dead_code, unused_imports)]
use gpui::*;
use gpui_component::*;

// div()
//     .id() 
//     // 1. layout role
//     .flex()
//     .flex_col()
//     .justify_center()
//     .items_center()
//     // 2. sizing
//     .w_full()
//     .h(...)
//     .aspect_square()
//     // 3. spacing
//     .px_3()
//     .py_2()
//     .gap_2()
//     // 4. visual
//     .bg(...)
//     .border_b_1()
//     .border_color(...)
//     .rounded_sm()
//     // 5. interaction
//     .occlude()
//     .on_click(cx.listener(...))
//     .hover(...)
//     // 6. children last
//     .child(...)
//     .children(...)

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

