#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug)]
pub struct OWMMainItem {
    pub temp: f64
}
#[derive(Deserialize, Debug)]
pub struct OWMWeatherItem {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String
}
#[derive(Deserialize, Debug)]
pub struct OWMListItems {
    pub dt: i64,
    pub main: OWMMainItem,
    pub weather: Vec<OWMWeatherItem>,
    pub dt_txt: String,
}

#[derive(Deserialize, Debug)]
pub struct OWMCityCoord {
	lat: f64,
	lon: f64
}

#[derive(Deserialize, Debug)]
pub struct OWMCity {
	pub id: i64,
	pub name: String,
	pub coord: OWMCityCoord,
	pub country: String,
	pub population: i64
}

#[derive(Deserialize, Debug)]
pub struct OWMResponse {
    pub cod: String,
    pub message: f64,
    pub cnt: i64,
    pub list: Vec<OWMListItems>,
    pub city: OWMCity
}