//! # Worker Management Feature
//!
//! Clean enum-based worker management operations with type-safe parameters.

use omni_feature_traits::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Define the worker management operations using the macro
define_feature_operations! {
    /// Worker Management Feature Operations
    enum WorkerManagement {
        /// Start a new worker
        StartWorker {
            args: [
                worker_name: String,
                instance_type: String,
                availability_zone: String,
                image: String,
                security_group: String,
                network: String,
            ]
        },
        
        /// Stop a worker
        StopWorker {
            args: [
                worker_id: String,
            ]
        },
        
        /// Delete a worker
        DeleteWorker {
            args: [
                worker_id: String,
            ]
        },
        
        /// List all workers
        ListWorkers {
            args: []
        },
        
        /// Get worker status
        GetWorkerStatus {
            args: [
                worker_id: String,
            ]
        },
        
        /// Scale workers
        ScaleWorkers {
            args: [
                target_count: i32,
                instance_type: String,
            ]
        },
    }
}

/// Worker state information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerState {
    Pending,
    Creating,
    Running,
    Stopping,
    Stopped,
    Terminated,
    Failed,
}

/// Worker metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetadata {
    pub id: String,
    pub name: String,
    pub state: WorkerState,
    pub instance_type: String,
    pub availability_zone: Option<String>,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: HashMap<String, String>,
    pub provider_metadata: HashMap<String, serde_json::Value>,
}

/// Feature plugin entry point
#[no_mangle]
pub extern "C" fn get_feature_name() -> *const std::os::raw::c_char {
    std::ffi::CString::new("worker_management").unwrap().into_raw()
}

/// Feature plugin registration
#[no_mangle]
pub extern "C" fn register_operations() -> *const std::os::raw::c_char {
    let operations = vec![
        "StartWorker",
        "StopWorker", 
        "DeleteWorker",
        "ListWorkers",
        "GetWorkerStatus",
        "ScaleWorkers",
    ];
    
    let json = serde_json::to_string(&operations).unwrap();
    std::ffi::CString::new(json).unwrap().into_raw()
}

/// Execute an operation using the type-safe enum system
#[no_mangle]
pub extern "C" fn execute_operation(
    operation_name: *const std::os::raw::c_char,
    args_json: *const std::os::raw::c_char,
) -> *const std::os::raw::c_char {
    let operation_name = unsafe { std::ffi::CStr::from_ptr(operation_name) }.to_str().unwrap();
    let args_json = unsafe { std::ffi::CStr::from_ptr(args_json) }.to_str().unwrap();
    
    // Parse arguments
    let args: HashMap<String, serde_json::Value> = serde_json::from_str(args_json)
        .unwrap_or_else(|_| HashMap::new());
    
    // Create the appropriate operation enum and execute
    let result = match operation_name {
        "StartWorker" => {
            let operation = WorkerManagement::StartWorker(
                args.get("worker_name").and_then(|v| v.as_str()).unwrap_or("default-worker").to_string(),
                args.get("instance_type").and_then(|v| v.as_str()).unwrap_or("t3.medium").to_string(),
                args.get("availability_zone").and_then(|v| v.as_str()).unwrap_or("us-east-1a").to_string(),
                args.get("image").and_then(|v| v.as_str()).unwrap_or("ami-default").to_string(),
                args.get("security_group").and_then(|v| v.as_str()).unwrap_or("default").to_string(),
                args.get("network").and_then(|v| v.as_str()).unwrap_or("default").to_string(),
            );
            
            let mut execution_values = HashMap::new();
            execution_values.insert("worker_name".to_string(), serde_json::Value::String(args.get("worker_name").and_then(|v| v.as_str()).unwrap_or("default-worker").to_string()));
            execution_values.insert("instance_type".to_string(), serde_json::Value::String(args.get("instance_type").and_then(|v| v.as_str()).unwrap_or("t3.medium").to_string()));
            execution_values.insert("availability_zone".to_string(), serde_json::Value::String(args.get("availability_zone").and_then(|v| v.as_str()).unwrap_or("us-east-1a").to_string()));
            execution_values.insert("image".to_string(), serde_json::Value::String(args.get("image").and_then(|v| v.as_str()).unwrap_or("ami-default").to_string()));
            execution_values.insert("security_group".to_string(), serde_json::Value::String(args.get("security_group").and_then(|v| v.as_str()).unwrap_or("default").to_string()));
            execution_values.insert("network".to_string(), serde_json::Value::String(args.get("network").and_then(|v| v.as_str()).unwrap_or("default").to_string()));
            
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    // Add worker-specific fields
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("worker_id".to_string(), serde_json::Value::String(Uuid::new_v4().to_string()));
                        obj.insert("state".to_string(), serde_json::Value::String("Creating".to_string()));
                        obj.insert("created_at".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        "StopWorker" => {
            let operation = WorkerManagement::StopWorker(
                args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            );
            
            let mut execution_values = HashMap::new();
            execution_values.insert("worker_id".to_string(), serde_json::Value::String(args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()));
            
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("state".to_string(), serde_json::Value::String("Stopping".to_string()));
                        obj.insert("message".to_string(), serde_json::Value::String("Worker stop initiated".to_string()));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        "DeleteWorker" => {
            let operation = WorkerManagement::DeleteWorker(
                args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            );
            
            let mut execution_values = HashMap::new();
            execution_values.insert("worker_id".to_string(), serde_json::Value::String(args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()));
            
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("state".to_string(), serde_json::Value::String("Terminated".to_string()));
                        obj.insert("message".to_string(), serde_json::Value::String("Worker deletion initiated".to_string()));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        "ListWorkers" => {
            let operation = WorkerManagement::ListWorkers();
            
            let execution_values = HashMap::new();
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("workers".to_string(), serde_json::Value::Array(vec![]));
                        obj.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(0)));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        "GetWorkerStatus" => {
            let operation = WorkerManagement::GetWorkerStatus(
                args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            );
            
            let mut execution_values = HashMap::new();
            execution_values.insert("worker_id".to_string(), serde_json::Value::String(args.get("worker_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()));
            
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("state".to_string(), serde_json::Value::String("Running".to_string()));
                        obj.insert("health".to_string(), serde_json::Value::String("Healthy".to_string()));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        "ScaleWorkers" => {
            let operation = WorkerManagement::ScaleWorkers(
                args.get("target_count").and_then(|v| v.as_i64()).unwrap_or(1).to_string(),
                args.get("instance_type").and_then(|v| v.as_str()).unwrap_or("t3.medium").to_string(),
            );
            
            let mut execution_values = HashMap::new();
            execution_values.insert("target_count".to_string(), serde_json::Value::Number(serde_json::Number::from(args.get("target_count").and_then(|v| v.as_i64()).unwrap_or(1))));
            execution_values.insert("instance_type".to_string(), serde_json::Value::String(args.get("instance_type").and_then(|v| v.as_str()).unwrap_or("t3.medium").to_string()));
            
            match operation.execute(execution_values) {
                Ok(mut result) => {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert("target_count".to_string(), serde_json::Value::Number(serde_json::Number::from(args.get("target_count").and_then(|v| v.as_i64()).unwrap_or(1))));
                        obj.insert("current_count".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                        obj.insert("message".to_string(), serde_json::Value::String("Scaling initiated".to_string()));
                    }
                    result
                },
                Err(e) => serde_json::json!({ "error": e.to_string() })
            }
        },
        _ => {
            serde_json::json!({
                "error": format!("Unknown operation: {}", operation_name)
            })
        }
    };
    
    let json = serde_json::to_string(&result).unwrap();
    std::ffi::CString::new(json).unwrap().into_raw()
}