pub mod cheddar {
    pub mod api {
        include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.rs"));
        pub mod user {
            include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.user.rs"));
        }
        pub mod request {
            include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.request.rs"));
            pub mod auth {
                include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.request.auth.rs"));
            }
            pub mod user {
                include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.request.user.rs"));
            }
            pub mod ws {
                include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.request.ws.rs"));
            }
        }
        pub mod auth {
            include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.auth.rs"));
        }
        pub mod common {
            include!(concat!(env!("OUT_DIR"), "/app.cheddar.api.common.rs"));
        }
    }
}
