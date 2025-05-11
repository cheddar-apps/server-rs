pub mod protobuf;

pub mod db;

use dotenvy::dotenv;
pub use protobuf::proto;
fn main() {
    dotenv().ok();

    let hello_world = proto::cheddar::api::HelloWorld {
        up_since: 0,
        healthy: true,
    };
    println!("hello_world = {:?}", hello_world);
}
