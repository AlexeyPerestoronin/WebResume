pub use yew::prelude::*;

pub trait HtmlElement{
    fn get_html(&self) -> Html;
}

pub mod div;
pub use div::*;

pub mod headers;
pub use headers::*;

pub mod input;
pub use input::*;

pub mod button;
pub use button::*;

pub mod lists;
pub use lists::*;