use std::rc::Rc;
use gpui::*;
use gpui_component::*;

use crate::testing::test_1::context_menu::ContextMenuEvent;

pub struct Page {
}

impl Page {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
        }
    }
}

impl EventEmitter<ContextMenuEvent> for Page {}

impl Render for Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main-page")
            .size_full()
            .bg(cx.theme().primary_foreground)
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                println!("Trying to open context menu page");
                cx.emit(ContextMenuEvent::Open { 
                    position: event.position, 
                    build: Rc::new(|_, _| {
                        div()
                            .child("Do something page")
                            .child("Do something else page")
                    })
                })
            }))
    }
}

