//! # VirtualBox Plugin for OmniDirector - Event-Driven Implementation
//!
//! This plugin provides VM_Manage feature implementation using VirtualBox.
//! NO MATCH STATEMENTS - pure event-driven architecture!

use chrono::Utc;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;

// Import the plugin system types
use async_trait::async_trait;
use omni_director::cpis::events::{FeatureActionCompleteEvent, FeatureActionEvent};
use omni_director::cpis::{LogLevel, Plugin, PluginError, ServerContext};


// VirtualBox Plugin Implementation - PURE EVENT-DRIVEN, NO MATCH STATEMENTS!
#[derive(Clone)]
pub struct VirtualBoxPlugin {
    vboxmanage_path: String,
    default_vm_folder: Option<String>,
    max_concurrent_vms: u32,
}

impl VirtualBoxPlugin {
    pub fn new() -> Self {
        Self {
            vboxmanage_path: Self::find_vboxmanage(),
            default_vm_folder: None,
            max_concurrent_vms: 10,
        }
    }

    /// Find VBoxManage executable path
    fn find_vboxmanage() -> String {
        let common_paths = if cfg!(windows) {
            vec![
                "C:\\Program Files\\Oracle\\VirtualBox\\VBoxManage.exe",
                "C:\\Program Files (x86)\\Oracle\\VirtualBox\\VBoxManage.exe",
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                "/Applications/VirtualBox.app/Contents/MacOS/VBoxManage",
                "/usr/local/bin/VBoxManage",
            ]
        } else {
            vec!["/usr/bin/VBoxManage", "/usr/local/bin/VBoxManage"]
        };

        for path in common_paths {
            if std::path::Path::new(path).exists() {
                return path.to_string();
            }
        }

        "VBoxManage".to_string()
    }

