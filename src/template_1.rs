#![allow(unused_variables, dead_code, unused_imports)]
use gpui::{*, prelude::*};
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

pub struct TitleBar {
    context_menu_pos: Option<Point<Pixels>>,
}

impl TitleBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            context_menu_pos: None,
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_hover = cx.theme().title_bar.darken(0.2);

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
            .on_click(|event, window, cx| {
                
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
            
        
        let window_controls = div()
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
            );
        
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
                    .p_1p5()
                    .gap_y_0p5()
                    .border_1()
                    .rounded_sm()
                    .bg(cx.theme().title_bar)
                    .border_color(cx.theme().title_bar_border)
                    .children((0..10).map(|num| div()
                        .id(format!("titlebar-cx-{}", num))
                        .px_2()
                        .py_1()
                        .text_sm()
                        .rounded_xs()
                        .hover(|this| this.bg(titlebar_hover))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            println!("Clicked menu item {}", num);
                            cx.notify(); // not sure why this isnt necessary
                            this.context_menu_pos = None;
                        }))
                        .child(format!("Example List Item {}", num))
                    ));
                
                deferred(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(rgb(0x220000))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.context_menu_pos = None;
                            cx.notify();
                        }))
                        .child(
                            anchored().position(pos).child(cx_menu)
                        )
                )
                .with_priority(1)
            }))
    }
}

pub struct SideBar {
    width: f32,
}

impl SideBar {
    pub fn new() -> Self {
        Self {
            width: 212.,
        }
    }
}

impl Render for SideBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(div()
                .id("sidebar")
                .w(px(self.width))
                .bg(cx.theme().sidebar)
            )
            .child(div()
                .id("sidebar-drag-handle")
                .w_0p5()
                .mx_1()
                .bg(cx.theme().sidebar_border)
                .cursor_col_resize()
                .on_drag_move(move |event: &DragMoveEvent<SideBar>, window, cx| {
                    
                })
            )
    }
}


pub struct MainWindow {
    titlebar: Entity<TitleBar>,
    sidebar: Entity<SideBar>,
}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            titlebar: cx.new(|cx| TitleBar::new(cx)),
            sidebar: cx.new(|cx| SideBar::new()),
        }
    }

    pub fn has_custom_titlebar() -> bool {
        true
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main-window")
            .bg(cx.theme().background)
            .size_full()
            .flex()
            .flex_col()
            .child(self.titlebar.clone())
            .child(div()
                .flex()
                .flex_row()
                .size_full()
                .child(self.sidebar.clone())
            )
    }
}

