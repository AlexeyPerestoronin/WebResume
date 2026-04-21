use std::rc::Rc;

use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Button {
    placeholder: String,
    on_click_event: Option<Callback<MouseEvent>>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            placeholder: String::new(),
            on_click_event: Option::None,
        }
    }

    pub fn set_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn set_on_click_event(mut self, on_click_event: Option<Callback<MouseEvent>>) -> Self {
        self.on_click_event = on_click_event;
        self
    }
}

impl HtmlElement for Button {
    fn get_html(&self) -> Html {
        html! { <button onclick={ self.on_click_event.clone() }> { self.placeholder.clone() } </button> }
    }
}