    /// Execute VBoxManage command
    async fn execute_vboxmanage(&self, args: &[&str]) -> Result<String, PluginError> {
        let output = Command::new(&self.vboxmanage_path)
            .args(args)
            .output()
            .map_err(|e| {
                PluginError::ExecutionFailed(format!("Failed to execute VBoxManage: {}", e))
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(PluginError::ExecutionFailed(format!(
                "VBoxManage failed: {}",
                error
            )))
        }
    }

    /// Create VM - called directly by event handler
    async fn create_vm(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Creating VM with request ID: {}", event.request_id),
        );

        let result = self.create_vm_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VM created successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM creation failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Delete VM - called directly by event handler
    async fn delete_vm(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Deleting VM with request ID: {}", event.request_id),
        );

        let result = self.delete_vm_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VM deleted successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM deletion failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Start VM - called directly by event handler
    async fn start_vm(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Starting VM with request ID: {}", event.request_id),
        );

        let result = self.start_vm_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VM started successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM start failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Stop VM - called directly by event handler
    async fn stop_vm(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        println!(
            "VirtualBoxPlugin: stop_vm handler triggered with request ID: {}",
            event.request_id
        );
        context.log(
            LogLevel::Info,
            &format!("Stopping VM with request ID: {}", event.request_id),
        );

        let result = self.stop_vm_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VM stopped successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM stop failed: {}", e)),
        }

        context.log(LogLevel::Info, "VirtualBoxPlugin: stop_vm about to emit completion event");
        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Get VM Info - called directly by event handler
    async fn get_vm_info(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Getting VM info with request ID: {}", event.request_id),
        );

        let result = self.get_vm_info_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VM info retrieved successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM info retrieval failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// List VMs - called directly by event handler
    async fn list_vms(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Listing VMs with request ID: {}", event.request_id),
        );

        let result = self.list_vms_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("VMs listed successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("VM listing failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Create Snapshot - called directly by event handler
    async fn create_snapshot(&self, event: FeatureActionEvent, context: Arc<dyn ServerContext>) {
        let start_time = std::time::Instant::now();
        context.log(
            LogLevel::Info,
            &format!("Creating snapshot with request ID: {}", event.request_id),
        );

        let result = self.create_snapshot_impl(&event.arguments).await;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => context.log(
                LogLevel::Info,
                &format!("Snapshot created successfully in {}ms", execution_time_ms),
            ),
            Err(e) => context.log(LogLevel::Error, &format!("Snapshot creation failed: {}", e)),
        }

        self.emit_completion_event(event.request_id, result, execution_time_ms, &context)
            .await;
    }

    /// Emit completion event
    async fn emit_completion_event(
        &self,
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

    // Implementation methods (unchanged from original)
    async fn create_vm_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing VM name")?;
        let memory_mb = args
            .get("memory_mb")
            .and_then(|v| v.as_f64())
            .ok_or("Missing memory_mb")? as u32;
        let cpu_count = args
            .get("cpu_count")
            .and_then(|v| v.as_f64())
            .ok_or("Missing cpu_count")? as u32;
        let disk_size_gb = args
            .get("disk_size_gb")
            .and_then(|v| v.as_f64())
            .unwrap_or(20.0) as u32;
        let os_type = args
            .get("os_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Linux_64");
        let network_adapter = args
            .get("network_adapter")
            .and_then(|v| v.as_str())
            .unwrap_or("NAT");

        // Create the VM
        self.execute_vboxmanage(&[
            "createvm",
            "--name",
            name,
            "--ostype",
            os_type,
            "--register",
        ])
        .await
        .map_err(|e| e.to_string())?;

        // Configure memory and CPUs
        self.execute_vboxmanage(&[
            "modifyvm",
            name,
            "--memory",
            &memory_mb.to_string(),
            "--cpus",
            &cpu_count.to_string(),
        ])
        .await
        .map_err(|e| e.to_string())?;

        // Configure network
        let nic_args = match network_adapter {
            "Bridged" => vec!["--nic1", "bridged"],
            "Host-only" => vec!["--nic1", "hostonly"],
            "Internal" => vec!["--nic1", "intnet"],
            _ => vec!["--nic1", "nat"],
        };
        let mut modify_args = vec!["modifyvm", name];
        modify_args.extend(&nic_args);
        self.execute_vboxmanage(&modify_args)
            .await
            .map_err(|e| e.to_string())?;

        // Create storage controller
        self.execute_vboxmanage(&[
            "storagectl",
            name,
            "--name",
            "SATA Controller",
            "--add",
            "sata",
            "--controller",
            "IntelAhci",
        ])
        .await
        .map_err(|e| e.to_string())?;

        // Create and attach disk
        let disk_path = format!("{}.vdi", name);
        self.execute_vboxmanage(&[
            "createhd",
            "--filename",
            &disk_path,
            "--size",
            &(disk_size_gb * 1024).to_string(),
            "--format",
            "VDI",
        ])
        .await
        .map_err(|e| e.to_string())?;
        self.execute_vboxmanage(&[
            "storageattach",
            name,
            "--storagectl",
            "SATA Controller",
            "--port",
            "0",
            "--device",
            "0",
            "--type",
            "hdd",
            "--medium",
            &disk_path,
        ])
        .await
        .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "vm_id": name,
            "name": name,
            "status": "created",
            "memory_mb": memory_mb,
            "cpu_count": cpu_count,
            "disk_size_gb": disk_size_gb
        }))
    }

    async fn delete_vm_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing vm_id")?;
        let delete_files = args
            .get("delete_files")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let _ = self
            .execute_vboxmanage(&["controlvm", vm_id, "poweroff"])
            .await;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        if delete_files {
            self.execute_vboxmanage(&["unregistervm", vm_id, "--delete"])
                .await
                .map_err(|e| e.to_string())?;
        } else {
            self.execute_vboxmanage(&["unregistervm", vm_id])
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(Value::Bool(true))
    }

    async fn start_vm_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing vm_id")?;
        let headless = args
            .get("headless")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let start_type = if headless { "headless" } else { "gui" };

        self.execute_vboxmanage(&["startvm", vm_id, "--type", start_type])
            .await
            .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "vm_id": vm_id,
            "status": "running",
            "started_at": Utc::now().to_rfc3339()
        }))
    }

    async fn stop_vm_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing vm_id")?;
        let force = args.get("force").and_then(|v| v.as_bool()).unwrap_or(false);
        let stop_command = if force { "poweroff" } else { "acpipowerbutton" };

        self.execute_vboxmanage(&["controlvm", vm_id, stop_command])
            .await
            .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "vm_id": vm_id,
            "status": "stopped",
            "stopped_at": Utc::now().to_rfc3339()
        }))
    }

    async fn get_vm_info_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing vm_id")?;
        let output = self
            .execute_vboxmanage(&["showvminfo", vm_id, "--machinereadable"])
            .await
            .map_err(|e| e.to_string())?;

        let mut info = HashMap::new();
        for line in output.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim_matches('"');
                match key {
                    "name" => {
                        info.insert("name".to_string(), Value::String(value.to_string()));
                    }
                    "VMState" => {
                        info.insert("status".to_string(), Value::String(value.to_string()));
                    }
                    "memory" => {
                        if let Ok(mem) = value.parse::<u64>() {
                            info.insert("memory_mb".to_string(), Value::Number(mem.into()));
                        }
                    }
                    "cpus" => {
                        if let Ok(cpus) = value.parse::<u64>() {
                            info.insert("cpu_count".to_string(), Value::Number(cpus.into()));
                        }
                    }
                    _ => {}
                }
            }
        }

        info.insert("vm_id".to_string(), Value::String(vm_id.to_string()));
        Ok(Value::Object(serde_json::Map::from_iter(info)))
    }

    async fn list_vms_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let include_running_only = args
            .get("include_running_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let list_arg = if include_running_only {
            "runningvms"
        } else {
            "vms"
        };
        let output = self
            .execute_vboxmanage(&["list", list_arg])
            .await
            .map_err(|e| e.to_string())?;

        let mut vms = Vec::new();
        let vm_regex = Regex::new(r#""([^"]+)"\s+\{([^}]+)\}"#).unwrap();

        for line in output.lines() {
            if let Some(captures) = vm_regex.captures(line) {
                let name = &captures[1];
                let uuid = &captures[2];
                vms.push(serde_json::json!({
                    "vm_id": name,
                    "name": name,
                    "uuid": uuid,
                    "status": "unknown"
                }));
            }
        }

        Ok(Value::Array(vms))
    }

    async fn create_snapshot_impl(&self, args: &HashMap<String, Value>) -> Result<Value, String> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing vm_id")?;
        let snapshot_name = args
            .get("snapshot_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing snapshot_name")?;
        let description = args
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut snapshot_args = vec!["snapshot", vm_id, "take", snapshot_name];
        if !description.is_empty() {
            snapshot_args.extend(&["--description", description]);
        }

        self.execute_vboxmanage(&snapshot_args)
            .await
            .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "snapshot_id": format!("{}_{}", vm_id, snapshot_name),
            "snapshot_name": snapshot_name,
            "vm_id": vm_id,
            "created_at": Utc::now().to_rfc3339()
        }))
    }
}

