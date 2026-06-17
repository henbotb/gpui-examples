use gpui::*;
use gpui_component::*;

pub struct Page {
    cx_menu_pos: Option<Point<Pixels>>,
}

impl Page {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            cx_menu_pos: None,
        }
    }
}

impl Render for Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main-page")
            .size_full()
            .bg(cx.theme().primary_foreground)
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                this.cx_menu_pos = Some(event.position);
                cx.notify();
            }))
            .children(self.cx_menu_pos.map(|pos| {
                let cx_menu = div()
                    .flex()
                    .flex_col()
                    .min_w_40()
                    .p_1p5()
                    .gap_y_0p5()
                    .border_1()
                    .rounded_sm()
                    .bg(cx.theme().title_bar)
                    .border_color(cx.theme().title_bar_border)
                    .occlude()
                    .children((0..10).map(|num| div()
                        .id(format!("sidebar-cx-{}", num))
                        .px_2()
                        .py_1()
                        .text_sm()
                        .rounded_xs()
                        .hover(|this| this.bg(cx.theme().sidebar_accent))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            println!("Clicked sidebar item {}", num);
                            this.cx_menu_pos = None;
                            cx.notify();
                        }))
                        .child(format!("Page {}", num))
                    ));
                
                let window_size = window.viewport_size();
                deferred(div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .w(window_size.width)
                    .h(window_size.height)
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                        this.cx_menu_pos = None;
                        cx.notify();
                    }))
                    .on_mouse_down(MouseButton::Right, cx.listener(|this, _, _, cx| {
                        this.cx_menu_pos = None;
                        cx.notify();
                    }))
                    .child(anchored()
                        .position(pos)
                        .child(cx_menu)
                    )
                )
                .with_priority(1)
            }))
    }
}

