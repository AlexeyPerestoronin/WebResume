use yew::prelude::*;

use crate::html_elements::HtmlElement;

pub struct Grid {
    text: String,
}

impl Grid {
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

impl HtmlElement for Grid {
    fn get_html(&self) -> Html {
        html! {
            <div class="w-layout-layout wf-layout-layout">
            </div>
        }
    }
}
