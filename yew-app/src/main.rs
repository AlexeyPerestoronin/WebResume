use yew::prelude::*;

mod html_elements;
mod web_pages;

fn main() {
    yew::Renderer::<web_pages::Welcome>::new().render();
}
