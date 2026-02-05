//! Metric name and label definitions.
//!
//! This module defines all metric names and common label keys used throughout moltis.
//! Centralizing these definitions ensures consistency and makes it easier to document
//! what metrics are available.

/// HTTP request metrics
pub mod http {
    /// Total number of HTTP requests handled
    pub const REQUESTS_TOTAL: &str = "moltis_http_requests_total";
    /// Duration of HTTP requests in seconds
    pub const REQUEST_DURATION_SECONDS: &str = "moltis_http_request_duration_seconds";
    /// Number of currently in-flight HTTP requests
    pub const REQUESTS_IN_FLIGHT: &str = "moltis_http_requests_in_flight";
    /// Total bytes received in HTTP requests
    pub const REQUEST_BYTES_TOTAL: &str = "moltis_http_request_bytes_total";
    /// Total bytes sent in HTTP responses
    pub const RESPONSE_BYTES_TOTAL: &str = "moltis_http_response_bytes_total";
}

/// WebSocket metrics
pub mod websocket {
    /// Total number of WebSocket connections established
    pub const CONNECTIONS_TOTAL: &str = "moltis_websocket_connections_total";
    /// Number of currently active WebSocket connections
    pub const CONNECTIONS_ACTIVE: &str = "moltis_websocket_connections_active";
    /// Total number of WebSocket messages received
    pub const MESSAGES_RECEIVED_TOTAL: &str = "moltis_websocket_messages_received_total";
    /// Total number of WebSocket messages sent
    pub const MESSAGES_SENT_TOTAL: &str = "moltis_websocket_messages_sent_total";
    /// WebSocket message processing duration in seconds
    pub const MESSAGE_DURATION_SECONDS: &str = "moltis_websocket_message_duration_seconds";
}

/// LLM/Agent metrics
pub mod llm {
    /// Total number of LLM completions requested
    pub const COMPLETIONS_TOTAL: &str = "moltis_llm_completions_total";
    /// Duration of LLM completion requests in seconds
    pub const COMPLETION_DURATION_SECONDS: &str = "moltis_llm_completion_duration_seconds";
    /// Total input tokens processed
    pub const INPUT_TOKENS_TOTAL: &str = "moltis_llm_input_tokens_total";
    /// Total output tokens generated
    pub const OUTPUT_TOKENS_TOTAL: &str = "moltis_llm_output_tokens_total";
    /// Total cache read tokens (for providers that support caching)
    pub const CACHE_READ_TOKENS_TOTAL: &str = "moltis_llm_cache_read_tokens_total";
    /// Total cache write tokens (for providers that support caching)
    pub const CACHE_WRITE_TOKENS_TOTAL: &str = "moltis_llm_cache_write_tokens_total";
    /// LLM completion errors
    pub const COMPLETION_ERRORS_TOTAL: &str = "moltis_llm_completion_errors_total";
    /// Time to first token in seconds (streaming latency)
    pub const TIME_TO_FIRST_TOKEN_SECONDS: &str = "moltis_llm_time_to_first_token_seconds";
    /// Tokens per second generation rate
    pub const TOKENS_PER_SECOND: &str = "moltis_llm_tokens_per_second";
}

/// Session metrics
pub mod session {
    /// Total number of sessions created
    pub const CREATED_TOTAL: &str = "moltis_sessions_created_total";
    /// Number of currently active sessions
    pub const ACTIVE: &str = "moltis_sessions_active";
    /// Total number of messages in sessions
    pub const MESSAGES_TOTAL: &str = "moltis_session_messages_total";
    /// Session duration in seconds
    pub const DURATION_SECONDS: &str = "moltis_session_duration_seconds";
}

/// Chat metrics
pub mod chat {
    /// Total number of chat messages sent
    pub const MESSAGES_SENT_TOTAL: &str = "moltis_chat_messages_sent_total";
    /// Total number of chat messages received
    pub const MESSAGES_RECEIVED_TOTAL: &str = "moltis_chat_messages_received_total";
    /// Chat message processing duration in seconds
    pub const PROCESSING_DURATION_SECONDS: &str = "moltis_chat_processing_duration_seconds";
}

