pub use yew::prelude::*;

pub trait HtmlElement{
    fn get_html(&self) -> Html;
}

// enum HtmlComponent2<'a> {
//     Div(Div),
//     H1(H1),
//     Input(Input<'a>),
// }

pub mod div;
pub use div::*;

pub mod h1;
pub use h1::*;

pub mod input;
pub use input::*;