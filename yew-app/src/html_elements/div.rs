use std::{cell::RefCell, rc::Rc};

use stylist::StyleSource;
use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    style_opt: Option<StyleSource>,
    components: Vec<Rc<RefCell<dyn HtmlElement>>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            style_opt: None,
            components: Vec::new(),
        }
    }

    pub fn add_styles(mut self, style: StyleSource) -> Self {
        self.style_opt.replace(style);
        self
    }

    pub fn add_component(mut self, component: Rc<RefCell<dyn HtmlElement>>) -> Self {
        self.components.push(component);
        self
    }
}

impl HtmlElement for Div {
    fn get_html(&self) -> Html {
        match &self.style_opt {
            Some(stylesheet) => html! {
                <div class={stylesheet.clone()}>
                    { for self.components.iter().map(|c| c.borrow().get_html()) }
                </div>
            },
            None => html! {
                <div>
                    { for self.components.iter().map(|c| c.borrow().get_html()) }
                </div>
            },
        }
    }
}
