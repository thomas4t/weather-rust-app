use std::ops::Deref;

use crate::{
    components::{
        button::Button, input::Input, loading_boundary::LoadingBoundary, weather_card::WeatherCard,
    },
    services::forecast::query_by_city,
};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_hooks::use_async;

#[styled_component(Forecast)]
pub fn forecast() -> Html {
    let query_handle = use_state(|| String::from(""));
    let query = query_handle.deref().clone();

    let query_fetch = {
        let query = query.clone();
        use_async(async move { query_by_city(query).await })
    };
    let on_submit = {
        let state = query_fetch.clone();
        Callback::from(move |_| state.run())
    };

    let handle_input = { Callback::from(move |val: String| query_handle.set(val)) };

    let loading = query_fetch.loading;
    let error = query_fetch.error.clone();
    let placeholder = String::from("Enter city ...");
    let can_submit = query.len() > 3;
    html! {
        <LoadingBoundary is_loading={loading} error={error}>
            // Input
            <div class={css!(r#"
                width: 100%;
                margin-top: 2vh;
                display:flex;
                justify-content: center;
            "#)}>
                <Input value={query.clone()} on_input={handle_input} placeholder={placeholder} />
                <Button on_click={on_submit} disabled={!can_submit}>{ "Load" }</Button>
            </div>
            // Loaded
            {
                if let Some(forecast) = &query_fetch.data {
                    html! { <WeatherCard weather={forecast.clone()}/> }
                } else {
                    html! {}
                }
            }
        </LoadingBoundary>
    }
}
