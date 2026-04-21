use std::{collections::LinkedList, rc::Rc};
use yew::prelude::*;

use crate::html_elements::HtmlElement;

// ListItem

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

// UnorderedList

pub struct UnorderedList {
    items: Rc<UseStateHandle<LinkedList<ListItem>>>,
}

impl UnorderedList {
    pub fn new(items: Rc<UseStateHandle<LinkedList<ListItem>>>) -> Self {
        UnorderedList {
            items: items,
        }
    }
}

impl HtmlElement for UnorderedList {
    fn get_html(&self) -> Html {
        html! {
            <ul>
                { for self.items.iter().map(|item| item.get_html()) }
            </ul>
        }
    }
}
