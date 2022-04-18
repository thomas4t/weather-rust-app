use crate::error::Error;
use stylist::yew::styled_component;
use yew::{html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct BoundaryProps {
    pub is_loading: bool,
    pub error: Option<Error>,
    pub children: Children,
}

// <div class={css!(r#"
//     height: 100%;
//     display: flex;
//     flex-direction: column;
//     .content {
//         flex: 1;
//     }
// "#)}>
//     <Header />
//     <div class="content">{ for props.children.iter() }</div>
//     <Footer />
// </div>

#[styled_component(LoadingBoundary)]
pub fn loading_boundary(props: &BoundaryProps) -> Html {
    // Loading
    if props.is_loading {
        return html! { "Loading, wait a sec..." };
    }

    // Loading error
    if let Some(error) = &props.error {
        return match error {
            Error::DeserializeError => html! { "DeserializeError" },
            Error::RequestError => html! { "RequestError" },
            _ => html! { "Error occured >{ " },
        };
    }

    // Content
    html! {
        <>{ for props.children.iter() }</>
    }
}
