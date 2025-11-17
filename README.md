# Flare Proto

Flare IM gRPC Protocol Definitions - 为 Flare IM 提供统一的 gRPC 协议定义，支持客户端和服务端使用。

---

## 📋 概述

`flare-proto` 是 Flare IM 的 gRPC 协议定义库，包含所有服务的 Protobuf 定义和生成的 Rust 代码。该库设计为同时支持客户端和服务端使用。

---

## 🚀 快速开始

### 安装

```toml
[dependencies]
flare-proto = { path = "../flare-proto" }
# 或者从 crates.io
# flare-proto = "0.1.0"
```

### 使用

```rust
use flare_proto::access_gateway::*;
use flare_proto::signaling::*;
use flare_proto::push::*;
use flare_proto::storage::*;
use flare_proto::media::*;
```

---

## 📦 包含的协议

### 1. Access Gateway (`access_gateway`)

业务系统推送消息给客户端的接口，包括：
- PushMessage：推送消息给客户端
- BatchPushMessage：批量推送消息
- QueryUserConnections：查询用户连接状态

### 2. Signaling (`signaling`)

信令系统服务接口，包括：
- Login, Logout
- UpdateOnlineStatus, GetOnlineStatus
- RouteMessage
- RegisterService

### 3. Push (`push`)

推送系统服务接口，包括：
- PushMessage
- BroadcastMessage
- SendOfflineNotification

### 4. Storage (`storage`)

存储系统服务接口，包括：
- StoreMessage, BatchStoreMessage
- QueryMessages
- DeleteMessage, GetMessageById

### 5. Media (`media`)

媒体服务接口，包括：
- UploadFile (流式上传，内建去重)
- CreateReference / DeleteReference（引用管理）
- ListReferences（引用查询）、CleanupOrphanedAssets（孤儿清理）
- GetFileUrl, GetFileInfo
- DeleteFile
- ProcessImage, ProcessVideo

---

## 🔧 特性

### 服务端特性

启用服务端特性以使用服务端代码：

```toml
[dependencies]
flare-proto = { path = "../flare-proto", features = ["server"] }
```

### 客户端特性

启用客户端特性以使用客户端代码：

```toml
[dependencies]
flare-proto = { path = "../flare-proto", features = ["client"] }
```

---

## 📚 相关文档

- [gRPC 服务设计](../doc/GRPC_SERVICES.md)
- [通信核心层设计](../doc/CORE_COMMUNICATION_LAYER.md)

---

**维护者**: Flare IM Architecture Team  
**最后更新**: 2025-11-08  
**版本**: 0.1.0

