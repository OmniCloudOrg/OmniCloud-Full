use omni_registration_macros::{register_feature, register_cpi};
use omni_feature_traits::*;
use omni_event_registry::*;
use serde_json::json;
use std::process::Command;

register_feature! {
    pub enum VirtualBoxFeatures {
        VmManagement,
        VmControl,
        VmMonitoring,
    }
}

register_cpi! {
    pub struct VirtualBoxCPI {}
}

pub fn setup_plugin() -> VirtualBoxCPI {
    VirtualBoxCPI::new()
        .with_name("VirtualBox Provider")
        .with_version("1.0.0")
        .add_feature(VirtualBoxFeatures::VmManagement)
        .add_method("VmManagement", "list", |_input| {
            list_vms().map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmManagement", "create", |input| {
            let name = input.get("name").and_then(|v| v.as_str()).unwrap_or("new-vm");
            let os_type = input.get("os_type").and_then(|v| v.as_str()).unwrap_or("Ubuntu_64");
            create_vm(name, os_type).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmManagement", "delete", |input| {
            let name = input.get("name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            delete_vm(name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_feature(VirtualBoxFeatures::VmControl)
        .add_method("VmControl", "start", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            start_vm(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmControl", "stop", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            stop_vm(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmControl", "pause", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            pause_vm(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmControl", "resume", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            resume_vm(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_method("VmControl", "reset", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            reset_vm(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
        .add_feature(VirtualBoxFeatures::VmMonitoring)
        .add_method("VmMonitoring", "info", |input| {
            let vm_name = input.get("vm_name").and_then(|v| v.as_str())
                .ok_or(EventError::InvalidPayload("VM name required".to_string()))?;
            get_vm_info(vm_name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
        })
}

fn run_vboxmanage(args: &[&str]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(windows)]
    let vboxmanage = "VBoxManage.exe";
    #[cfg(not(windows))]
    let vboxmanage = "VBoxManage";
    
    let output = Command::new(vboxmanage)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute VBoxManage: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("VBoxManage failed: {}", stderr).into());
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn list_vms() -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let output = run_vboxmanage(&["list", "vms"])?;
    let vms: Vec<&str> = output.lines().collect();
    
    Ok(json!({
        "action": "list",
        "status": "success",
        "vms": vms,
        "count": vms.len()
    }))
}

fn create_vm(name: &str, os_type: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["createvm", "--name", name, "--ostype", os_type, "--register"])?;
    
    Ok(json!({
        "action": "create",
        "status": "success",
        "vm_name": name,
        "os_type": os_type
    }))
}

fn delete_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["unregistervm", name, "--delete"])?;
    
    Ok(json!({
        "action": "delete",
        "status": "success",
        "vm_name": name
    }))
}

fn start_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["startvm", name, "--type", "headless"])?;
    
    Ok(json!({
        "action": "start",
        "status": "success",
        "vm_name": name
    }))
}

fn stop_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["controlvm", name, "poweroff"])?;
    
    Ok(json!({
        "action": "stop",
        "status": "success",
        "vm_name": name
    }))
}

fn pause_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["controlvm", name, "pause"])?;
    
    Ok(json!({
        "action": "pause",
        "status": "success",
        "vm_name": name
    }))
}

fn resume_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["controlvm", name, "resume"])?;
    
    Ok(json!({
        "action": "resume",
        "status": "success",
        "vm_name": name
    }))
}

fn reset_vm(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    run_vboxmanage(&["controlvm", name, "reset"])?;
    
    Ok(json!({
        "action": "reset",
        "status": "success",
        "vm_name": name
    }))
}

fn get_vm_info(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let output = run_vboxmanage(&["showvminfo", name, "--machinereadable"])?;
    
    let mut info = serde_json::Map::new();
    for line in output.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let clean_value = value.trim_matches('"');
            info.insert(key.to_string(), json!(clean_value));
        }
    }
    
    Ok(json!({
        "action": "info",
        "status": "success",
        "vm_name": name,
        "info": info
    }))
}