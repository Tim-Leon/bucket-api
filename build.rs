use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
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
                "proto/account/v1/account_service.proto",
                "proto/storage/v1/storage_service.proto",
                "proto/common/v1/account.proto",
                "proto/common/v1/storage.proto",
                "proto/common/v1/types.proto",
                "proto/engine/v1/engine_service.proto",
                "proto/region/v1/region_service.proto",
                "proto/search/v1/search_service.proto",
                "proto/share/v1/share_service.proto",
                "proto/transcoding/v1/transcoding_service.proto",
                "proto/webhook/v1/webhook_service.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
