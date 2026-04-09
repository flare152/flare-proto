//! Flare IM **基础** protobuf（prost）：`common/*` 与辅助工具。
//!
//! gRPC 服务定义与 tonic 生成见 [`flare_grpc_proto`](https://docs.rs/flare-grpc-proto)（crate `flare-grpc-proto`）。

pub mod flare {
    pub mod common {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.common.v1.rs"));
        }
    }
}

pub mod common {
    pub use crate::flare::common::v1::*;
}

/// `Any` 等便捷构建（不再提供旧的 response envelope 模型）。
pub mod response;

/// 元数据/上下文类型的便捷构建（与 metadata.proto 对齐：Pagination、Filter、Actor、Device 等）
pub mod metadata_builder;

// MessageContent 扩展方法（统一的编码/解码接口）
pub mod message_content_ext;
pub use message_content_ext::{MessageContentExt, encode_message_content, decode_message_content};

pub use response::pack_any;

// Metadata 便捷构建（业务层使用 pagination()、filter_eq()、device_context() 等，与 metadata.proto 一致）
pub use metadata_builder::{
    actor_service, actor_system, actor_tenant_admin, actor_user, actor_with_attributes,
    actor_with_roles, audit_context, device_context, device_with_priority_critical,
    device_with_priority_high, device_with_priority_low, filter_contains, filter_eq, filter_ge,
    filter_gt, filter_in, filter_le, filter_lt, filter_ne, filter_not_in, pagination,
    pagination_first, pagination_with_more, sort_asc, sort_desc, time_range, timestamp_seconds,
};

// Re-export commonly used types（仅 common）
pub use common::{
    AuditContext, MediaAttachment, Pagination, PushTaskPayloadKind,
    Message, MessageContent, MessageType, MessageStatus, MessageSource,
    DeleteType, MarkType, ReactionAction,
    MessageTimeline, MessageReadRecord,
    TextContent, ImageContent, VideoContent, AudioContent, FileContent,
    LocationContent, CardContent, NotificationContent, CustomContent,
    ForwardContent, ForwardItem, ForwardMode, Mention, ImageInfo, VideoInfo, AudioInfo,
    OfflinePushInfo,
    MqEnvelope, MqPayloadKind,
    EventBusEnvelope,
    ConnectionQuality,
    SyncKind, Sync, SyncRes,
    SingleConversationSync, MultiConversationSync,
    ConversationsIncrementalSync, ConversationsAllSync, ConversationDetailSync,
    QueryEventsSync, GetSyncCursorSync, UpdateSyncCursorSync,
    SingleConversationSyncRes, MultiConversationSyncRes,
    ConversationsIncrementalSyncRes, ConversationsAllSyncRes, ConversationDetailSyncRes,
    QueryEventsSyncRes, GetSyncCursorSyncRes, UpdateSyncCursorSyncRes,
    SyncSliceItem, ConversationSyncSlice, ConversationPatchType, ConversationPatch,
    ConversationSyncAllOptions, MultiDeviceCursor,
    ConflictResolution, ConversationSummary as ConversationSummaryProto,
    DeviceState as ConversationDeviceState,
};
