fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 尝试编译 protobuf 文件
    let compile_result = compile_protos();
    
    // 如果编译失败
    if let Err(e) = compile_result {
        // 检查是否是 protoc 未找到的错误
        let error_msg = format!("{}", e);
        if error_msg.contains("Could not find `protoc`") || error_msg.contains("protoc") {
            // 检查 OUT_DIR 中是否已有生成的文件
            let out_dir = std::env::var("OUT_DIR")?;
            let generated_files = [
                "flare.common.v1.rs",
                "flare.signaling.v1.rs",
                "flare.push.v1.rs",
                "flare.storage.v1.rs",
                "flare.media.v1.rs",
                "flare.hooks.v1.rs",
                "flare.session.v1.rs",
                "flare.message.v1.rs",
                "flare.access_gateway.v1.rs",
            ];
            
            let all_files_exist = generated_files.iter().all(|file| {
                std::path::Path::new(&out_dir).join(file).exists()
            });
            
            if all_files_exist {
                // 文件已存在，使用预生成的文件
                println!("cargo:warning=protoc not found, using pre-generated protobuf files");
                println!("cargo:warning=If you modify proto files, install protoc: brew install protobuf");
                // 标记需要重新运行（如果 proto 文件改变）
                for proto_file in &[
                    "proto/common/errors.proto",
                    "proto/common/metadata.proto",
                    "proto/common/message.proto",
                    "proto/signaling.proto",
                    "proto/push.proto",
                    "proto/storage.proto",
                    "proto/media.proto",
                    "proto/hooks.proto",
                    "proto/session.proto",
                    "proto/message.proto",
                    "proto/access_gateway.proto",
                ] {
                    println!("cargo:rerun-if-changed={}", proto_file);
                }
                return Ok(());
            } else {
                // 文件不存在，返回错误
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to compile protobuf files: {}. Please install protoc: brew install protobuf", e)
                )));
            }
        } else {
            // 其他错误，直接返回
            return Err(e);
        }
    }
    
    Ok(())
}

fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let is_wasm = target_arch == "wasm32";

    let mut cfg = tonic_prost_build::configure();
    if is_wasm {
        cfg = cfg.build_server(false).build_client(false);
    } else {
        cfg = cfg.build_server(true).build_client(true);
    }

    cfg.compile_protos(
        &[
            "proto/common/errors.proto",
            "proto/common/metadata.proto",
            "proto/common/message.proto",
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
