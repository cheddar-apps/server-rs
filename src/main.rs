#[allow(non_snake_case)]
pub mod I_LOVE_PROTOBUF;

pub mod db;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
pub use I_LOVE_PROTOBUF::proto;
fn main() {
    let it = proto::cheddar::api::HelloWorld {
        up_since: 0,
        healthy: true,
        ..Default::default()
    };
    println!("{:?}", it);

    // db connection
    dotenv().ok();
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));

    // TODO(i3vie): test creating or otherwise getting a user
}
