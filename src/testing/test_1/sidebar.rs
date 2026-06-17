use gpui::{*, prelude::*};
use gpui_component::*;

pub struct Sidebar {
    width: f32,
    cx_menu_pos: Option<Point<Pixels>>,
}

impl Sidebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            width: 212.,
            cx_menu_pos: None,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("sidebar")
            .flex()
            .flex_row()
            .size_full()
            .w(px(self.width))
            .bg(cx.theme().sidebar)
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                this.cx_menu_pos = Some(event.position);
                cx.notify();
            }))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.cx_menu_pos = None;
                cx.notify();
            }))
            .child(div()
                .id("sidebar-main")
                .flex()
                .flex_grow()
            )
            .child(div()
                .id("sidebar-drag-handle")
                .w_0p5()
                .m_1p5()
                .bg(cx.theme().sidebar_border)
                .cursor_col_resize()
                .on_drag_move(move |event: &DragMoveEvent<Sidebar>, window, cx| {
                    
                })
            )
            .when_some(self.cx_menu_pos, |this, pos| {
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
                    .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                        this.cx_menu_pos = None;
                        cx.notify();
                    }))
                    .occlude()
                    .children((0..5).map(|num| div()
                        .id(format!("titlebar-cx-{}", num))
                        .px_2()
                        .py_1()
                        .text_sm()
                        .rounded_sm()
                        .hover(|this| this.bg(cx.theme().primary_foreground))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            println!("Clicked menu item {}", num);
                            this.cx_menu_pos = None;
                            cx.notify(); // not sure why this isnt necessary
                        }))
                        .child(format!("Sidebar {}", num))
                    ));
                this.child(
                    deferred(anchored()
                        .anchor(Anchor::TopLeft)
                        .position(pos)
                        .snap_to_window_with_margin(px(8.))
                        .child(cx_menu)
                    )
                )
            })
    }
}