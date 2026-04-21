use std::rc::Rc;

use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Button {
    placeholder: String,
    on_click_event: Option<Rc<Callback<MouseEvent>>>,
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

    pub fn set_on_click_event(mut self, on_click_event: Rc<Callback<MouseEvent>>) -> Self {
        self.on_click_event = Some(on_click_event);
        self
    }
}

impl HtmlElement for Button {
    fn get_html(&self) -> Html {
        match &self.on_click_event {
            Some(call_back) => {
                html! { <button onclick={ call_back.as_ref() }> { self.placeholder.clone() } </button> }
            }
            None => html! { <button> { self.placeholder.clone() } </button> },
        }
    }
}
