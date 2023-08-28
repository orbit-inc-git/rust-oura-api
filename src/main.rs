use oura_api::{DateQuery, OuraClient};

fn main() {
    let token = std::env::var("OURA_TOKEN").unwrap();
    let client = OuraClient::build(&token);
    let personal_info = client.get_personal_info().unwrap();
    println!("{:?}", personal_info);
    let daily_sleep = client
        .list_daily_sleep(DateQuery::builder().build())
        .unwrap();
    println!("{:?}", daily_sleep);
}
