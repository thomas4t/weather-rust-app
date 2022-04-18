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
            border-radius: 5px;
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
            float: right;
            padding: 10px;
            }
              
        "#)}>
            <h1>{"Weather app"}</h1>
            <nav>
                <ul>
                    <li><Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ "Home" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::About} classes="app-link">{ "About" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::PageNotFound} classes="app-link">{ "Not found page" }</Link<AppRoute>></li>
                </ul>
            </nav>
      </header>

    }
}

#[styled_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class={css!(r#"
            color: white;
            height: 50px;
            font-size: 20px;
            background-color: rgb(88, 164, 255);
            border-radius: 5px;
            border: none;
        "#)}>
        { "Footer" }
        </footer>
    }
}
