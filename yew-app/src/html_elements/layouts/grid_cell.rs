use yew::html::Html;

use crate::html_elements::{Div, HtmlElement};

pub struct GridCell(Div);

impl std::ops::Deref for GridCell {
    type Target = Div;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GridCell {
    pub fn new() -> Self {
        GridCell(Div::new().set_style(Some("w-layout-cell".to_string())))
    }

    pub fn add_component<T>(mut self, component: T) -> Self
    where
        T: HtmlElement + 'static,
    {
        self.0 = self.0.add_component(component);
        self
    }
}

impl HtmlElement for GridCell {
    fn get_html(&self) -> Html {
        self.0.get_html()
    }
}
