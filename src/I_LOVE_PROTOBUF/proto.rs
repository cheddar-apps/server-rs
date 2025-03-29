mod private {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}


pub mod cheddar {
    pub mod api {
        pub use super::super::private::hello_world::HelloWorld;
    }
}
