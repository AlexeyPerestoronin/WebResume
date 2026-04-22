use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    components: Vec<Box<dyn HtmlElement>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component<T>(mut self, component: T) -> Self
    where
        T: HtmlElement + 'static,
    {
        self.components.push(Box::new(component));
        self
    }
}

impl HtmlElement for Div {
    fn get_html(&self) -> Html {
        html! {
            <div>
                { for self.components.iter().map(|c| c.get_html()) }
            </div>
        }
    }
}
