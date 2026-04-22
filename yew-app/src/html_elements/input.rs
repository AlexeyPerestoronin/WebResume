use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct TextInput {
    value: Option<String>,
    placeholder: Option<String>,
    input_event_handler: Option<Callback<InputEvent>>,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            value: Option::None,
            placeholder: Option::None,
            input_event_handler: Option::None,
        }
    }

    pub fn set_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    pub fn set_placeholder(mut self, placeholder: Option<String>) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn set_input_event_handler(mut self, input_event_handler: Option<Callback<InputEvent>>,
    ) -> Self {
        self.input_event_handler = input_event_handler;
        self
    }
}

impl HtmlElement for TextInput {
    fn get_html(&self) -> Html {
        html! {
            <input
                type="text"
                value={self.value.clone()}
                oninput={self.input_event_handler.clone()}
                placeholder={self.placeholder.clone()}
            />
        }
    }
}
