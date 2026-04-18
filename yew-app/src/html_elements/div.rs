use stylist::StyleSource;
use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Div {
    style_opt: Option<StyleSource>,
    components: Vec<Box<dyn HtmlElement>>,
}

impl Div {
    pub fn new() -> Self {
        Self {
            style_opt: None,
            components: Vec::new(),
        }
    }

    pub fn add_style(mut self, style: StyleSource) -> Self {
        self.style_opt.replace(style);
        self
    }

    pub fn add_component(mut self, component: Box<dyn HtmlElement>) -> Self {
        self.components.push(component);
        self
    }
}

impl HtmlElement for Div {
    fn get_html(&self) -> Html {
        match &self.style_opt {
            Some(stylesheet) => html! {
                <div class={stylesheet.clone()}>
                    { for self.components.iter().map(|c| c.get_html()) }
                </div>
            },
            None => html! {
                <div>
                    { for self.components.iter().map(|c| c.get_html()) }
                </div>
            },
        }
    }
}
