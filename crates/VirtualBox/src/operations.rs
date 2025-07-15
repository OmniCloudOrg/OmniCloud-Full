//! VM operations implementation for VirtualBox

use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use chrono::Utc;
use omni_director::cpis::{LogLevel, ServerContext};
use crate::config::VirtualBoxConfig;
use crate::error::{VBoxError, VBoxResult};
use crate::utils::VBoxUtils;

#[derive(Clone)]
pub struct VBoxOperations {
    config: VirtualBoxConfig,
}

impl VBoxOperations {
    pub fn new(config: VirtualBoxConfig) -> Self {
        Self { config }
    }

    /// Verify VirtualBox installation
    pub async fn verify_installation(&self, context: Arc<dyn ServerContext>) -> VBoxResult<()> {
        match VBoxUtils::execute_vboxmanage(self.config.get_vboxmanage_path(), &["--version"]).await {
            Ok(version) => {
                context.log(
                    LogLevel::Info,
                    &format!("VirtualBox version: {}", version.trim()),
                );
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("VBoxManage not found or not working: {}", e);
                context.log(LogLevel::Error, &error_msg);
                Err(VBoxError::NotInstalled(error_msg))
            }
        }
    }

    /// Create VM implementation
    pub async fn create_vm(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("name".to_string()))?;
        
        let memory_mb = args
            .get("memory_mb")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| VBoxError::MissingArgument("memory_mb".to_string()))? as u32;
        
        let cpu_count = args
            .get("cpu_count")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| VBoxError::MissingArgument("cpu_count".to_string()))? as u32;
        
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

        // Validate inputs
        VBoxUtils::validate_vm_name(name)?;
        VBoxUtils::validate_memory_mb(memory_mb)?;
        VBoxUtils::validate_cpu_count(cpu_count)?;

        // Create the VM
        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["createvm", "--name", name, "--ostype", os_type, "--register"]
        ).await?;

        // Configure memory and CPUs
        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &[
                "modifyvm", name,
                "--memory", &memory_mb.to_string(),
                "--cpus", &cpu_count.to_string(),
            ]
        ).await?;

        // Configure network
        let nic_args = VBoxUtils::get_network_args(network_adapter);
        let mut modify_args = vec!["modifyvm", name];
        modify_args.extend(&nic_args);
        VBoxUtils::execute_vboxmanage(self.config.get_vboxmanage_path(), &modify_args).await?;

        // Create storage controller
        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &[
                "storagectl", name,
                "--name", "SATA Controller",
                "--add", "sata",
                "--controller", "IntelAhci",
            ]
        ).await?;

        // Create and attach disk
        let disk_path = format!("{}.vdi", name);
        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &[
                "createhd",
                "--filename", &disk_path,
                "--size", &(disk_size_gb * 1024).to_string(),
                "--format", "VDI",
            ]
        ).await?;

        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &[
                "storageattach", name,
                "--storagectl", "SATA Controller",
                "--port", "0",
                "--device", "0",
                "--type", "hdd",
                "--medium", &disk_path,
            ]
        ).await?;

        Ok(serde_json::json!({
            "vm_id": name,
            "name": name,
            "status": "created",
            "memory_mb": memory_mb,
            "cpu_count": cpu_count,
            "disk_size_gb": disk_size_gb
        }))
    }

    /// Delete VM implementation
    pub async fn delete_vm(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("vm_id".to_string()))?;
        
        let delete_files = args
            .get("delete_files")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // Try to power off the VM first (ignore errors)
        let _ = VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["controlvm", vm_id, "poweroff"]
        ).await;

        // Wait a bit for the VM to power off
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Unregister and optionally delete files
        if delete_files {
            VBoxUtils::execute_vboxmanage(
                self.config.get_vboxmanage_path(),
                &["unregistervm", vm_id, "--delete"]
            ).await?;
        } else {
            VBoxUtils::execute_vboxmanage(
                self.config.get_vboxmanage_path(),
                &["unregistervm", vm_id]
            ).await?;
        }

        Ok(Value::Bool(true))
    }

    /// Start VM implementation
    pub async fn start_vm(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("vm_id".to_string()))?;
        
        let headless = args
            .get("headless")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let start_type = if headless { "headless" } else { "gui" };

        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["startvm", vm_id, "--type", start_type]
        ).await?;

        Ok(serde_json::json!({
            "vm_id": vm_id,
            "status": "running",
            "started_at": Utc::now().to_rfc3339()
        }))
    }

    /// Stop VM implementation
    pub async fn stop_vm(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("vm_id".to_string()))?;
        
        let force = args
            .get("force")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let stop_command = if force { "poweroff" } else { "acpipowerbutton" };

        VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["controlvm", vm_id, stop_command]
        ).await?;

        Ok(serde_json::json!({
            "vm_id": vm_id,
            "status": "stopped",
            "stopped_at": Utc::now().to_rfc3339()
        }))
    }

    /// Get VM info implementation
    pub async fn get_vm_info(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("vm_id".to_string()))?;

        let output = VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["showvminfo", vm_id, "--machinereadable"]
        ).await?;

        Ok(VBoxUtils::parse_vm_info(&output, vm_id))
    }

    /// List VMs implementation
    pub async fn list_vms(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let include_running_only = args
            .get("include_running_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let list_arg = if include_running_only { "runningvms" } else { "vms" };
        
        let output = VBoxUtils::execute_vboxmanage(
            self.config.get_vboxmanage_path(),
            &["list", list_arg]
        ).await?;

        let vms = VBoxUtils::parse_vm_list(&output)?;
        Ok(Value::Array(vms))
    }

    /// Create snapshot implementation
    pub async fn create_snapshot(&self, args: &HashMap<String, Value>) -> VBoxResult<Value> {
        let vm_id = args
            .get("vm_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("vm_id".to_string()))?;
        
        let snapshot_name = args
            .get("snapshot_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VBoxError::MissingArgument("snapshot_name".to_string()))?;
        
        let description = args
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut snapshot_args = vec!["snapshot", vm_id, "take", snapshot_name];
        if !description.is_empty() {
            snapshot_args.extend(&["--description", description]);
        }

        VBoxUtils::execute_vboxmanage(self.config.get_vboxmanage_path(), &snapshot_args).await?;

        Ok(serde_json::json!({
            "snapshot_id": format!("{}_{}", vm_id, snapshot_name),
            "snapshot_name": snapshot_name,
            "vm_id": vm_id,
            "created_at": Utc::now().to_rfc3339()
        }))
    }
}