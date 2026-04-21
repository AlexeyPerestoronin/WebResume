use std::{cell::RefCell, rc::Rc};
use stylist::StyleSource;
use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    style: Option<StyleSource>,
    components: Vec<Rc<RefCell<dyn HtmlElement>>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            style: None,
            components: Vec::new(),
        }
    }

    pub fn set_css_stylesheet(mut self, style: Option<StyleSource>) -> Self {
        self.style = style;
        self
    }

    pub fn add_component(mut self, component: Rc<RefCell<dyn HtmlElement>>) -> Self {
        self.components.push(component);
        self
    }
}

impl HtmlElement for Div {
    fn get_html(&self) -> Html {
        html! {
            <div class={self.style.clone()}>
                { for self.components.iter().map(|c| c.borrow().get_html()) }
            </div>
        }
    }
}
