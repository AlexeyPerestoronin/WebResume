use yew::html::Html;

use crate::html_elements::{Div, GridCell, HtmlElement};

pub struct GridLayout(Div);

impl std::ops::Deref for GridLayout {
    type Target = Div;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GridLayout {
    pub fn new() -> Self {
        GridLayout(Div::new().set_style(Some("w-layout-layout wf-layout-layout".to_string())))
    }

    pub fn add_cell(mut self, cell: GridCell) -> Self
    {
        self.0 = self.0.add_component(cell);
        self
    }
}

impl HtmlElement for GridLayout {
    fn get_html(&self) -> Html {
        self.0.get_html()
    }
}
