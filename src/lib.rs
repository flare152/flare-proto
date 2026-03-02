//! Flare IM gRPC Protocol Definitions
//!
//! This crate provides gRPC protocol definitions for Flare IM communication core layer.
//! It can be used by both client and server implementations.

pub mod flare {
    pub mod common {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.common.v1.rs"));
        }
    }

    pub mod signaling {
        pub mod online {
            include!(concat!(env!("OUT_DIR"), "/flare.signaling.online.rs"));
        }
        pub mod router {
            include!(concat!(env!("OUT_DIR"), "/flare.signaling.router.rs"));
        }
    }

    pub mod push {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.push.v1.rs"));
        }
    }

    pub mod storage {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.storage.v1.rs"));
        }
    }

    pub mod media {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.media.v1.rs"));
        }
    }

    pub mod hooks {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.hooks.v1.rs"));
        }
    }

    pub mod conversation {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.conversation.v1.rs"));
        }
    }

    pub mod message {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.message.v1.rs"));
        }
    }

    pub mod access_gateway {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.access_gateway.v1.rs"));
        }
    }
}

pub mod common {
    pub use crate::flare::common::v1::*;
}

// MessageContent 扩展方法（统一的编码/解码接口）
pub mod message_content_ext;
pub use message_content_ext::{MessageContentExt, encode_message_content, decode_message_content};

// Signaling 模块已拆分为 online 和 router 子模块
// 为了向后兼容，导出 online 模块的类型到 signaling 根目录
pub mod signaling {
    pub mod online {
        pub use crate::flare::signaling::online::*;
    }
    pub mod router {
        pub use crate::flare::signaling::router::*;
    }
    // 向后兼容：默认导出 online 模块的类型
    pub use crate::flare::signaling::online::*;
}

pub mod push {
    pub use crate::flare::push::v1::*;
}

pub mod storage {
    pub use crate::flare::storage::v1::*;
}

pub mod media {
    pub use crate::flare::media::v1::*;
}

pub mod hooks {
    pub use crate::flare::hooks::v1::*;
}

pub mod conversation {
    pub use crate::flare::conversation::v1::*;
}

pub mod message {
    pub use crate::flare::message::v1::*;
}

pub mod access_gateway {
    pub use crate::flare::access_gateway::v1::*;
}

// Re-export commonly used types
// Note: Using explicit re-exports to avoid ambiguous glob re-exports warnings
pub use common::{
    AuditContext, MediaAttachment, Pagination, RequestContext, RpcStatus, TenantContext,
    // 消息相关类型（从 common 模块导出，统一消息定义）
    Message, MessageContent, MessageType, MessageStatus, MessageSource, ContentType,
    DeleteType, MarkType, ReactionAction,
    MessageTimeline, MessageReadRecord,
    TextContent, ImageContent, VideoContent, AudioContent, FileContent,
    LocationContent, CardContent, NotificationContent, CustomContent,
    ForwardContent, Mention, ImageInfo, VideoInfo, AudioInfo,
    OfflinePushInfo, VisibilityStatus,
    // 连接质量相关
    ConnectionQuality,
};

pub use signaling::online::{
    GetOnlineStatusRequest as SignalingGetOnlineStatusRequest,
    GetOnlineStatusResponse as SignalingGetOnlineStatusResponse,
    HeartbeatRequest,
    HeartbeatResponse,
    LoginRequest as SignalingLoginRequest,
    LoginResponse as SignalingLoginResponse,
    LogoutRequest,
    LogoutResponse,
    DeviceInfo,
    UserPresence,
};

pub use signaling::router::{
    SelectPushTargetsRequest,
    SelectPushTargetsResponse,
    GetDeviceRouteRequest,
    GetDeviceRouteResponse,
    BatchGetDeviceRoutesRequest,
    BatchGetDeviceRoutesResponse,
    RouteTarget,
    PushStrategy,
};

pub use push::{
    CancelScheduledPushRequest, CancelScheduledPushResponse, CreateTemplateRequest,
    CreateTemplateResponse, ListTemplatesRequest, ListTemplatesResponse, PushFailure,
    PushMessageRequest as PushPushMessageRequest,
    PushMessageResponse as PushPushMessageResponse, PushNotificationRequest as PushPushNotificationRequest,
    PushNotificationResponse as PushPushNotificationResponse, PushOptions, PushSchedule,
    PushTemplate, QueryPushStatusRequest, QueryPushStatusResponse, SchedulePushRequest,
    SchedulePushResponse, UpdateTemplateRequest, UpdateTemplateResponse,
};

