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
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.signaling.v1.rs"));
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

    pub mod session {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/flare.session.v1.rs"));
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

pub mod signaling {
    pub use crate::flare::signaling::v1::*;
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

pub mod session {
    pub use crate::flare::session::v1::*;
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
    MessageTimeline, MessageReadRecord, MessageOperation,
    TextContent, ImageContent, VideoContent, AudioContent, FileContent,
    LocationContent, CardContent, NotificationContent, CustomContent,
    ForwardContent, TypingContent, Mention, ImageInfo, VideoInfo, AudioInfo,
    OfflinePushInfo, VisibilityStatus,
};

pub use signaling::{
    GetOnlineStatusRequest as SignalingGetOnlineStatusRequest,
    GetOnlineStatusResponse as SignalingGetOnlineStatusResponse, HeartbeatRequest,
    HeartbeatResponse, LoginRequest as SignalingLoginRequest,
    LoginResponse as SignalingLoginResponse, LogoutRequest, LogoutResponse,
    RouteMessageRequest as SignalingRouteMessageRequest,
    RouteMessageResponse as SignalingRouteMessageResponse,
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
    BatchStoreMessageRequest, BatchStoreMessageResponse, ClearSessionRequest, ClearSessionResponse,
    DeleteMessageForUserRequest, DeleteMessageForUserResponse, DeleteMessageRequest,
    DeleteMessageResponse, ExportMessagesRequest, ExportMessagesResponse, FailedMessage,
    GetMessageRequest as StorageGetMessageRequest, GetMessageResponse as StorageGetMessageResponse,
    MarkMessageReadRequest, MarkMessageReadResponse,
    QueryMessagesRequest as StorageQueryMessagesRequest,
    QueryMessagesResponse as StorageQueryMessagesResponse, RecallMessageRequest,
    RecallMessageResponse, SearchMessagesRequest, SearchMessagesResponse,
    SetMessageAttributesRequest, SetMessageAttributesResponse,
    StoreMessageRequest as StorageStoreMessageRequest,
    StoreMessageResponse as StorageStoreMessageResponse,
    // 注意：Message、MessageOperation、MessageTimeline 已迁移到 common 模块
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
    PresenceHookRequest, PresenceHookResponse, SessionLifecycleHookRequest,
    SessionLifecycleHookResponse,
};
pub use hooks::hook_extension_client::HookExtensionClient;

pub use session::{
    session_service_client::SessionServiceClient,
    session_service_server::SessionServiceServer,
    DevicePresence as SessionDevicePresence,
    DeviceState as SessionDeviceState,
    ConflictResolution as SessionConflictResolution,
    ListSessionsRequest as SessionListSessionsRequest,
    ListSessionsResponse as SessionListSessionsResponse,
    SessionBootstrapRequest,
    SessionBootstrapResponse,
    SessionSummary as SessionSummaryProto,
    SortOrder as SessionSortOrder,
    ForceSessionSyncRequest,
    ForceSessionSyncResponse,
    SessionPolicy,
    SyncMessagesRequest as SessionSyncMessagesRequest,
    SyncMessagesResponse as SessionSyncMessagesResponse,
    UpdateCursorRequest,
    UpdateCursorResponse,
    UpdatePresenceRequest,
    UpdatePresenceResponse,
};
