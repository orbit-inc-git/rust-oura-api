use mockito::{Matcher, Server};
use serde_json;

use oura_api::{self, DateQuery, ListResponse, OuraClient};

fn get_empty_date_query() -> DateQuery<'static> {
    DateQuery::builder().build()
}

fn get_empty_datetime_query() -> oura_api::DatetimeQuery<'static> {
    oura_api::DatetimeQuery::builder().build()
}

fn get_id() -> &'static str {
    "123"
}

#[test]
fn it_applies_query_to_list_requests() {
    let mut server = Server::new();
    let base_url = server.url();
    let client = OuraClient::build_with_base_url("token", &base_url);

    let fixture = std::fs::read_to_string("tests/fixtures/list_daily_activity.json").unwrap();

    let mock = server
        .mock("GET", "/daily_activity")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_date".into(), "2022-12-01".into()),
            Matcher::UrlEncoded("end_date".into(), "2023-08-20".into()),
            Matcher::UrlEncoded("next_token".into(), "next_token".into()),
        ]))
        .match_header("Authorization", "Bearer token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(fixture)
        .create();

    let query = oura_api::DateQuery::builder()
        .start_date("2022-12-01")
        .end_date("2023-08-20")
        .next_token("next_token")
        .build();

    let result = client.list_daily_activity(query);

    mock.assert();
    assert!(result.is_ok());
}

macro_rules! test_endpoint {
    ($test_name: ident, $client_function: ident, $fixture_path: literal, $url_path: literal, $type: ty, $($get_arguments: ident)?) => {
        #[test]
        fn $test_name() {
            let mut server = Server::new();
            let base_url = server.url();
            let client = OuraClient::build_with_base_url("token", &base_url);

            let fixture = std::fs::read_to_string($fixture_path).unwrap();

            let mock = server
                .mock("GET", $url_path)
                .match_header("Authorization", "Bearer token")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&fixture)
                .create();

            let result = client.$client_function($($get_arguments())?);

            mock.assert();
            assert!(result.is_ok());

            let expected_response: $type = serde_json::from_str(&fixture).unwrap();

            assert_eq!(result.unwrap(), expected_response);
        }
    };
}

test_endpoint! {
    it_gets_daily_activity,
    get_daily_activity,
    "tests/fixtures/get_daily_activity.json",
    "/daily_activity/123",
    oura_api::DailyActivity,
    get_id
}

test_endpoint! {
    it_lists_daily_activity,
    list_daily_activity,
    "tests/fixtures/list_daily_activity.json",
    "/daily_activity",
    ListResponse<oura_api::DailyActivity>,
    get_empty_date_query
}

test_endpoint! {
    it_gets_daily_readiness,
    get_daily_readiness,
    "tests/fixtures/get_daily_readiness.json",
    "/daily_readiness/123",
    oura_api::DailyReadiness,
    get_id
}

test_endpoint! {
    it_lists_daily_readiness,
    list_daily_readiness,
    "tests/fixtures/list_daily_readiness.json",
    "/daily_readiness",
    ListResponse<oura_api::DailyReadiness>,
    get_empty_date_query
}

test_endpoint! {
    it_gets_daily_sleep,
    get_daily_sleep,
    "tests/fixtures/get_daily_sleep.json",
    "/daily_sleep/123",
    oura_api::DailySleep,
    get_id
}

test_endpoint! {
    it_lists_daily_sleep,
    list_daily_sleep,
    "tests/fixtures/list_daily_sleep.json",
    "/daily_sleep",
    ListResponse<oura_api::DailySleep>,
    get_empty_date_query
}

test_endpoint! {
    it_gets_daily_spo2,
    get_daily_spo2,
    "tests/fixtures/get_daily_spo2.json",
    "/daily_spo2/123",
    oura_api::DailySpO2,
    get_id
}

test_endpoint! {
    it_lists_daily_spo2,
    list_daily_spo2,
    "tests/fixtures/list_daily_spo2.json",
    "/daily_spo2",
    ListResponse<oura_api::DailySpO2>,
    get_empty_date_query
}

test_endpoint! {
    it_lists_heart_rate,
    list_heart_rate,
    "tests/fixtures/list_heart_rate.json",
    "/heartrate",
    ListResponse<oura_api::HeartRate>,
    get_empty_datetime_query
}

test_endpoint! {
    it_gets_personal_info,
    get_personal_info,
    "tests/fixtures/get_personal_info.json",
    "/personal_info",
    oura_api::PersonalInfo,
}

test_endpoint! {
    it_gets_rest_mode_period,
    get_rest_mode_period,
    "tests/fixtures/get_read_mode_period.json",
    "/rest_mode_period/123",
    oura_api::RestModePeriod,
    get_id
}

test_endpoint! {
    it_lists_read_mode_period,
    list_rest_mode_period,
    "tests/fixtures/list_rest_mode_period.json",
    "/rest_mode_period",
    ListResponse<oura_api::RestModePeriod>,
    get_empty_date_query
}

test_endpoint! {
    it_gets_ring_configuration,
    get_ring_configuration,
    "tests/fixtures/get_ring_configuration.json",
    "/ring_configuration/123",
    oura_api::RingConfiguration,
    get_id
}

test_endpoint! {
    it_lists_ring_configuration,
    list_ring_configuration,
    "tests/fixtures/list_ring_configuration.json",
    "/ring_configuration",
    ListResponse<oura_api::RingConfiguration>,
    get_empty_date_query
}
