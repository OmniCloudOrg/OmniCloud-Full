//! Utility functions for VirtualBox operations

use std::process::Command;
use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;
use crate::error::{VBoxError, VBoxResult};

pub struct VBoxUtils;

impl VBoxUtils {
    /// Execute VBoxManage command
    pub async fn execute_vboxmanage(vboxmanage_path: &str, args: &[&str]) -> VBoxResult<String> {
        let output = Command::new(vboxmanage_path)
            .args(args)
            .output()
            .map_err(|e| VBoxError::ExecutionFailed(format!("Failed to execute VBoxManage: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(VBoxError::ExecutionFailed(format!("VBoxManage failed: {}", error)))
        }
    }

    /// Parse VM info from machine-readable output
    pub fn parse_vm_info(output: &str, vm_id: &str) -> Value {
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
        Value::Object(serde_json::Map::from_iter(info))
    }

    /// Parse VM list output
    pub fn parse_vm_list(output: &str) -> VBoxResult<Vec<Value>> {
        let mut vms = Vec::new();
        let vm_regex = Regex::new(r#""([^"]+)"\s+\{([^}]+)\}"#)
            .map_err(|e| VBoxError::ParseError(format!("Failed to create regex: {}", e)))?;

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

        Ok(vms)
    }

    /// Validate VM name
    pub fn validate_vm_name(name: &str) -> VBoxResult<()> {
        if name.is_empty() {
            return Err(VBoxError::InvalidArgument("VM name cannot be empty".to_string()));
        }
        
        if name.len() > 255 {
            return Err(VBoxError::InvalidArgument("VM name too long (max 255 characters)".to_string()));
        }

        // Check for invalid characters
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(VBoxError::InvalidArgument(
                "VM name contains invalid characters".to_string()
            ));
        }

        Ok(())
    }

    /// Validate memory size
    pub fn validate_memory_mb(memory_mb: u32) -> VBoxResult<()> {
        if memory_mb < 1 {
            return Err(VBoxError::InvalidArgument("Memory must be at least 1 MB".to_string()));
        }
        
        if memory_mb > 1024 * 1024 {
            return Err(VBoxError::InvalidArgument("Memory too large (max 1TB)".to_string()));
        }

        Ok(())
    }

    /// Validate CPU count
    pub fn validate_cpu_count(cpu_count: u32) -> VBoxResult<()> {
        if cpu_count < 1 {
            return Err(VBoxError::InvalidArgument("CPU count must be at least 1".to_string()));
        }
        
        if cpu_count > 256 {
            return Err(VBoxError::InvalidArgument("CPU count too large (max 256)".to_string()));
        }

        Ok(())
    }

    /// Get network adapter arguments for VBoxManage
    pub fn get_network_args(network_adapter: &str) -> Vec<&'static str> {
        match network_adapter {
            "Bridged" => vec!["--nic1", "bridged"],
            "Host-only" => vec!["--nic1", "hostonly"],
            "Internal" => vec!["--nic1", "intnet"],
            _ => vec!["--nic1", "nat"],
        }
    }
}