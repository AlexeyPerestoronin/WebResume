# Task: Implement view method for Div

## Steps:
- [x] Create TODO.md with steps
- [ ] Edit yew-app/src/html_elements/div.rs to implement view method (style as class, render components)
- [ ] Verify compilation with cargo check
- [ ] Attempt completion

        <div class={stylesheet}>
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

html_component::Div::new()
    .add_style(stylesheet)
    .add_component(
        html_component::Div::new()
            .add_component(html_component::H1::new().add_text("Yew Form with Inline Styles"))
            .add_component(html_component::Input::new().add_placeholder("Input new text..."))
            .add_component(html_component::Button::new().add_text("Добавить").add_onclick(on_click))
            .add_component(html_component::Hr::new())
            .add_component(html_component::Ul::new())
    )
    .get_html()