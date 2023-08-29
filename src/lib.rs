use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use typed_builder::TypedBuilder;

const API_BASE_URL: &str = "https://api.ouraring.com/v2/usercollection";

#[derive(Deserialize, Debug)]
pub struct Sample {
    pub interval: f32,
    pub items: Vec<Option<f32>>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct ActivityContributors {
    pub meet_daily_targets: Option<u8>,
    pub move_every_hour: Option<u8>,
    pub recovery_time: Option<u8>,
    pub stay_active: Option<u8>,
    pub training_frequency: Option<u8>,
    pub training_volume: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailyActivity {
    pub id: String,
    pub class_5_min: Option<String>,
    pub score: Option<u8>,
    pub active_calories: u32,
    pub average_met_minutes: f32,
    pub contributors: ActivityContributors,
    pub equivalent_walking_distance: u32,
    pub high_activity_met_minutes: u32,
    pub high_activity_time: u32,
    pub inactivity_alerts: u32,
    pub low_activity_met_minutes: u32,
    pub low_activity_time: u32,
    pub medium_activity_met_minutes: u32,
    pub medium_activity_time: u32,
    pub met: Sample,
    pub meters_to_target: u32,
    pub non_wear_time: u32,
    pub resting_time: u32,
    pub sedentary_met_minutes: u32,
    pub sedentary_time: u32,
    pub steps: u32,
    pub target_calories: u32,
    pub target_meters: u32,
    pub total_calories: u32,
    pub day: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct ReadinessContributors {
    pub activity_balance: Option<u8>,
    pub body_temperature: Option<u8>,
    pub hrv_balance: Option<u8>,
    pub previous_day_activity: Option<u8>,
    pub previous_night: Option<u8>,
    pub recovery_index: Option<u8>,
    pub resting_heart_rate: Option<u8>,
    pub sleep_balance: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailyReadiness {
    pub id: String,
    pub contributors: ReadinessContributors,
    pub day: String,
    pub score: Option<u8>,
    pub temperature_deviation: Option<f32>,
    pub temperature_trend_deviation: Option<f32>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct SleepContributors {
    pub deep_sleep: Option<u8>,
    pub efficiency: Option<u8>,
    pub latency: Option<u8>,
    pub rem_sleep: Option<u8>,
    pub restfulness: Option<u8>,
    pub timing: Option<u8>,
    pub total_sleep: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailySleep {
    pub id: String,
    pub contributors: SleepContributors,
    pub day: String,
    pub timestamp: String,
    pub score: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailySpO2AggregatedValues {
    pub average: f32,
}

#[derive(Deserialize, Debug)]
pub struct DailySpO2 {
    pub id: String,
    pub day: String,
    pub spo2_percentage: Option<DailySpO2AggregatedValues>,
}

#[derive(Deserialize, Debug)]
pub struct HeartRate {
    pub bpm: u8,
    pub source: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct PersonalInfo {
    pub id: String,
    pub age: Option<u32>,
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub biological_sex: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RestModeEpisode {
    pub tags: Vec<String>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct RestModePeriod {
    pub id: String,
    pub end_day: Option<String>,
    pub end_time: Option<String>,
    pub episodes: Vec<RestModeEpisode>,
    pub start_day: String,
    pub start_time: String,
}

#[derive(Serialize, TypedBuilder)]
pub struct DateQuery {
    #[builder(default = None, setter(strip_option))]
    start_date: Option<String>,
    #[builder(default = None, setter(strip_option))]
    end_date: Option<String>,
    #[builder(default = None, setter(strip_option))]
    next_token: Option<String>,
}

#[derive(Serialize, TypedBuilder)]
pub struct DatetimeQuery {
    #[builder(default = None, setter(strip_option))]
    start_datetime: Option<String>,
    #[builder(default = None, setter(strip_option))]
    end_datetime: Option<String>,
    #[builder(default = None, setter(strip_option))]
    next_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub next_token: Option<String>,
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
    ($name: ident, $type: ty, $path: literal, $params: ty) => {
        pub fn $name(&self, params: $params) -> Result<ListResponse<$type>, Box<dyn Error>> {
            let url = format!("{}/{}", API_BASE_URL, $path);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .query(&params)
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

    list_endpoint!(
        list_daily_activity,
        DailyActivity,
        "daily_activity",
        DateQuery
    );
    get_endpoint!(get_daily_activity, DailyActivity, "daily_activity");

    list_endpoint!(
        list_daily_readiness,
        DailyReadiness,
        "daily_readiness",
        DateQuery
    );
    get_endpoint!(get_daily_readiness, DailyReadiness, "daily_readiness");

    list_endpoint!(list_daily_sleep, DailySleep, "daily_sleep", DateQuery);
    get_endpoint!(get_daily_sleep, DailySleep, "daily_sleep");

    list_endpoint!(list_daily_spo2, DailySpO2, "daily_spo2", DateQuery);
    get_endpoint!(get_daily_spo2, DailySpO2, "daily_spo2");

    list_endpoint!(list_heart_rate, HeartRate, "heartrate", DatetimeQuery);

    generic_endpoint!(get_personal_info, PersonalInfo, "personal_info");

    list_endpoint!(
        list_rest_mode_period,
        RestModePeriod,
        "rest_mode_period",
        DateQuery
    );
    get_endpoint!(get_rest_mode_period, RestModePeriod, "rest_mode_period");
}
