#![allow(unused_variables, dead_code, unused_imports)]
use std::ops::Deref;

use gpui::{*, prelude::*};
use gpui_component::*;

use crate::testing::test_1::{titlebar::Titlebar, sidebar::Sidebar, page::Page, context_menu::*};

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

pub struct MainWindow {
    titlebar: Entity<Titlebar>,
    sidebar: Entity<Sidebar>,
    page: Entity<Page>,
    context_menu: Option<Entity<ContextMenu>>,
    // dropdown_menu: Option<Entity<DropDownMenu>>,
}

impl EventEmitter<ContextMenuEvent> for MainWindow {}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let titlebar = cx.new(|cx| Titlebar::new(cx));
        let sidebar = cx.new(|cx| Sidebar::new(cx));
        let page = cx.new(|cx| Page::new(cx));

        Self::subscribe_to_cx_menu(&titlebar, cx).detach();
        Self::subscribe_to_cx_menu(&sidebar, cx).detach();
        Self::subscribe_to_cx_menu(&page, cx).detach();

        Self {
            titlebar,
            sidebar,
            page,
            context_menu: None,
            // dropdown_menu: None,
        }
    }

    fn subscribe_to_cx_menu<E>(entity: &Entity<E>, cx: &mut Context<Self>,) -> Subscription 
    where
        E: EventEmitter<ContextMenuEvent>
    {
        cx.subscribe(&entity, |this, _, event, cx| {
            match event {
                ContextMenuEvent::Open { position, build } => {
                    // maybe add some logic to merely move if calculating in the same area, will need new flag to indicate a conditional change
                    let cx_menu = cx.new(|cx| ContextMenu::new(*position, build.clone(), cx));
                    Self::subscribe_to_cx_menu(&cx_menu, cx).detach();
                    this.context_menu = Some(cx_menu);
                    cx.notify();
                },
                ContextMenuEvent::Close => {
                    this.context_menu = None;
                    cx.notify();
                }
            }
        })
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
            .when_some(self.context_menu.clone(), move |this, context_menu| {
                this.child(
                    deferred(anchored()
                        .anchor(Anchor::TopLeft)
                        .position(context_menu.read(cx).position)
                        .snap_to_window_with_margin(px(8.))
                        .child(context_menu)
                    )
                )
            })
        

    }
}