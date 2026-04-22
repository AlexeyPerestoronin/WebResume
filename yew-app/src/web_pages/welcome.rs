use std::{cell::RefCell, collections::LinkedList, rc::Rc};
use web_sys::{History, HtmlInputElement};
use yew::prelude::*;

use crate::html_elements::{Button, Div, H1, HtmlElement, ListItem, TextInput, UnorderedList};

pub struct Welcome {
    user_input: Option<String>,
    history: Option<LinkedList<String>>,
}

pub enum Msg {
    UpdateInput(String),
    AddToList,
}

impl Component for Welcome {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user_input: Option::None,
            history: Option::None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(new_user_input) => {
                if new_user_input.is_empty() == false {
                    self.user_input = Some(new_user_input);
                    return true;
                }
                false
            }
            Msg::AddToList => {
                if let Some(input) = self.user_input.take() {
                    if input.is_empty() == false {
                        let history = self.history.get_or_insert_with(|| LinkedList::new());
                        history.push_back(input);
                        return true;
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let input_event_handler = link.callback(|input: InputEvent| {
            let input: HtmlInputElement = input.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });

        // Создаем callback для кнопки
        let click_event_handler = link.callback(|_: MouseEvent| Msg::AddToList);

        let filler = match &self.history {
            Some(user_inputs) => Some(user_inputs.iter().map(|input| ListItem::new(input.clone()))),
            None => Option::None,
        };

        Div::new()
            .add_component(Rc::new(RefCell::new(
                H1::new().set_text("Стиль через структуру".into()),
            )))
            .add_component(Rc::new(RefCell::new(
                TextInput::new()
                    .set_value(self.user_input.clone())
                    .set_placeholder(
                        self.user_input
                            .clone()
                            .or(Some("введите текст...".to_string())),
                    )
                    .set_input_event_handler(Some(input_event_handler)),
            )))
            .add_component(Rc::new(RefCell::new(
                Button::new()
                    .set_placeholder("Добавить".into())
                    .set_on_click_event_handler(Some(click_event_handler)),
            )))
            .add_component(Rc::new(RefCell::new(
                UnorderedList::new().fill_items(filler),
            )))
            .get_html()
    }
}
