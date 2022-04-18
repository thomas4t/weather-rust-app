use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::layout::Layout,
    routes::{switch, AppRoute},
};

/// Root
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </Layout>
        </BrowserRouter>
    }
}
