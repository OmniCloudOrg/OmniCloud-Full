//! Event handlers for VirtualBox operations

use std::sync::Arc;
use omni_director::cpis::events::FeatureActionEvent;
use omni_director::cpis::{LogLevel, ServerContext};
use crate::operations::VBoxOperations;
use crate::events::EventManager;
use crate::error::VBoxResult;

#[derive(Clone)]
pub struct EventHandlers {
    operations: VBoxOperations,
}

impl EventHandlers {
    pub fn new(operations: VBoxOperations) -> Self {
        Self { operations }
    }

    /// Register all event handlers - NO MATCH STATEMENTS!
    pub async fn register_all_handlers(&self, context: Arc<dyn ServerContext>) -> VBoxResult<()> {
        let events = context.events();

        // Create VM handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:create_vm", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_create_vm(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // Delete VM handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:delete_vm", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_delete_vm(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // Start VM handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:start_vm", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_start_vm(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // Stop VM handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:stop_vm", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_stop_vm(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // Get VM Info handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:get_vm_info", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_get_vm_info(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // List VMs handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:list_vms", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_list_vms(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        // Create Snapshot handler
        {
            let ops = self.operations.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:create_snapshot", move |event| {
                    let operations = ops.clone();
                    let context = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                Self::handle_create_snapshot(operations, event, context).await;
                            });
                        }
                        Err(e) => {
                            context.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| crate::error::VBoxError::OperationFailed(e.to_string()))?;
        }

        Ok(())
    }

    /// Handle create VM events
    async fn handle_create_vm(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Creating VM",
            event.request_id,
            context.clone(),
            || async { operations.create_vm(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle delete VM events
    async fn handle_delete_vm(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Deleting VM",
            event.request_id,
            context.clone(),
            || async { operations.delete_vm(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle start VM events
    async fn handle_start_vm(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Starting VM",
            event.request_id,
            context.clone(),
            || async { operations.start_vm(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle stop VM events
    async fn handle_stop_vm(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        println!(
            "VirtualBoxPlugin: stop_vm handler triggered with request ID: {}",
            event.request_id
        );

        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Stopping VM",
            event.request_id,
            context.clone(),
            || async { operations.stop_vm(&event.arguments).await },
        ).await;

        context.log(LogLevel::Info, "VirtualBoxPlugin: stop_vm about to emit completion event");
        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle get VM info events
    async fn handle_get_vm_info(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Getting VM info",
            event.request_id,
            context.clone(),
            || async { operations.get_vm_info(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle list VMs events
    async fn handle_list_vms(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Listing VMs",
            event.request_id,
            context.clone(),
            || async { operations.list_vms(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }

    /// Handle create snapshot events
    async fn handle_create_snapshot(
        operations: VBoxOperations,
        event: FeatureActionEvent,
        context: Arc<dyn ServerContext>,
    ) {
        let (result, execution_time_ms) = EventManager::execute_with_timing(
            "Creating snapshot",
            event.request_id,
            context.clone(),
            || async { operations.create_snapshot(&event.arguments).await },
        ).await;

        let result = result.map_err(|e| e.to_string());
        EventManager::emit_completion_event(event.request_id, result, execution_time_ms, &context).await;
    }
}