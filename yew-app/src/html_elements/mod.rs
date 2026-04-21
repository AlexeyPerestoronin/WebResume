pub use yew::prelude::*;

pub trait HtmlElement{
    fn get_html(&self) -> Html;
}

pub mod div;
pub use div::*;

pub mod h1;
pub use h1::*;

pub mod input;
pub use input::*;

pub mod button;
pub use button::*;