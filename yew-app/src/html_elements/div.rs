use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    components: Vec<Rc<RefCell<dyn HtmlElement>>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component(mut self, component: Rc<RefCell<dyn HtmlElement>>) -> Self {
        self.components.push(component);
        self
    }
}

impl HtmlElement for Div {
    fn get_html(&self) -> Html {
        html! {
            <div>
                { for self.components.iter().map(|c| c.borrow().get_html()) }
            </div>
        }
    }
}