/// Tool execution metrics
pub mod tools {
    /// Total number of tool executions
    pub const EXECUTIONS_TOTAL: &str = "moltis_tool_executions_total";
    /// Tool execution duration in seconds
    pub const EXECUTION_DURATION_SECONDS: &str = "moltis_tool_execution_duration_seconds";
    /// Tool execution errors
    pub const EXECUTION_ERRORS_TOTAL: &str = "moltis_tool_execution_errors_total";
    /// Number of currently running tool executions
    pub const EXECUTIONS_IN_FLIGHT: &str = "moltis_tool_executions_in_flight";
}

/// Sandbox metrics
pub mod sandbox {
    /// Total number of sandbox command executions
    pub const COMMAND_EXECUTIONS_TOTAL: &str = "moltis_sandbox_command_executions_total";
    /// Sandbox command execution duration in seconds
    pub const COMMAND_DURATION_SECONDS: &str = "moltis_sandbox_command_duration_seconds";
    /// Sandbox command errors
    pub const COMMAND_ERRORS_TOTAL: &str = "moltis_sandbox_command_errors_total";
    /// Number of sandbox images available
    pub const IMAGES_AVAILABLE: &str = "moltis_sandbox_images_available";
}

/// MCP (Model Context Protocol) metrics
pub mod mcp {
    /// Total number of MCP server connections
    pub const SERVER_CONNECTIONS_TOTAL: &str = "moltis_mcp_server_connections_total";
    /// Number of currently connected MCP servers
    pub const SERVERS_CONNECTED: &str = "moltis_mcp_servers_connected";
    /// Total number of MCP tool calls
    pub const TOOL_CALLS_TOTAL: &str = "moltis_mcp_tool_calls_total";
    /// MCP tool call duration in seconds
    pub const TOOL_CALL_DURATION_SECONDS: &str = "moltis_mcp_tool_call_duration_seconds";
    /// MCP tool call errors
    pub const TOOL_CALL_ERRORS_TOTAL: &str = "moltis_mcp_tool_call_errors_total";
    /// Total number of MCP resource reads
    pub const RESOURCE_READS_TOTAL: &str = "moltis_mcp_resource_reads_total";
    /// Total number of MCP prompt fetches
    pub const PROMPT_FETCHES_TOTAL: &str = "moltis_mcp_prompt_fetches_total";
}

/// Channel metrics (Telegram, etc.)
pub mod channels {
    /// Total number of channel messages received
    pub const MESSAGES_RECEIVED_TOTAL: &str = "moltis_channel_messages_received_total";
    /// Total number of channel messages sent
    pub const MESSAGES_SENT_TOTAL: &str = "moltis_channel_messages_sent_total";
    /// Number of active channels
    pub const ACTIVE: &str = "moltis_channels_active";
    /// Channel errors
    pub const ERRORS_TOTAL: &str = "moltis_channel_errors_total";
}

/// Memory/embedding metrics
pub mod memory {
    /// Total number of memory searches performed
    pub const SEARCHES_TOTAL: &str = "moltis_memory_searches_total";
    /// Memory search duration in seconds
    pub const SEARCH_DURATION_SECONDS: &str = "moltis_memory_search_duration_seconds";
    /// Total number of embeddings generated
    pub const EMBEDDINGS_GENERATED_TOTAL: &str = "moltis_memory_embeddings_generated_total";
    /// Number of documents in memory
    pub const DOCUMENTS_COUNT: &str = "moltis_memory_documents_count";
    /// Total memory size in bytes
    pub const SIZE_BYTES: &str = "moltis_memory_size_bytes";
}

