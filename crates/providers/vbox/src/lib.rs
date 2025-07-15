use std::process::Command;
use serde_json::{json, Value};
use thiserror::Error;
use omni_ffi_macros::{export_cpi_provider, EventRegistry, Provider};

/// VirtualBox provider errors
#[derive(Error, Debug)]
pub enum VBoxError {
    #[error("VBoxManage execution failed: {0}")]
    VBoxManageFailed(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("VM operation failed: {0}")]
    VmOperationFailed(String),
}

/// VirtualBox provider implementation
pub struct VirtualBoxProvider {
    name: String,
    version: String,
}

impl VirtualBoxProvider {
    pub fn new() -> Self {
        Self {
            name: "omni_vbox_provider".to_string(),
            version: "1.0.0".to_string(),
        }
    }

    fn run_vboxmanage(&self, args: &[&str]) -> Result<String, VBoxError> {
        #[cfg(windows)]
        let vboxmanage = "VBoxManage.exe";
        #[cfg(not(windows))]
        let vboxmanage = "VBoxManage";
        
        let output = Command::new(vboxmanage)
            .args(args)
            .output()
            .map_err(|e| VBoxError::VBoxManageFailed(format!("Failed to execute VBoxManage: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(VBoxError::VBoxManageFailed(format!("VBoxManage failed: {}", stderr)));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn list_vms(&self) -> Result<Value, VBoxError> {
        let output = self.run_vboxmanage(&["list", "vms"])?;
        let vms: Vec<&str> = output.lines().collect();
        
        Ok(json!({
            "action": "list",
            "status": "success",
            "vms": vms,
            "count": vms.len()
        }))
    }

    fn create_vm(&self, name: &str, os_type: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["createvm", "--name", name, "--ostype", os_type, "--register"])?;
        
        Ok(json!({
            "action": "create",
            "status": "success",
            "vm_name": name,
            "os_type": os_type
        }))
    }

    fn delete_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["unregistervm", name, "--delete"])?;
        
        Ok(json!({
            "action": "delete",
            "status": "success",
            "vm_name": name
        }))
    }

    fn start_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["startvm", name, "--type", "headless"])?;
        
        Ok(json!({
            "action": "start",
            "status": "success",
            "vm_name": name
        }))
    }

    fn stop_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["controlvm", name, "poweroff"])?;
        
        Ok(json!({
            "action": "stop",
            "status": "success",
            "vm_name": name
        }))
    }

    fn pause_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["controlvm", name, "pause"])?;
        
        Ok(json!({
            "action": "pause",
            "status": "success",
            "vm_name": name
        }))
    }

    fn resume_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["controlvm", name, "resume"])?;
        
        Ok(json!({
            "action": "resume",
            "status": "success",
            "vm_name": name
        }))
    }

    fn reset_vm(&self, name: &str) -> Result<Value, VBoxError> {
        self.run_vboxmanage(&["controlvm", name, "reset"])?;
        
        Ok(json!({
            "action": "reset",
            "status": "success",
            "vm_name": name
        }))
    }

    fn get_vm_info(&self, name: &str) -> Result<Value, VBoxError> {
        let output = self.run_vboxmanage(&["showvminfo", name, "--machinereadable"])?;
        
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
}


impl Provider for VirtualBoxProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn initialize(&mut self, registry: &mut dyn EventRegistry) -> Result<(), String> {
        // Check if VBoxManage is available
        #[cfg(windows)]
        let vboxmanage = "VBoxManage.exe";
        #[cfg(not(windows))]
        let vboxmanage = "VBoxManage";
        
        let vbox_available = Command::new(vboxmanage)
            .arg("--version")
            .output()
            .is_ok();
        
        if !vbox_available {
            println!("Warning: VBoxManage not available, registering stub operations");
        }

        // Create a shared provider instance
        let provider = std::sync::Arc::new(VirtualBoxProvider::new());

        // Register all VirtualBox operations with the central registry
        
        // VM Management operations
        {
            let provider = provider.clone();
            registry.register("vm-management.list", Box::new(move |_data: Value| {
                if vbox_available {
                    provider.list_vms().map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-management.create", Box::new(move |data: Value| {
                if vbox_available {
                    let name = data.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("new-vm");
                    let os_type = data.get("os_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Ubuntu_64");
                    provider.create_vm(name, os_type).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-management.delete", Box::new(move |data: Value| {
                if vbox_available {
                    let name = data.get("name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for delete operation")?;
                    provider.delete_vm(name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        // VM Control operations
        {
            let provider = provider.clone();
            registry.register("vm-control.start", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for start operation")?;
                    provider.start_vm(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-control.stop", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for stop operation")?;
                    provider.stop_vm(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-control.pause", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for pause operation")?;
                    provider.pause_vm(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-control.resume", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for resume operation")?;
                    provider.resume_vm(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        {
            let provider = provider.clone();
            registry.register("vm-control.reset", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for reset operation")?;
                    provider.reset_vm(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        // VM Monitoring operations
        {
            let provider = provider.clone();
            registry.register("vm-monitoring.info", Box::new(move |data: Value| {
                if vbox_available {
                    let vm_name = data.get("vm_name")
                        .and_then(|v| v.as_str())
                        .ok_or("VM name is required for info operation")?;
                    provider.get_vm_info(vm_name).map_err(|e| e.to_string())
                } else {
                    Err("VBoxManage not available".to_string())
                }
            }));
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        // Nothing specific to clean up for VirtualBox
        Ok(())
    }
}

// Export CPI provider using macro
export_cpi_provider! {
    provider: VirtualBoxProvider,
    metadata: {
        "name": "omni_vbox_provider",
        "version": "1.0.0",
        "description": "VirtualBox provider for VM management",
        "supports_features": [
            "vm-management",
            "vm-control",
            "vm-monitoring"
        ]
    }
}