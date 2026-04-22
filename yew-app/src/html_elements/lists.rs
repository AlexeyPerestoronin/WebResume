use std::collections::LinkedList;
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
    items: LinkedList<ListItem>,
}

impl UnorderedList {
    pub fn new() -> Self {
        UnorderedList {
            items: LinkedList::<ListItem>::new(),
        }
    }

    pub fn fill_items<I>(mut self, mut filler: Option<I>) -> Self
    where
        I: IntoIterator<Item = ListItem>,
    {
        if let Some(it) = filler.take() {
            self.items.extend(it);
        };
        self
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
