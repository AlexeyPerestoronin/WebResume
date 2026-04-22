use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct H3 {
    text: String,
}

impl H3 {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl HtmlElement for H3 {
    fn get_html(&self) -> Html {
        html! { <h3>{ { self.text.clone() } }</h3> }
    }
}
