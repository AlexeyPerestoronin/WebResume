use std::{cell::RefCell, collections::LinkedList, rc::Rc};

use stylist::yew::styled_component; // Используем специальный макрос
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::html_elements::{HtmlElement, ListItem};

mod html_elements;

#[styled_component(App)] // Заменяем function_component на styled_component
pub fn app() -> Html {
    let input_value: UseStateHandle<String> = use_state(|| String::new());
    let messages = use_state(|| Vec::<String>::new());

    // Описываем стили прямо в переменной
    let stylesheet = stylist::css!(
        r#"
        .btn-submit {
            background-color: #007bff;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
            transition: background-color 0.1s;
        }

        .btn-submit:hover {
            background-color: #0056b3;
        }

        .btn-submit:active {
            background-color: #28a745 !important;
        }

        input {
            margin-right: 10px;
            padding: 8px;
            border: 1px solid #ccc;
            border-radius: 4px;
        }
        "#
    );

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let on_click = {
        let input_value = input_value.clone();
        let messages = messages.clone();
        Callback::from(move |s| {
            if !input_value.is_empty() {
                let mut current_messages = (*messages).clone();
                current_messages.push((*input_value).clone());
                messages.set(current_messages);
                input_value.set(String::new());
            }
        })
    };

    let c = RefCell::new("hello".to_owned());

    *c.borrow_mut() = "bonjour".to_owned();

    assert_eq!(&*c.borrow(), "bonjour");

    let _ = html! {
        // Оборачиваем всё в div с нашими стилями
        <div class={stylesheet.clone()}>
            <div style="padding: 20px; font-family: sans-serif;">
                <h1>{ "Yew Form with Inline Styles" }</h1>

                <input
                    type="text"
                    value={(*input_value).clone()}
                    oninput={on_input}
                    placeholder="Введите текст..."
                />
                <button class="btn-submit" onclick={on_click}>{ "Добавить" }</button>

                <hr />
                <h3>{ "Список записей:" }</h3>
                <ul>
                    { for (*messages).iter().map(|msg| html! { <li>{ msg }</li> }) }
                </ul>
            </div>
        </div>
    };

    // TODO: change use_state to RefCell
    let user_input_history = Rc::new(use_state(|| LinkedList::<ListItem>::new()));
    let user_input = Rc::new(use_state(|| String::new()));
    let user_click_event = Rc::new({
        let user_input = user_input.clone();
        let user_input_history = user_input_history.clone();
        Callback::from(move |_mouse_event| {
            let mut new_history : LinkedList::<ListItem> = user_input_history.iter().cloned().collect();
            let new_item = html_elements::ListItem::new((*user_input).to_string());
            new_history.push_back(new_item);
            user_input_history.set(new_history);
        })
    });

    html_elements::Div::new()
        .add_styles(stylesheet)
        .add_component(Rc::new(RefCell::new(
            html_elements::H1::new().set_text("Пример тестирования подхода!".to_string()),
        )))
        .add_component(Rc::new(RefCell::new(
            html_elements::Input::new(user_input.clone())
                .set_placeholder("Введите текст...".to_string()),
        )))
        .add_component(Rc::new(RefCell::new(
            html_elements::Button::new()
                .set_placeholder("Добавить".to_string())
                .set_on_click_event(user_click_event.clone()),
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
