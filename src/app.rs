use std::{cell::RefCell, rc::Rc};

use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext, WindowContext};


pub struct App{
    count: Rc<RefCell<i32>>,
}

impl App {
    pub fn new(cx: &mut WindowContext) -> gpui::View<Self> {
        cx.new_view(|_| App { count: Rc::new(RefCell::new(0)) })
    }
}

impl Render for App {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let count = self.count.clone();
        div().children(vec![
            div()
                .text_color(rgb(0xFFFFFF))
                .child(self.count.borrow().to_string()),
            div()
                .text_color(rgb(0xFFFFFF))
                .child("+")
                .on_mouse_down(gpui::MouseButton::Left, move |_, cx| {
                    *count.borrow_mut() += 1;
                    println!("{}", count.borrow());
                    cx.refresh();
                }),
        ])
    }
}
