use super::request_get;
use crate::error::Error;
use crate::types::*;
use dotenv_codegen::dotenv;

/// Get weather forecast by city
pub async fn query_by_city(city: String) -> Result<RootWeather, Error> {
    const WEATHER_ENDPOINT: &str = dotenv!("WEATHER_ENDPOINT");
    const WEATHER_API_KEY: &str = dotenv!("WEATHER_API_KEY");
    let url = format!(
        "{}/data/2.5/forecast?q={}&lang=en&mode=json&units=metric&appid={}",
        WEATHER_ENDPOINT, city, WEATHER_API_KEY
    );
    request_get::<RootWeather>(url).await
}
