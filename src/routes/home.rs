use yew::prelude::*;
use yew_hooks::use_counter;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };

    html! {
        <div class="app">
            <a
                class="app-logo"
                href="https://yew.rs"
                target="_blank"
                rel="noopener noreferrer"
            >
            </a>
            <p>
                { "Edit " } <code>{ "src/routes/home.rs" }</code> { " and save to reload." }
            </p>
            <a
                id="learn_yew"
                class="app-link"
                href="https://yew.rs"
                target="_blank"
                rel="noopener noreferrer"
            >
                { "Learn Yew" }
            </a>
            <p>
                <button onclick={ondecrease}>{ "Decrease" }</button>
                { *counter }
                <button onclick={onincrease}>{ "Increase" }</button>
            </p>
        </div>
    }
}
