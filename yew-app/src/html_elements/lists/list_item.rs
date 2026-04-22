use std::collections::LinkedList;
use yew::prelude::*;

use crate::html_elements::HtmlElement;

#[derive(Clone)]
pub struct ListItem {
    text: String,
}

impl ListItem {
    pub fn new(text: String) -> Self {
        Self { text: text }
    }
}

impl HtmlElement for ListItem {
    fn get_html(&self) -> Html {
        html! { <li>{self.text.clone()}</li> }
    }
}