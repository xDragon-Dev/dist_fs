mod auth;

use auth::*;
use chrono::{Duration, prelude::*};
use uuid::Uuid;

fn main() {
    let _uuid = Uuid::new_v4();
    let _obtained_uuid = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
    let _exp_time = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("Date out of range")
        .timestamp();

    let jwt_payload = JWTClaims::default();

    let json_string = serde_json::to_string_pretty(&jwt_payload).unwrap();
    print!("{}", json_string);

    let jwt_payload_from_json = serde_json::from_str::<JWTClaims>(&json_string).unwrap();
    println!("{:#?}", jwt_payload_from_json);

    let generated_jwt = generate_jwt(jwt_payload_from_json).unwrap();
    println!("{}", generated_jwt);
}
