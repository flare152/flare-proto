fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/common/errors.proto",
                "proto/common/metadata.proto",
                "proto/common/message.proto",  // 新增：统一消息定义
                "proto/signaling.proto",
                "proto/push.proto",
                "proto/storage.proto",
                "proto/media.proto",
                "proto/hooks.proto",
                "proto/session.proto",
                "proto/message.proto",
                "proto/access_gateway.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
