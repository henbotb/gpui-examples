#![allow(unused_variables, dead_code, unused_imports)]
use gpui::{*, prelude::*};
use gpui_component::*;

use crate::testing::test_1::{titlebar::Titlebar, sidebar::Sidebar, page::Page};

// first experiment, intended to learn how to make context menus, resizable windows, and dropdown menus

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

pub enum ContextMenuEmitter {}

impl EventEmitter<ContextMenuEmitter> for Titlebar {}
impl EventEmitter<ContextMenuEmitter> for Sidebar {}
impl EventEmitter<ContextMenuEmitter> for MainWindow {}




pub struct ContextMenu {
}

pub struct DropDownMenu {
}

pub struct MainWindow {
    titlebar: Entity<Titlebar>,
    sidebar: Entity<Sidebar>,
    page: Entity<Page>,
    context_menu: Option<Entity<ContextMenu>>,
    dropdown_menu: Option<Entity<DropDownMenu>>,
}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            titlebar: cx.new(|cx| Titlebar::new(cx)),
            sidebar: cx.new(|cx| Sidebar::new(cx)),
            page: cx.new(|cx| Page::new(cx)),
            context_menu: None,
            dropdown_menu: None,
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
                .child(self.page.clone())
            )
    }
}

