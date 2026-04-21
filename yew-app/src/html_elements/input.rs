use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::html_elements::HtmlElement;


pub struct Input {
    placeholder: String,
    input_value: Rc<UseStateHandle<String>>,
}

impl Input {
    pub fn new(input_value: Rc<UseStateHandle<String>>) -> Self {
        Self {
            placeholder: String::new(),
            input_value: input_value,
        }
    }

    pub fn set_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }
}

impl HtmlElement for Input {
    fn get_html(&self) -> Html {
        let on_input = {
            let input_value = self.input_value.clone();
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                input_value.set(input.value());
            })
        };

        html! {
            <input
                type="text"
                value={String::from((*self.input_value).as_str())}
                oninput={on_input}
                placeholder={self.placeholder.clone()}
            />
        }
    }
}
