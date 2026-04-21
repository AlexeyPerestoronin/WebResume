use yew::prelude::*;

use crate::html_elements::HtmlElement;

// H1

pub struct H1 {
    text: String,
}

impl H1 {
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

impl HtmlElement for H1 {
    fn get_html(&self) -> Html {
        html! { <h1>{ { self.text.clone() } }</h1> }
    }
}

// H2

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
