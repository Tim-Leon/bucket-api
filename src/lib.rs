
pub mod backend_api {
    //include!(concat!(env!("OUT_DIR"), "/backend_api.rs"));
    tonic::include_proto!("backend_api");
}

pub mod webhook_event {
    tonic::include_proto!("webhook_event");
}