#[async_trait]
impl Plugin for VirtualBoxPlugin {
    fn name(&self) -> &str {
        "VirtualBox"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn declared_features(&self) -> Vec<String> {
        vec!["VM_Manage".to_string()]
    }

    async fn pre_init(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(
            LogLevel::Info,
            "VirtualBox Plugin: Registering event handlers - NO MATCH STATEMENTS!",
        );

        // Debug: Print the pointer of the event system instance used for registration
        let events = context.events();
        context.log(
            LogLevel::Info,
            &format!("VirtualBoxPlugin: context.events() instance={:p}", Arc::as_ptr(&events)),
        );

        // Each action gets its own dedicated event handler - NO MATCH STATEMENTS!


        // Create VM handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:create_vm", move |event| {
                    let p = plugin.clone();
                    let c = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                p.create_vm(event, c).await;
                            });
                        },
                        Err(e) => {
                            c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // Delete VM handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:delete_vm", move |event| {
                    let p = plugin.clone();
                    let c = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                p.delete_vm(event, c).await;
                            });
                        },
                        Err(e) => {
                            c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // Start VM handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:start_vm", move |event| {
                    let p = plugin.clone();
                    let c = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                p.start_vm(event, c).await;
                            });
                        },
                        Err(e) => {
                            c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // Stop VM handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:stop_vm", move |event| {
                    let p = plugin.clone();
                    let c = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                p.stop_vm(event, c).await;
                            });
                        },
                        Err(e) => {
                            c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // Get VM Info handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>(
                    "feature:VM_Manage:get_vm_info",
                    move |event| {
                        let p = plugin.clone();
                        let c = Arc::clone(&ctx);
                        match tokio::runtime::Runtime::new() {
                            Ok(rt) => {
                                rt.block_on(async move {
                                    p.get_vm_info(event, c).await;
                                });
                            },
                            Err(e) => {
                                c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                            }
                        }
                        Ok(())
                    },
                )
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // List VMs handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>("feature:VM_Manage:list_vms", move |event| {
                    let p = plugin.clone();
                    let c = Arc::clone(&ctx);
                    match tokio::runtime::Runtime::new() {
                        Ok(rt) => {
                            rt.block_on(async move {
                                p.list_vms(event, c).await;
                            });
                        },
                        Err(e) => {
                            c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                        }
                    }
                    Ok(())
                })
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        // Create Snapshot handler
        {
            let plugin = self.clone();
            let ctx = Arc::clone(&context);
            events
                .on_event::<FeatureActionEvent, _>(
                    "feature:VM_Manage:create_snapshot",
                    move |event| {
                        let p = plugin.clone();
                        let c = Arc::clone(&ctx);
                        match tokio::runtime::Runtime::new() {
                            Ok(rt) => {
                                rt.block_on(async move {
                                    p.create_snapshot(event, c).await;
                                });
                            },
                            Err(e) => {
                                c.log(LogLevel::Error, &format!("VirtualBoxPlugin: Failed to create Tokio runtime: {}", e));
                            }
                        }
                        Ok(())
                    },
                )
                .await
                .map_err(|e| PluginError::EventError(e.to_string()))?;
        }

        context.log(
            LogLevel::Info,
            "VirtualBox Plugin: All event handlers registered successfully!",
        );
        Ok(())
    }

    async fn init(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(LogLevel::Info, "VirtualBox Plugin: Initializing...");

        match self.execute_vboxmanage(&["--version"]).await {
            Ok(version) => {
                context.log(
                    LogLevel::Info,
                    &format!("VirtualBox version: {}", version.trim()),
                );
            }
            Err(e) => {
                let error_msg = format!("VBoxManage not found or not working: {}", e);
                context.log(LogLevel::Error, &error_msg);
                return Err(PluginError::InitializationFailed(error_msg));
            }
        }

        context.log(LogLevel::Info, "VirtualBox Plugin: Initialization complete");
        Ok(())
    }

    async fn shutdown(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(LogLevel::Info, "VirtualBox Plugin: Shutting down...");
        context.log(LogLevel::Info, "VirtualBox Plugin: Shutdown complete");
        Ok(())
    }
}

// Plugin factory function for dynamic loading (returns *mut dyn Plugin)
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    let plugin: Box<dyn Plugin> = Box::new(VirtualBoxPlugin::new());
    Box::into_raw(plugin)
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    if !plugin.is_null() {
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}
