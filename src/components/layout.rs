use crate::routes::AppRoute;
use stylist::yew::styled_component;
use yew::{html, Children, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[styled_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class={css!(r#"
            height: 100%;
            display: flex;
            flex-direction: column;
            .content {
                flex: 1;
                max-width: 70vw;
                margin: auto;
            }
        "#)}>
            <Header />
            <div class="content">{ for props.children.iter() }</div>
            <Footer />
        </div>
    }
}

#[styled_component(Header)]
fn header() -> Html {
    html! {
        <header class={css!(r#"
            background-color: rgb(88, 164, 255);
            border: none;
            display: flex;
            flex-direction:column;
            justify-content: center;
            align-items: center;

            nav ul {
                padding: 0;
                margin: 0;
              }
              
            nav li {
            display: block;
            float: left;
            padding: 10px;
            }
              
        "#)}>
            <h1>{"Weather app"}</h1>
            <nav>
                <ul>
                    <li><Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Forecast}>{ "Forecast" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::About}>{ "About" }</Link<AppRoute>></li>
                </ul>
            </nav>
      </header>

    }
}

#[styled_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class={css!(r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            text-align: center;
            background-color: rgb(88, 164, 255);
            border: none;
            span {
                margin: 5px;
            }
        "#)}>
                <span>{"Built with Rust&Yew"}</span>
                <span>{"© 2022 Tomáš Trávníček - All Rights Reserved."}</span>
        </footer>
    }
}
