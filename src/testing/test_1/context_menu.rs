use std::rc::Rc;
use gpui::*;
use gpui_component::*;

pub enum ContextMenuEvent {
    Open { 
        position: Point<Pixels>, 
        build: Rc<dyn Fn(&mut Window, &mut App) -> Div>,
    },
    Close,
}

pub struct ContextMenu {
    pub position: Point<Pixels>,
    content: Rc<dyn Fn(&mut Window, &mut App) -> Div>,
}

impl EventEmitter<ContextMenuEvent> for ContextMenu {}

impl ContextMenu {
    pub fn new(position: Point<Pixels>, content: Rc<dyn Fn(&mut Window, &mut App) -> Div>, cx: &mut Context<Self>) -> Self {
        Self {
            position,
            content,
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("context-menu")
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
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(ContextMenuEvent::Close);
            }))
            .child((self.content)(window, cx))
    }
}