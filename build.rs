fn main() {
    let mut config = tonic_build::configure();
    #[cfg(feature = "server-api")]
    {
        config = config.build_server(true);
    }

    #[cfg(feature = "client-api")]
    {
        config = config.build_client(true);
    }
    config
        .compile_protos(
            &[
                "proto/backend_api.proto",
                "proto/webhook_event.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
