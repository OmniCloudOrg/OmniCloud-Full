//! # VM Management Feature Interface
//!
//! Defines the minimum API surface for VM management operations
//! that all CPI providers must implement.

use omni_ffi_macros::export_feature_plugin;

/// VM Management Feature Interface
/// 
/// This defines the operations that any CPI provider claiming to support
/// the "vm-management" feature must implement.
pub struct VmManagementFeature;

impl VmManagementFeature {
    /// List all virtual machines
    /// 
    /// Expected signature: `fn list_vms() -> Result<Value, String>`
    /// 
    /// Returns:
    /// ```json
    /// {
    ///   "action": "list",
    ///   "status": "success", 
    ///   "vms": ["vm1", "vm2"],
    ///   "count": 2
    /// }
    /// ```
    pub fn list_operation_spec() -> serde_json::Value {
        serde_json::json!({
            "name": "list",
            "description": "List all virtual machines",
            "parameters": {},
            "returns": {
                "type": "object",
                "properties": {
                    "action": {"type": "string"},
                    "status": {"type": "string"},
                    "vms": {"type": "array", "items": {"type": "string"}},
                    "count": {"type": "number"}
                }
            }
        })
    }
    
    /// Create a new virtual machine
    /// 
    /// Expected signature: `fn create_vm(name: String, os_type: String) -> Result<Value, String>`
    /// 
    /// Parameters:
    /// - `name`: VM name (required)
    /// - `os_type`: Operating system type (optional, defaults to "Ubuntu_64")
    /// 
    /// Returns:
    /// ```json
    /// {
    ///   "action": "create",
    ///   "status": "success",
    ///   "vm_name": "my-vm",
    ///   "os_type": "Ubuntu_64"
    /// }
    /// ```
    pub fn create_operation_spec() -> serde_json::Value {
        serde_json::json!({
            "name": "create",
            "description": "Create a new virtual machine",
            "parameters": {
                "name": {"type": "string", "required": true},
                "os_type": {"type": "string", "required": false, "default": "Ubuntu_64"}
            },
            "returns": {
                "type": "object",
                "properties": {
                    "action": {"type": "string"},
                    "status": {"type": "string"},
                    "vm_name": {"type": "string"},
                    "os_type": {"type": "string"}
                }
            }
        })
    }
    
    /// Delete a virtual machine
    /// 
    /// Expected signature: `fn delete_vm(name: String) -> Result<Value, String>`
    /// 
    /// Parameters:
    /// - `name`: VM name to delete (required)
    /// 
    /// Returns:
    /// ```json
    /// {
    ///   "action": "delete",
    ///   "status": "success",
    ///   "vm_name": "my-vm"
    /// }
    /// ```
    pub fn delete_operation_spec() -> serde_json::Value {
        serde_json::json!({
            "name": "delete",
            "description": "Delete a virtual machine",
            "parameters": {
                "name": {"type": "string", "required": true}
            },
            "returns": {
                "type": "object",
                "properties": {
                    "action": {"type": "string"},
                    "status": {"type": "string"},
                    "vm_name": {"type": "string"}
                }
            }
        })
    }
}

// Export the feature plugin using the macro
export_feature_plugin! {
    name: "vm-management",
    operations: [
        "list",
        "create",
        "delete"
    ]
}