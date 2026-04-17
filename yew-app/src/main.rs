use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component(App)]
fn app() -> Html {
    // Состояние для хранения текста в поле ввода
    let input_value = use_state(|| String::new());
    // Состояние для списка добавленных сообщений
    let messages = use_state(|| Vec::<String>::new());

    // Обработчик изменения текста в input
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    // Обработчик нажатия кнопки
    let on_click = {
        let input_value = input_value.clone();
        let messages = messages.clone();
        Callback::from(move |_| {
            if !input_value.is_empty() {
                let mut current_messages = (*messages).clone();
                current_messages.push((*input_value).clone());
                messages.set(current_messages);
                input_value.set(String::new()); // Очищаем поле
            }
        })
    };

    html! {
        <div style="padding: 20px; font-family: sans-serif;">
            <h1>{ "Yew Form" }</h1>
            
            <input 
                type="text" 
                value={(*input_value).clone()} 
                oninput={on_input} 
                placeholder="Введите текст..." 
            />
            <button onclick={on_click}>{ "Добавить" }</button>

            <hr />
            <h3>{ "Список записей:" }</h3>
            <ul>
                { for (*messages).iter().map(|msg| html! { <li>{ msg }</li> }) }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
