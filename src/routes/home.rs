use stylist::yew::styled_component;
use yew::prelude::*;

/// Home page
#[styled_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h2 class={css!(r#"
                margin: 3rem;
            "#)}>
            {"Homepage, welcome!"}
            </h2>
            <div class={css!(r#"
                display: flex;
                justify-content: center;
            "#)}>
                <img src="https://cataas.com/cat/says/Hello%20weather%20app!" alt="cat" />
            </div>
        </div>
    }
}
