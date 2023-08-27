use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

const API_BASE_URL: &str = "https://api.ouraring.com/v2/usercollection";

#[derive(Deserialize, Debug)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub next_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PersonalInfo {
    pub id: String,
    pub age: u8,
    pub weight: f64,
    pub height: f64,
    pub biological_sex: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct SleepContributors {
    pub deep_sleep: u8,
    pub efficiency: u8,
    pub latency: u8,
    pub rem_sleep: u8,
    pub restfulness: u8,
    pub timing: u8,
    pub total_sleep: u8,
}

#[derive(Deserialize, Debug)]
pub struct DailySleep {
    pub id: String,
    pub contributors: SleepContributors,
    pub day: String,
    pub timestamp: String,
    pub score: u8,
}

#[derive(Deserialize, Debug)]
pub struct ReadinessContributors {
    pub activity_balance: u8,
    pub body_temperature: u8,
    pub hrv_balance: u8,
    pub previous_day_activity: u8,
    pub previous_night: u8,
    pub recovery_index: u8,
    pub resting_heart_rate: u8,
    pub sleep_balance: u8,
}

#[derive(Deserialize, Debug)]
pub struct DailyReadiness {
    pub id: String,
    pub contributors: ReadinessContributors,
    pub day: String,
    pub score: u8,
    pub temperature_deviation: f64,
    pub temperature_trend_deviation: f64,
    pub timestamp: String,
}

macro_rules! generic_endpoint {
    ($name: ident, $type: ty, $path: literal) => {
        pub fn $name(&self) -> Result<$type, Box<dyn Error>> {
            let url = format!("{}/{}", API_BASE_URL, $path);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .send()?
                .json::<$type>()?;
            Ok(response)
        }
    };
}

macro_rules! list_endpoint {
    ($name: ident, $type: ty, $path: literal) => {
        pub fn $name(&self) -> Result<ListResponse<$type>, Box<dyn Error>> {
            let url = format!("{}/{}", API_BASE_URL, $path);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .send()?
                .json::<ListResponse<$type>>()?;
            Ok(response)
        }
    };
}

macro_rules! get_endpoint {
    ($name: ident, $type: ty, $path: literal) => {
        pub fn $name(&self, id: &str) -> Result<$type, Box<dyn Error>> {
            let url = format!("{}/{}/{}", API_BASE_URL, $path, id);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .send()?
                .json::<$type>()?;
            Ok(response)
        }
    };
}

pub struct OuraClient<'a> {
    token: &'a str,
    client: Client,
}

impl<'a> OuraClient<'a> {
    pub fn build(token: &'a str) -> Self {
        let client = Client::new();
        Self { token, client }
    }

    generic_endpoint!(get_personal_info, PersonalInfo, "personal_info");

    list_endpoint!(list_daily_sleep, DailySleep, "daily_sleep");
    get_endpoint!(get_daily_sleep, DailySleep, "daily_sleep");

    list_endpoint!(list_daily_readiness, DailyReadiness, "daily_readiness");
    get_endpoint!(get_daily_readiness, DailyReadiness, "daily_readiness");
}