pub use storage::{
    ExportMessagesRequest, ExportMessagesResponse, FailedMessage,
    GetMessageRequest as StorageGetMessageRequest, GetMessageResponse as StorageGetMessageResponse,
    QueryMessagesRequest as StorageQueryMessagesRequest,
    QueryMessagesResponse as StorageQueryMessagesResponse, SearchMessagesRequest, SearchMessagesResponse,
};

pub use media::{
    AbortMultipartUploadRequest, AbortMultipartUploadResponse, AccessControlEntry,
    CleanupOrphanedAssetsRequest, CleanupOrphanedAssetsResponse, CompleteMultipartUploadRequest,
    CompressOperation, CompressVideoOperation, CreateReferenceRequest, CreateReferenceResponse,
    DeleteFileRequest as MediaDeleteFileRequest, DeleteFileResponse as MediaDeleteFileResponse,
    DeleteReferenceRequest, DeleteReferenceResponse, DescribeBucketRequest, DescribeBucketResponse,
    FileInfo, GenerateUploadUrlRequest, GenerateUploadUrlResponse, GetFileInfoRequest,
    GetFileInfoResponse, GetFileUrlRequest, GetFileUrlResponse, ImageOperation,
    InitiateMultipartUploadRequest, InitiateMultipartUploadResponse, ListObjectsRequest,
    ListObjectsResponse, ListReferencesRequest, ListReferencesResponse, MediaReferenceInfo,
    ProcessImageRequest, ProcessImageResponse, ProcessVideoRequest, ProcessVideoResponse,
    ResizeOperation, SetObjectAclRequest, SetObjectAclResponse, SubtitleBurnOperation,
    ThumbnailOperation, UploadFileMetadata, UploadFileRequest, UploadFileResponse,
    UploadMultipartChunkRequest, UploadMultipartChunkResponse, VideoOperation, WatermarkOperation,
};
pub use hooks::{
    CustomHookRequest, CustomHookResponse, DeliveryHookRequest as ProtoDeliveryHookRequest,
    DeliveryHookResponse as ProtoDeliveryHookResponse,
    HookDeliveryEvent as ProtoHookDeliveryEvent,
    HookInvocationContext as ProtoHookInvocationContext,
    HookMessageDraft as ProtoHookMessageDraft,
    HookMessageRecord as ProtoHookMessageRecord,
    PostSendHookRequest as ProtoPostSendHookRequest,
    PostSendHookResponse as ProtoPostSendHookResponse,
    PreSendHookRequest as ProtoPreSendHookRequest,
    PreSendHookResponse as ProtoPreSendHookResponse,
    RecallHookRequest as ProtoRecallHookRequest,
    RecallHookResponse as ProtoRecallHookResponse,
    HookRecallEvent as ProtoHookRecallEvent,
    PresenceHookRequest, PresenceHookResponse,     ConversationLifecycleHookRequest,
    ConversationLifecycleHookResponse,
};
#[cfg(not(target_arch = "wasm32"))]
pub use hooks::hook_extension_client::HookExtensionClient;

#[cfg(not(target_arch = "wasm32"))]
pub use conversation::{
    conversation_service_client::ConversationServiceClient,
    conversation_service_server::ConversationServiceServer,
    DevicePresence as ConversationDevicePresence,
    ListConversationsRequest as ConversationListConversationsRequest,
    ListConversationsResponse as ConversationListConversationsResponse,
    ConversationBootstrapRequest,
    ConversationBootstrapResponse,
    SortOrder as ConversationSortOrder,
    ForceConversationSyncRequest,
    ForceConversationSyncResponse,
    ConversationPolicy,
    SyncMessagesRequest as ConversationSyncMessagesRequest,
    SyncMessagesResponse as ConversationSyncMessagesResponse,
    UpdateCursorRequest,
    UpdateCursorResponse,
    UpdatePresenceRequest,
    UpdatePresenceResponse,
};

// 这些类型现在在 common 模块中定义，重新导出使用一致的别名
pub use common::{
    DeviceState as ConversationDeviceState,
    ConflictResolution as ConversationConflictResolution,
    ConversationSummary as ConversationSummaryProto,
};
