use yew::prelude::*;
use yew_hooks::use_async;

use crate::services::forecast::query_by_city;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let state = use_async(async move { query_by_city(("Praha".to_string()).clone()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    log::info!("Data: {:?}", state.data);

    html! {
        <div class="app">
                <p>
                    <a
                        class="app-link"
                        href="https://github.com/jetli/create-yew-app"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Create Yew App" }
                    </a>
                    { ", Set up a modern yewww web app by running one command." }
                </p>
                <p>
                    <button {onclick}>{ "Load info of this repo" }</button>
                </p>
                <p>
                    {
                        if state.loading {
                            html! { "Loading, wait a sec..." }
                        } else {
                            html! {}
                        }
                    }
                </p>
                {
                    if let Some(forecast) = &state.data {
                        html! {
                            <>
                                {"loaded"}
                                <p>{&forecast.city.name}</p>
                                // <p>{ "Repo name x: " }<b>{ &repo.city }</b></p>
                                // <p>{ "Repo full name: " }<b>{ &repo.full_name }</b></p>
                                // <p>{ "Repo description: " }<b>{ &repo.description }</b></p>
                            </>
                            }
                    } else {
                        html! {}
                    }
                }
                // <p>
                //     {
                //         if let Some(error) = &state.error {
                //             match error {
                //                 Error::DeserializeError => html! { "DeserializeError" },
                //                 Error::RequestError => html! { "RequestError" },
                //             }
                //         } else {
                //             html! {}
                //         }
                //     }
                // </p>
                <p>
                    { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                </p>
        </div>
    }
}

// #[cfg(test)]
// mod tests {
//     use wasm_bindgen_test::*;

//     wasm_bindgen_test_configure!(run_in_browser);

//     use super::About;
//     use yew::start_app;

//     #[wasm_bindgen_test]
//     fn about_page_has_an_app_link() {
//         start_app::<About>();

//         let app_links = gloo_utils::document().get_elements_by_class_name("app-link");

//         assert_eq!(app_links.length(), 1);

//         let link = app_links.item(0).expect("No app-link").inner_html();
//         assert_eq!(link, "Create Yew App");
//     }
// }
