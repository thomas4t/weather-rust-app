use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yew::{html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub placeholder: String,
    pub on_input: Callback<String>,
}

#[styled_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let cb = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            cb.emit(value)
        })
    };
    html! {
        <div>
            <input
                type="text"
                {oninput}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                class={css!(r#"
                    box-sizing: border-box;
                    border: solid;
                    color: white;
                    font-size: 1.2rem;
                    border-width: 1px;
                    border-color: gray;
                    padding: 0.5rem;
                    width: 100%;
                    background: gray;
                    border-radius: 10px;
                    &:hover {
                        background-color: gray;
                        color: white;
                        border-color: white;
                    }
                "#)} />
        </div>
    }
}
