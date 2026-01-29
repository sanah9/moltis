use std::sync::Arc;

use anyhow::{bail, Result};
use tracing::{debug, info, trace, warn};

use crate::model::{CompletionResponse, LlmProvider};
use crate::tool_registry::ToolRegistry;

/// Maximum number of tool-call loop iterations before giving up.
const MAX_ITERATIONS: usize = 25;

/// Result of running the agent loop.
#[derive(Debug)]
pub struct AgentRunResult {
    pub text: String,
    pub iterations: usize,
    pub tool_calls_made: usize,
}

/// Callback for streaming events out of the runner.
pub type OnEvent = Box<dyn Fn(RunnerEvent) + Send + Sync>;

/// Events emitted during the agent run.
#[derive(Debug, Clone)]
pub enum RunnerEvent {
    /// LLM is processing (show a "thinking" indicator).
    Thinking,
    /// LLM finished thinking (hide the indicator).
    ThinkingDone,
    ToolCallStart { id: String, name: String },
    ToolCallEnd { id: String, name: String, success: bool },
    TextDelta(String),
    Iteration(usize),
}

/// Run the agent loop: send messages to the LLM, execute tool calls, repeat.
pub async fn run_agent_loop(
    provider: Arc<dyn LlmProvider>,
    tools: &ToolRegistry,
    system_prompt: &str,
    user_message: &str,
    on_event: Option<&OnEvent>,
) -> Result<AgentRunResult> {
    let tool_schemas = tools.list_schemas();

    let mut messages: Vec<serde_json::Value> = vec![
        serde_json::json!({
            "role": "system",
            "content": system_prompt,
        }),
        serde_json::json!({
            "role": "user",
            "content": user_message,
        }),
    ];

    let mut iterations = 0;
    let mut total_tool_calls = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            warn!("agent loop exceeded max iterations ({})", MAX_ITERATIONS);
            bail!("agent loop exceeded max iterations");
        }

        if let Some(cb) = on_event {
            cb(RunnerEvent::Iteration(iterations));
        }

        debug!(iteration = iterations, messages_count = messages.len(), "calling LLM");
        trace!(iteration = iterations, messages = %serde_json::to_string(&messages).unwrap_or_default(), "LLM request messages");

        if let Some(cb) = on_event {
            cb(RunnerEvent::Thinking);
        }

        let response: CompletionResponse = provider.complete(&messages, &tool_schemas).await?;

        if let Some(cb) = on_event {
            cb(RunnerEvent::ThinkingDone);
        }

        debug!(
            iteration = iterations,
            has_text = response.text.is_some(),
            tool_calls_count = response.tool_calls.len(),
            input_tokens = response.usage.input_tokens,
            output_tokens = response.usage.output_tokens,
            "LLM response received"
        );
        if let Some(ref text) = response.text {
            trace!(iteration = iterations, text = %text, "LLM response text");
        }
        for tc in &response.tool_calls {
            debug!(
                iteration = iterations,
                tool_call_id = %tc.id,
                tool_name = %tc.name,
                arguments = %tc.arguments,
                "LLM requested tool call"
            );
        }

        // If no tool calls, return the text response.
        if response.tool_calls.is_empty() {
            let text = response.text.unwrap_or_default();

            info!(
                iterations,
                tool_calls = total_tool_calls,
                "agent loop complete"
            );
            return Ok(AgentRunResult {
                text,
                iterations,
                tool_calls_made: total_tool_calls,
            });
        }

        // Append assistant message with tool calls.
        let tool_calls_json: Vec<serde_json::Value> = response
            .tool_calls
            .iter()
            .map(|tc| {
                serde_json::json!({
                    "id": tc.id,
                    "type": "function",
                    "function": {
                        "name": tc.name,
                        "arguments": tc.arguments.to_string(),
                    }
                })
            })
            .collect();

        let mut assistant_msg = serde_json::json!({
            "role": "assistant",
            "tool_calls": tool_calls_json,
        });
        if let Some(ref text) = response.text {
            assistant_msg["content"] = serde_json::Value::String(text.clone());
        }
        messages.push(assistant_msg);

        // Execute each tool call.
        for tc in &response.tool_calls {
            total_tool_calls += 1;

            if let Some(cb) = on_event {
                cb(RunnerEvent::ToolCallStart {
                    id: tc.id.clone(),
                    name: tc.name.clone(),
                });
            }

            debug!(tool = %tc.name, id = %tc.id, args = %tc.arguments, "executing tool");

            let result = if let Some(tool) = tools.get(&tc.name) {
                match tool.execute(tc.arguments.clone()).await {
                    Ok(val) => {
                        info!(tool = %tc.name, id = %tc.id, "tool execution succeeded");
                        trace!(tool = %tc.name, result = %val, "tool result");
                        if let Some(cb) = on_event {
                            cb(RunnerEvent::ToolCallEnd {
                                id: tc.id.clone(),
                                name: tc.name.clone(),
                                success: true,
                            });
                        }
                        serde_json::json!({ "result": val })
                    }
                    Err(e) => {
                        warn!(tool = %tc.name, id = %tc.id, error = %e, "tool execution failed");
                        if let Some(cb) = on_event {
                            cb(RunnerEvent::ToolCallEnd {
                                id: tc.id.clone(),
                                name: tc.name.clone(),
                                success: false,
                            });
                        }
                        serde_json::json!({ "error": e.to_string() })
                    }
                }
            } else {
                warn!(tool = %tc.name, id = %tc.id, "unknown tool requested by LLM");
                if let Some(cb) = on_event {
                    cb(RunnerEvent::ToolCallEnd {
                        id: tc.id.clone(),
                        name: tc.name.clone(),
                        success: false,
                    });
                }
                serde_json::json!({ "error": format!("unknown tool: {}", tc.name) })
            };

            let tool_result_str = result.to_string();
            debug!(
                tool = %tc.name,
                id = %tc.id,
                result_len = tool_result_str.len(),
                "appending tool result to messages"
            );
            trace!(tool = %tc.name, content = %tool_result_str, "tool result message content");

            messages.push(serde_json::json!({
                "role": "tool",
                "tool_call_id": tc.id,
                "content": tool_result_str,
            }));
        }
    }
}

