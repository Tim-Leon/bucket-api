fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "proto/backend_api.proto",
                "proto/webhook_event.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
