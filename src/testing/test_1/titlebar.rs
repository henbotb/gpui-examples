#![allow(unused_variables, dead_code, unused_imports)]
use std::rc::Rc;
use gpui::{*, prelude::*};
use gpui_component::*;

use crate::testing::test_1::context_menu::ContextMenuEvent;

pub struct Titlebar {
}

impl Titlebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
        }
    }
}

impl EventEmitter<ContextMenuEvent> for Titlebar {}

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_hover: Hsla = cx.theme().title_bar.darken(0.2);

        let hover_modify = |color: &Hsla| -> Hsla {
            if cx.theme().is_dark() {
                color.lighten(0.2)
            } else {
                color.darken(0.2)
            }
        };

        let titlebar_hover = hover_modify(&cx.theme().title_bar);

        let settings_dropdown = div()
            .id("settings")
            .flex()
            .h_full()
            .justify_center()
            .items_center()
            .aspect_square()
            .occlude()
            .hover(|this| this.bg(titlebar_hover))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, window, cx| {
            }))
            .child(IconName::Settings);

        let theme_switcher = div()
            .id("theme-switcher")
            .flex()
            .h_full()
            .justify_center()
            .items_center()
            .aspect_square()
            .occlude()
            .hover(|this| this.bg(titlebar_hover))
            .on_click(|event, window, cx| {
                Theme::change(
                    if cx.theme().is_dark() {
                        ThemeMode::Light
                    } else {
                        ThemeMode::Dark
                    },
                    Some(window), cx);
            })
            .child(
                if cx.theme().is_dark() {
                    IconName::Moon
                } else {
                    IconName::Sun
                }
            );

        let history_nav = div()
            .id("nav-directions")
            .flex()
            .flex_row()
            .child(div()
                .id("nav-backwards")
                .flex()
                .h_full()
                .justify_center()
                .items_center()
                .aspect_square()
                .occlude()
                .hover(|this| this.bg(titlebar_hover))
                .on_click(|event, window, cx| {
                    println!("Navigate Left");
                })
                .child(IconName::ArrowLeft)
            )
            .child(div()
                .id("nav-forwards")
                .flex()
                .h_full()
                .justify_center()
                .items_center()
                .aspect_square()
                .occlude()
                .hover(|this| this.bg(titlebar_hover))
                .on_click(|event, window, cx| {
                    println!("Navigate Right")
                })
                .child(IconName::ArrowRight)
            );
        
        let window_controls = if cfg!(target_os = "macos") {
            div()
                .id("window-controls")
                .flex()
                .flex_row()
                .gap_2()
                .px_2()
                .items_center()
                .child(div()
                    .id("window-minimize")
                    .flex()
                    .size(px(12.))
                    .rounded_full()
                    .bg(rgb(0xf6be50))
                    .border_1()
                    .border_color(rgb(0xe1a73e))
                    .window_control_area(WindowControlArea::Min)
                    .occlude()
                    .hover(|this| this.border_color(rgb(0x90591d)))
                    .on_click(|_, window, _| {
                        window.minimize_window();
                    })
                )
                .child(div()
                    .id("window-zoom")
                    .flex()
                    .size(px(12.))
                    .rounded_full()
                    .bg(rgb(0x61c555))
                    .border_1()
                    .border_color(rgb(0x2dac2f))
                    .window_control_area(WindowControlArea::Max)
                    .occlude()
                    .hover(|this| this.border_color(rgb(0x2a6128)))
                    .on_click(|_, window, _| {
                        window.zoom_window();
                    })
                )
                .child(div()
                    .id("window-close")
                    .flex()
                    .size(px(12.))
                    .rounded_full()
                    .bg(rgb(0xed6a5f))
                    .border_1()
                    .border_color(rgb(0xe24b41))
                    .window_control_area(WindowControlArea::Close)
                    .occlude()
                    .hover(|this| this.border_color(rgb(0x460804)))
                    .on_click(|_, window, _| {
                        window.remove_window();
                    })
                )
        } else {
            div()
                .id("window-controls")
                .flex()
                .flex_row()
                .child(div()
                    .id("window-minimize")
                    .flex()
                    .h_full()
                    .aspect_square()
                    .items_center()
                    .justify_center()
                    .occlude()
                    .window_control_area(WindowControlArea::Min)
                    .hover(|this| this.bg(titlebar_hover))
                    .on_click(|event, window, cx| {
                        window.minimize_window();
                    })
                    .child(IconName::WindowMinimize)
                )
                .child(
                    if window.is_maximized() {
                        div().id("window-restore").child(IconName::WindowRestore)
                    } else {
                        div().id("window-maximize").child(IconName::WindowMaximize)
                    }
                    .flex()
                    .h_full()
                    .aspect_square()
                    .items_center()
                    .justify_center()
                    .occlude()
                    .window_control_area(WindowControlArea::Max)
                    .hover(|this| this.bg(titlebar_hover))
                    .on_click(|event, window, cx| {
                        window.zoom_window();
                    })
                )
                .child(div()
                    .id("window-close")
                    .flex()
                    .h_full()
                    .aspect_square()
                    .items_center()
                    .justify_center()
                    .occlude()
                    .window_control_area(WindowControlArea::Close)
                    .hover(|this| this.bg(cx.theme().danger))
                    .on_click(|event, window, cx| {
                        window.remove_window();
                    })
                    .child(IconName::WindowClose)
                )
        };
        
        div()
            .id("titlebar")
            .flex()
            .flex_row()
            .justify_between()
            .w_full()
            .h_8()
            .bg(cx.theme().title_bar)
            .window_control_area(WindowControlArea::Drag)
            .on_mouse_down(MouseButton::Left, |_, window, _| {
                window.start_window_move();
            })
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                cx.emit(ContextMenuEvent::Open { 
                    position: event.position, 
                    build: Rc::new(|_, _| {
                        div()
                            .child("Do something titlebar")
                            .child("Do something else page")
                    })
                });
            }))
            .child(div()
                .flex()
                .flex_row()
                .child(settings_dropdown)
                .child(history_nav)
                .child(theme_switcher)
            )
            .child(window_controls)
    }
}