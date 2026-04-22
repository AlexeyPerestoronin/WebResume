use std::{cell::RefCell, collections::LinkedList, rc::Rc};
use stylist::yew::styled_component;
use yew::prelude::*;

mod html_elements;
use crate::html_elements::{HtmlElement, ListItem};

#[styled_component(App)] // Заменяем function_component на styled_component
pub fn app() -> Html {
    let user_input_history = Rc::new(use_state(|| LinkedList::<ListItem>::new()));
    let user_input = Rc::new(use_state(|| String::new()));
    let user_click_event = {
        let user_input = user_input.clone();
        let user_input_history = user_input_history.clone();
        Callback::from(move |_mouse_event: MouseEvent| {
            let mut new_history: LinkedList<ListItem> =
                user_input_history.iter().cloned().collect();
            let new_item = html_elements::ListItem::new((*user_input).to_string());
            new_history.push_back(new_item);
            user_input_history.set(new_history);
        })
    };

    html_elements::Div::new()
        .add_component(Rc::new(RefCell::new(
            html_elements::H1::new().set_text("Пример тестирования подхода!".to_string()),
        )))
        .add_component(Rc::new(RefCell::new(
            html_elements::Input::new(user_input.clone())
                .set_placeholder("Введите текст...".to_string()),
        )))
        .add_component(Rc::new(RefCell::new(
            html_elements::Button::new()
                .set_style(Some("test-button".to_string()))
                .set_placeholder("Добавить".to_string())
                .set_on_click_event(Some(user_click_event)),
        )))
        .add_component(Rc::new(RefCell::new(
            html_elements::H3::new().set_text("Список записей:".to_string()),
        )))
        .add_component(Rc::new(RefCell::new(html_elements::UnorderedList::new(
            user_input_history.clone(),
        ))))
        .get_html()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
