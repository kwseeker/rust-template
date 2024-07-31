/// 用户请求消息
struct Query {
    trans_type: TransferMode,
}

/// ChatGLM响应消息
struct Reply {
}

enum TransferMode {
    /// 流式传输
    Sse,
    Sync,
    Async
}