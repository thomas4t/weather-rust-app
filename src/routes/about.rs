use yew::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h2>{"About, contributions"}</h2>
            <p>{"This project has been scraped together as an example of Rust WASM Single Page Application for a bachellor's thesis @UHK_FIM."}</p>
            <p>{"The main purpose is to demonstrate current architectural trends and to compare this project with a similar app written in JavaScript"}</p>
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
