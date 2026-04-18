use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct H1 {
    text: String,
}

impl H1 {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn add_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl HtmlElement for H1 {
    fn get_html(&self) -> Html {
        html! { <h1>{ { self.text.clone() } }</h1> }
    }
}
