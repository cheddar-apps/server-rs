#[allow(non_snake_case)]
pub mod I_LOVE_PROTOBUF;

pub use I_LOVE_PROTOBUF::proto;
fn main() {
    let mut it = proto::cheddar::api::HelloWorld {
        up_since: 0,
        healthy: true,
        ..Default::default()
    };
    println!("{:?}", it);

}
