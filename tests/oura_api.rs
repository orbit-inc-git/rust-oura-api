use mockito::{Matcher, Server};

use oura_api;

fn get_date_query() -> oura_api::DateQuery<'static> {
    oura_api::DateQuery::builder()
        .start_date("2022-12-01")
        .end_date("2023-08-20")
        .build()
}

fn get_empty_date_query() -> oura_api::DateQuery<'static> {
    oura_api::DateQuery::builder().build()
}

#[test]
fn test_daily_spo2() {
    let mut server = Server::new();
    let base_url = server.url();
    let client = oura_api::OuraClient::build_with_base_url("token", &base_url);

    let mock = server
        .mock("GET", "/daily_spo2")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_date".into(), "2022-12-01".into()),
            Matcher::UrlEncoded("end_date".into(), "2023-08-20".into())
          ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data": [{"id": "123", "day": "2023-08-28", "spo2_percentage": {"average": 98.52}}]}"#)
        .create();

    let result = client.list_daily_spo2(get_date_query());

    mock.assert();

    println!("{:?}", result);
    assert_eq!(result.is_ok(), true);
}
