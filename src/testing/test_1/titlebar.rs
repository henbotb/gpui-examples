#![allow(unused_variables, dead_code, unused_imports)]
use gpui::{*, prelude::*};
use gpui_component::*;

pub struct Titlebar {
    context_menu_pos: Option<Point<Pixels>>,
    titlebar_dropdown: Option<TitlebarDropdown>,
}

impl Titlebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            context_menu_pos: None,
            titlebar_dropdown: None,
        }
    }
}

#[derive(Clone, Copy)]
enum TitlebarDropdown {
    Settings,
}

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
                this.titlebar_dropdown = Some(TitlebarDropdown::Settings);
                cx.notify();
            }))
            .when_some(self.titlebar_dropdown, |this, dropdown| {
                let dropdown = match dropdown {
                    TitlebarDropdown::Settings => div()
                        .id("settings-dropdown")
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
                        .child("Theme")
                        .child("Check for updates")
                        .child("Help")
                    ,
                };

                this.child(
                    deferred(
                        anchored()
                            .anchor(Anchor::RightCenter)
                            .snap_to_window_with_margin(px(8.))
                            .child(dropdown
                                .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                                    this.titlebar_dropdown = None;
                                    cx.notify();
                                }))
                            )
                    )
                )
            })
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
                this.context_menu_pos = Some(event.position);
                cx.notify();
            }))
            .child(div()
                .flex()
                .flex_row()
                .child(settings_dropdown)
                .child(history_nav)
                .child(theme_switcher)
            )
            .child(window_controls)
            .children(self.context_menu_pos.map(|pos| {
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
                    .children((0..5).map(|num| div()
                        .id(format!("titlebar-cx-{}", num))
                        .px_2()
                        .py_1()
                        .text_sm()
                        .rounded_xs()
                        .hover(|this| this.bg(titlebar_hover))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            println!("Clicked menu item {}", num);
                            this.context_menu_pos = None;
                            cx.notify(); // not sure why this isnt necessary
                        }))
                        .child(format!("Titlebar {}", num))
                    ));
                deferred({
                    let window_size = window.viewport_size();
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .w(window_size.width)
                        .h(window_size.height)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.context_menu_pos = None;
                            cx.notify();
                        }))
                        .child(
                            anchored().position(pos).child(cx_menu)
                        )
                    })
                .with_priority(1)
            }))
    }
}