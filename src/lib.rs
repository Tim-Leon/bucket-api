
pub mod backend_api {
    //include!(concat!(env!("OUT_DIR"), "/backend_api.rs"));
    tonic::include_proto!("backend_api");
}

pub mod webhook_event {
    tonic::include_proto!("webhook_event");
}


// Error model that is used is: https://cloud.google.com/apis/design/errors
#[path = "proto/tonic"]
pub mod tonic {
    #[cfg(feature = "metrics")]
    pub use opentelemetry_proto::tonic;
    #[cfg(feature = "bucket")]
    #[path = "storage/v1"]
    pub mod storage {

    }
    #[cfg(feature = "account")]
    #[path = "account/v1"]
    pub mod account {

    }
    #[cfg(feature = "region")]
    #[path = "region/v1"]
    pub mod region {

    }
    #[cfg(feature = "webhook")]
    #[path = "webhook/v1"]
    pub mod webhook {

    }
    #[cfg(feature = "share")]
    #[path = "share/v1"]
    pub mod share {

    }
    #[cfg(feature = "engine")]
    #[path = "engine/v1"]
    pub mod engine  {

    }
}