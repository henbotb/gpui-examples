use std::rc::Rc;
use gpui::{*, prelude::*};
use gpui_component::*;

use crate::testing::test_1::context_menu::ContextMenuEvent;

pub struct Sidebar {
    width: f32,
}

impl Sidebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            width: 212.,
        }
    }
}

impl EventEmitter<ContextMenuEvent> for Sidebar {}

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
                cx.emit(ContextMenuEvent::Open { 
                    position: event.position, 
                    build: Rc::new(|_, _| {
                        div()
                            .child("Do something sidebar")
                            .child("Do something else page")
                    })
                });
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
    }
}