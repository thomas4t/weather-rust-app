use crate::types::RootWeather;

use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WeatherCardProps {
    pub weather: RootWeather,
}

#[styled_component(WeatherCard)]
pub fn weather_card(props: &WeatherCardProps) -> Html {
    let weather = props.weather.clone();
    let city = weather.city;
    let list = weather.list;

    let list_start_idx = 0;
    let list_end_idx = list.len() - 1;

    let date_from = list[list_start_idx].dt_txt.format("%e/%m/%Y").to_string();
    let date_to = list[list_end_idx].dt_txt.format("%e/%m/%Y").to_string();

    html! {
        //Card
        <div>
            <h2>{format!("{} {}", city.name, city.country)}</h2>
            <h3>{format!("Lat/lng - {}/{}", city.coord.lat, city.coord.lon)}</h3>
            <span>{format!("Weather from {} to {}", date_from, date_to)}</span>
            <div class={css!(r#"
                display:flex;
                overflow-y:auto;
                margin-top: 5rem;
                border: 1px solid white;
                padding: 10px;
                border-radius: 10px;
            "#)}>
                {for list.iter().map(|item| {
                    let time = item.dt_txt.format("%H:%M").to_string();
                    let date = item.dt_txt.format("%e/%m").to_string();
                    let info = item.weather.clone();
                    html! {
                        <div class={css!(r#"
                            min-width: 250px;
                            min-height: 250px;
                            margin-right: 3rem;
                            margin-bottom: 3rem;
                            border: 1px solid white;
                            padding: 10px;
                            border-radius: 10px;
                        "#)}>
                            <div>
                                <span>{time}{" "}{date}</span>
                            </div>
                            {for info.iter().map(|item| {
                                html! { <div>{format!("{}, {}", item.main, item.description)}</div>}
                            })}
                            <div>{format!("Temp {} °C", item.main.temp)}</div>
                            <div>{format!("Temp feels like {} °C", item.main.feels_like)}</div>
                    </div>
                    }
                })}

            </div>
        </div>
    }
}
