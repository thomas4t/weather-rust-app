use stylist::yew::styled_component;
use yew::prelude::*;

use yew::{html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub on_click: Callback<MouseEvent>,
    pub disabled: bool,
    pub children: Children,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.on_click.clone();
    html! {
        <button
            type="button"
            {onclick}
            disabled={props.disabled}
            class={css!(r#"
                box-sizing: border-box;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                min-width: 200px;
                padding: 0.2rem 0.5rem;
                margin-left: 0.3rem;
                border-style: solid;
                border-width: 1px;
                border-color: gray;
                border-radius: 10px;
                background-color: white;
                font-weight: bold;
                transition: 0.3s;
                &:hover:enabled {
                    background-color: gray;
                    color: white;
                    border-color: white;
                    cursor: pointer;
                }
            "#)} >
        { for props.children.iter() }
        </button>

    }
}