/// Convenience wrapper matching the old stub signature.
pub async fn run_agent(
    _agent_id: &str,
    _session_key: &str,
    _message: &str,
) -> Result<String> {
    bail!("run_agent requires a configured provider and tool registry; use run_agent_loop instead")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CompletionResponse, LlmProvider, StreamEvent, ToolCall, Usage};
    use async_trait::async_trait;
    use std::pin::Pin;
    use tokio_stream::Stream;

    /// A mock provider that returns text on the first call.
    struct MockProvider {
        response_text: String,
    }

    #[async_trait]
    impl LlmProvider for MockProvider {
        fn name(&self) -> &str { "mock" }
        fn id(&self) -> &str { "mock-model" }

        async fn complete(
            &self,
            _messages: &[serde_json::Value],
            _tools: &[serde_json::Value],
        ) -> Result<CompletionResponse> {
            Ok(CompletionResponse {
                text: Some(self.response_text.clone()),
                tool_calls: vec![],
                usage: Usage { input_tokens: 10, output_tokens: 5 },
            })
        }

        fn stream(
            &self,
            _messages: Vec<serde_json::Value>,
        ) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send + '_>> {
            Box::pin(tokio_stream::empty())
        }
    }

    #[tokio::test]
    async fn test_simple_text_response() {
        let provider = Arc::new(MockProvider {
            response_text: "Hello!".into(),
        });
        let tools = ToolRegistry::new();
        let result = run_agent_loop(
            provider,
            &tools,
            "You are a test bot.",
            "Hi",
            None,
        )
        .await
        .unwrap();
        assert_eq!(result.text, "Hello!");
        assert_eq!(result.iterations, 1);
        assert_eq!(result.tool_calls_made, 0);
    }

    /// Mock provider that makes one tool call then returns text.
    struct ToolCallingProvider {
        call_count: std::sync::atomic::AtomicUsize,
    }

    #[async_trait]
    impl LlmProvider for ToolCallingProvider {
        fn name(&self) -> &str { "mock" }
        fn id(&self) -> &str { "mock-model" }

        async fn complete(
            &self,
            _messages: &[serde_json::Value],
            _tools: &[serde_json::Value],
        ) -> Result<CompletionResponse> {
            let count = self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count == 0 {
                Ok(CompletionResponse {
                    text: None,
                    tool_calls: vec![ToolCall {
                        id: "call_1".into(),
                        name: "echo_tool".into(),
                        arguments: serde_json::json!({"text": "hi"}),
                    }],
                    usage: Usage { input_tokens: 10, output_tokens: 5 },
                })
            } else {
                Ok(CompletionResponse {
                    text: Some("Done!".into()),
                    tool_calls: vec![],
                    usage: Usage { input_tokens: 20, output_tokens: 10 },
                })
            }
        }

        fn stream(
            &self,
            _messages: Vec<serde_json::Value>,
        ) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send + '_>> {
            Box::pin(tokio_stream::empty())
        }
    }

    /// Simple echo tool for testing.
    struct EchoTool;

    #[async_trait]
    impl crate::tool_registry::AgentTool for EchoTool {
        fn name(&self) -> &str { "echo_tool" }
        fn description(&self) -> &str { "Echoes input" }
        fn parameters_schema(&self) -> serde_json::Value {
            serde_json::json!({"type": "object", "properties": {"text": {"type": "string"}}})
        }
        async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value> {
            Ok(params)
        }
    }

    #[tokio::test]
    async fn test_tool_call_loop() {
        let provider = Arc::new(ToolCallingProvider {
            call_count: std::sync::atomic::AtomicUsize::new(0),
        });
        let mut tools = ToolRegistry::new();
        tools.register(Box::new(EchoTool));

        let result = run_agent_loop(
            provider,
            &tools,
            "You are a test bot.",
            "Use the tool",
            None,
        )
        .await
        .unwrap();

        assert_eq!(result.text, "Done!");
        assert_eq!(result.iterations, 2);
        assert_eq!(result.tool_calls_made, 1);
    }

    /// A tool that actually runs shell commands (test-only, mirrors ExecTool).
    struct TestExecTool;

    #[async_trait]
    impl crate::tool_registry::AgentTool for TestExecTool {
        fn name(&self) -> &str { "exec" }
        fn description(&self) -> &str { "Execute a shell command" }
        fn parameters_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Shell command to execute" }
                },
                "required": ["command"]
            })
        }
        async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value> {
            let command = params["command"].as_str().unwrap_or("echo noop");
            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .await?;
            Ok(serde_json::json!({
                "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
                "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
                "exit_code": output.status.code().unwrap_or(-1),
            }))
        }
    }

    /// Mock provider that calls the "exec" tool and verifies the result is fed back.
    struct ExecSimulatingProvider {
        call_count: std::sync::atomic::AtomicUsize,
    }

    #[async_trait]
    impl LlmProvider for ExecSimulatingProvider {
        fn name(&self) -> &str { "mock" }
        fn id(&self) -> &str { "mock-model" }

        async fn complete(
            &self,
            messages: &[serde_json::Value],
            _tools: &[serde_json::Value],
        ) -> Result<CompletionResponse> {
            let count = self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count == 0 {
                Ok(CompletionResponse {
                    text: None,
                    tool_calls: vec![ToolCall {
                        id: "call_exec_1".into(),
                        name: "exec".into(),
                        arguments: serde_json::json!({"command": "echo hello"}),
                    }],
                    usage: Usage { input_tokens: 10, output_tokens: 5 },
                })
            } else {
                // Verify tool result is in messages
                let tool_msg = messages.iter().find(|m| {
                    m["role"].as_str() == Some("tool")
                });
                let tool_content = tool_msg
                    .and_then(|m| m["content"].as_str())
                    .unwrap_or("");

                assert!(
                    tool_content.contains("hello"),
                    "tool result should contain 'hello', got: {tool_content}"
                );

                let parsed: serde_json::Value = serde_json::from_str(tool_content)
                    .expect("tool result should be valid JSON");
                let stdout = parsed["result"]["stdout"].as_str().unwrap_or("");
                assert!(stdout.contains("hello"), "stdout should contain 'hello', got: {stdout}");
                assert_eq!(parsed["result"]["exit_code"].as_i64().unwrap_or(-1), 0);

                Ok(CompletionResponse {
                    text: Some(format!("The output was: {}", stdout.trim())),
                    tool_calls: vec![],
                    usage: Usage { input_tokens: 20, output_tokens: 10 },
                })
            }
        }

        fn stream(
            &self,
            _messages: Vec<serde_json::Value>,
        ) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send + '_>> {
            Box::pin(tokio_stream::empty())
        }
    }

    #[tokio::test]
    async fn test_exec_tool_end_to_end() {
        let provider = Arc::new(ExecSimulatingProvider {
            call_count: std::sync::atomic::AtomicUsize::new(0),
        });

        let mut tools = ToolRegistry::new();
        tools.register(Box::new(TestExecTool));

        let events: Arc<std::sync::Mutex<Vec<RunnerEvent>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&events);
        let on_event: OnEvent = Box::new(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        let result = run_agent_loop(
            provider,
            &tools,
            "You are a test bot.",
            "Run echo hello",
            Some(&on_event),
        )
        .await
        .unwrap();

        assert!(
            result.text.contains("hello"),
            "final text should contain 'hello', got: {}",
            result.text
        );
        assert_eq!(result.iterations, 2);
        assert_eq!(result.tool_calls_made, 1);

        let evts = events.lock().unwrap();
        let event_names: Vec<&str> = evts.iter().map(|e| match e {
            RunnerEvent::Thinking => "thinking",
            RunnerEvent::ThinkingDone => "thinking_done",
            RunnerEvent::ToolCallStart { .. } => "tool_call_start",
            RunnerEvent::ToolCallEnd { .. } => "tool_call_end",
            RunnerEvent::TextDelta(_) => "text_delta",
            RunnerEvent::Iteration(_) => "iteration",
        }).collect();

        assert!(event_names.contains(&"tool_call_start"), "should emit tool_call_start");
        assert!(event_names.contains(&"tool_call_end"), "should emit tool_call_end");
        assert!(event_names.contains(&"thinking"), "should emit thinking");

        let tool_end = evts.iter().find(|e| matches!(e, RunnerEvent::ToolCallEnd { .. }));
        if let Some(RunnerEvent::ToolCallEnd { success, name, .. }) = tool_end {
            assert!(success, "exec tool should succeed");
            assert_eq!(name, "exec");
        }
    }
}
