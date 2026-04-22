use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    style: Option<String>,
    components: Vec<Box<dyn HtmlElement>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            style: Option::None,
            components: Vec::new(),
        }
    }

    pub fn set_style(mut self, style: Option<String>) -> Self {
        self.style = style;
        self
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
            <div class={self.style.clone()}>
                { for self.components.iter().map(|c| c.get_html()) }
            </div>
        }
    }
}
