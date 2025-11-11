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

    pub mod communication_core {
        pub mod v1 {
            include!(concat!(
                env!("OUT_DIR"),
                "/flare.communication_core.v1.rs"
            ));
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
}

pub mod common {
    pub use crate::flare::common::v1::*;
}

pub mod communication_core {
    pub use crate::flare::communication_core::v1::*;
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

// Re-export commonly used types
// Note: Using explicit re-exports to avoid ambiguous glob re-exports warnings
pub use common::{
    AuditContext, MediaAttachment, Pagination, RequestContext, RpcStatus, TenantContext,
};

pub use communication_core::{
    GetOnlineStatusRequest, GetOnlineStatusResponse, LoginRequest, LoginResponse, Message,
    Notification, OnlineStatus, PushFailure, PushMessageRequest, PushMessageResponse,
    PushNotificationRequest, PushNotificationResponse, PushOptions, QueryMessagesRequest,
    QueryMessagesResponse, RouteMessageRequest, RouteMessageResponse, StoreMessageRequest,
    StoreMessageResponse,
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
    PushMessageRequest as PushPushMessageRequest, PushMessageResponse as PushPushMessageResponse,
    PushNotificationRequest as PushPushNotificationRequest,
    PushNotificationResponse as PushPushNotificationResponse,
};

pub use storage::{
    BatchStoreMessageRequest, BatchStoreMessageResponse, DeleteMessageRequest,
    DeleteMessageResponse, FailedMessage, QueryMessagesRequest as StorageQueryMessagesRequest,
    QueryMessagesResponse as StorageQueryMessagesResponse,
    StoreMessageRequest as StorageStoreMessageRequest,
    StoreMessageResponse as StorageStoreMessageResponse,
};

pub use media::{
    AbortMultipartUploadRequest, AbortMultipartUploadResponse, CleanupOrphanedAssetsRequest,
    CleanupOrphanedAssetsResponse, CompleteMultipartUploadRequest, CompressOperation,
    CreateReferenceRequest, CreateReferenceResponse, DeleteFileRequest as MediaDeleteFileRequest,
    DeleteFileResponse as MediaDeleteFileResponse, DeleteReferenceRequest, DeleteReferenceResponse,
    FileInfo, GetFileInfoRequest, GetFileInfoResponse, GetFileUrlRequest, GetFileUrlResponse,
    ImageOperation, InitiateMultipartUploadRequest, InitiateMultipartUploadResponse,
    ListReferencesRequest, ListReferencesResponse, MediaReferenceInfo, ProcessImageRequest,
    ProcessImageResponse, ProcessVideoRequest, ProcessVideoResponse, ResizeOperation,
    ThumbnailOperation, UploadFileMetadata, UploadFileRequest, UploadFileResponse,
    UploadMultipartChunkRequest, UploadMultipartChunkResponse, VideoOperation, WatermarkOperation,
};
pub use hooks::{
    DeliveryHookRequest as ProtoDeliveryHookRequest,
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
};
pub use hooks::hook_extension_client::HookExtensionClient;
