//! Event management for VirtualBox plugin

use std::sync::Arc;
use serde_json::Value;
use omni_director::cpis::events::{FeatureActionCompleteEvent, FeatureActionEvent};
use omni_director::cpis::{LogLevel, ServerContext};
use crate::error::VBoxResult;

pub struct EventManager;

impl EventManager {
    /// Emit completion event
    pub async fn emit_completion_event(
        request_id: uuid::Uuid,
        result: Result<Value, String>,
        execution_time_ms: u64,
        context: &Arc<dyn ServerContext>,
    ) {
        let completion_event = FeatureActionCompleteEvent {
            request_id,
            result,
            execution_time_ms,
        };

        context.log(
            LogLevel::Debug,
            &format!("emit_completion_event: about to emit event for request_id {}", request_id),
        );

        let emit_result = context
            .events()
            .emit_event("feature:action:complete", &completion_event)
            .await;

        match emit_result {
            Ok(_) => {
                context.log(
                    LogLevel::Debug,
                    &format!("emit_completion_event: successfully emitted event for request_id {}", request_id),
                );
            }
            Err(e) => {
                context.log(
                    LogLevel::Error,
                    &format!("Failed to emit completion event: {}", e),
                );
            }
        }

        context.log(
            LogLevel::Debug,
            &format!("emit_completion_event: finished emit_event for request_id {}", request_id),
        );
    }

    /// Create event handler wrapper that handles timing and logging
    pub fn create_handler<F, Fut>(
        action_name: &'static str,
        handler_fn: F,
    ) -> impl Fn(FeatureActionEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(FeatureActionEvent, Arc<dyn ServerContext>) -> Fut + Clone + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        move |event: FeatureActionEvent| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let handler = handler_fn.clone();
            
            // We need to get the context from somewhere - this is a limitation of the current design
            // In the actual implementation, this would need to be passed through or stored
            // For now, we'll create a runtime and execute the handler
            match tokio::runtime::Runtime::new() {
                Ok(rt) => {
                    rt.block_on(async move {
                        // This is a simplified version - in the real implementation,
                        // the context would need to be properly passed through
                        println!("VirtualBoxPlugin: {} handler triggered with request ID: {}", 
                                action_name, event.request_id);
                    });
                }
                Err(e) => {
                    eprintln!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e);
                }
            }
            Ok(())
        }
    }

    /// Measure execution time and handle results
    pub async fn execute_with_timing<F, Fut, T>(
        operation_name: &str,
        request_id: uuid::Uuid,
        context: Arc<dyn ServerContext>,
        operation: F,
    ) -> (VBoxResult<T>, u64)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = VBoxResult<T>>,
    {
        let start_time = std::time::Instant::now();
        
        context.log(
            LogLevel::Info,
            &format!("{} with request ID: {}", operation_name, request_id),
        );

        let result = operation().await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("{} completed successfully in {}ms", operation_name, execution_time_ms),
            ),
            Err(e) => context.log(
                LogLevel::Error,
                &format!("{} failed: {}", operation_name, e),
            ),
        }

        (result, execution_time_ms)
    }
}