/// Plugin metrics
pub mod plugins {
    /// Number of loaded plugins
    pub const LOADED: &str = "moltis_plugins_loaded";
    /// Total plugin executions
    pub const EXECUTIONS_TOTAL: &str = "moltis_plugin_executions_total";
    /// Plugin execution duration in seconds
    pub const EXECUTION_DURATION_SECONDS: &str = "moltis_plugin_execution_duration_seconds";
    /// Plugin errors
    pub const ERRORS_TOTAL: &str = "moltis_plugin_errors_total";
}

/// Cron job metrics
pub mod cron {
    /// Number of scheduled cron jobs
    pub const JOBS_SCHEDULED: &str = "moltis_cron_jobs_scheduled";
    /// Total cron job executions
    pub const EXECUTIONS_TOTAL: &str = "moltis_cron_executions_total";
    /// Cron job execution duration in seconds
    pub const EXECUTION_DURATION_SECONDS: &str = "moltis_cron_execution_duration_seconds";
    /// Cron job errors
    pub const ERRORS_TOTAL: &str = "moltis_cron_errors_total";
}

/// Authentication metrics
pub mod auth {
    /// Total login attempts
    pub const LOGIN_ATTEMPTS_TOTAL: &str = "moltis_auth_login_attempts_total";
    /// Successful logins
    pub const LOGIN_SUCCESS_TOTAL: &str = "moltis_auth_login_success_total";
    /// Failed logins
    pub const LOGIN_FAILURES_TOTAL: &str = "moltis_auth_login_failures_total";
    /// Active sessions
    pub const ACTIVE_SESSIONS: &str = "moltis_auth_active_sessions";
    /// API key authentications
    pub const API_KEY_AUTH_TOTAL: &str = "moltis_auth_api_key_auth_total";
}

/// System/runtime metrics
pub mod system {
    /// Process uptime in seconds
    pub const UPTIME_SECONDS: &str = "moltis_uptime_seconds";
    /// Build information (labels: version, commit, build_date)
    pub const BUILD_INFO: &str = "moltis_build_info";
    /// Number of connected clients
    pub const CONNECTED_CLIENTS: &str = "moltis_connected_clients";
}

/// Common label keys used across metrics
pub mod labels {
    pub const ENDPOINT: &str = "endpoint";
    pub const METHOD: &str = "method";
    pub const STATUS: &str = "status";
    pub const PROVIDER: &str = "provider";
    pub const MODEL: &str = "model";
    pub const TOOL: &str = "tool";
    pub const CHANNEL: &str = "channel";
    pub const SERVER: &str = "server";
    pub const ERROR_TYPE: &str = "error_type";
    pub const ROLE: &str = "role";
    pub const SUCCESS: &str = "success";
}

/// Standard histogram buckets for different metric types
pub mod buckets {
    use once_cell::sync::Lazy;

    /// HTTP request duration buckets (in seconds)
    /// Covers 1ms to 60s
    pub static HTTP_DURATION: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0,
        ]
    });

    /// LLM completion duration buckets (in seconds)
    /// Covers 100ms to 5 minutes (LLM calls can be slow)
    pub static LLM_DURATION: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 15.0, 30.0, 60.0, 120.0, 180.0, 300.0,
        ]
    });

    /// Time to first token buckets (in seconds)
    /// Covers 10ms to 30s
    pub static TTFT: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 30.0,
        ]
    });

    /// Tool execution duration buckets (in seconds)
    /// Covers 1ms to 5 minutes
    pub static TOOL_DURATION: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            0.001, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0,
        ]
    });

    /// Token count buckets
    /// Covers 1 to 200k tokens
    pub static TOKEN_COUNT: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            1.0, 10.0, 50.0, 100.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0, 32000.0,
            64000.0, 128000.0, 200000.0,
        ]
    });

    /// Tokens per second buckets
    /// Covers 1 to 500 tokens/sec
    pub static TOKENS_PER_SECOND: Lazy<Vec<f64>> = Lazy::new(|| {
        vec![
            1.0, 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 75.0, 100.0, 150.0, 200.0, 300.0, 500.0,
        ]
    });
}
