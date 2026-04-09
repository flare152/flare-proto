//! `flare-proto` 辅助：`Any` 打包工具。

use prost::Message as ProstMessage;
use prost_types::Any;

/// 将实现了 `prost::Message` 的类型打包为 `prost_types::Any`
#[inline]
pub fn pack_any<M>(msg: &M, type_url: impl Into<String>) -> Any
where
    M: ProstMessage,
{
    Any {
        type_url: type_url.into(),
        value: msg.encode_to_vec(),
    }
}

// `StatusOnlyResponse` / `RpcEnvelope` 相关扩展与宏已移除：
// 无数据返回建议改用 gRPC `google.protobuf.Empty`，错误用 `tonic::Status` 表达。
