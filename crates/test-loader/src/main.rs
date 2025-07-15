use libloading::{Library, Symbol};
use omni_event_registry::*;
use serde_json::json;
use std::path::Path;

// Match the wrapper structure from the macro
#[repr(C)]
struct PluginWrapper {
    _data: [u8; 0],
}

#[tokio::main]
async fn main() {
    println!("🚀 Testing VirtualBox Provider DLL Loading");
    println!("==========================================");

    let dll_path = Path::new("../providers/vbox/target/release/omni_vbox_provider.dll");
    
    if !dll_path.exists() {
        println!("❌ DLL not found at: {:?}", dll_path);
        println!("Make sure you've built the VirtualBox provider first!");
        return;
    }

    println!("📦 Loading DLL from: {:?}", dll_path);

    // Load the library
    let lib = match unsafe { Library::new(dll_path) } {
        Ok(lib) => {
            println!("✅ Successfully loaded DLL");
            lib
        },
        Err(e) => {
            println!("❌ Failed to load DLL: {}", e);
            return;
        }
    };

    // Get the plugin factory function
    let create_plugin: Symbol<unsafe extern "C" fn() -> *mut PluginWrapper> = 
        match unsafe { lib.get(b"create_plugin") } {
            Ok(func) => {
                println!("✅ Found create_plugin function");
                func
            },
            Err(e) => {
                println!("❌ Failed to find create_plugin function: {}", e);
                return;
            }
        };

    // Register handlers first
    let register_handlers: Symbol<unsafe extern "C" fn()> = 
        match unsafe { lib.get(b"register_handlers") } {
            Ok(func) => {
                println!("✅ Found register_handlers function");
                func
            },
            Err(e) => {
                println!("❌ Failed to find register_handlers function: {}", e);
                return;
            }
        };
    
    unsafe { register_handlers() };
    println!("✅ Handlers registered");

    // Create the plugin instance
    let plugin_ptr = unsafe { create_plugin() };
    if plugin_ptr.is_null() {
        println!("❌ create_plugin returned null pointer");
        return;
    }
    println!("✅ Plugin instance created successfully");

    // Get the plugin name function
    let get_plugin_name: Symbol<unsafe extern "C" fn(*mut PluginWrapper) -> *const std::os::raw::c_char> = 
        match unsafe { lib.get(b"get_plugin_name") } {
            Ok(func) => {
                println!("✅ Found get_plugin_name function");
                func
            },
            Err(e) => {
                println!("❌ Failed to find get_plugin_name function: {}", e);
                return;
            }
        };

    // Get the plugin name
    let name_ptr = unsafe { get_plugin_name(plugin_ptr) };
    if !name_ptr.is_null() {
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        match name.to_str() {
            Ok(name_str) => println!("✅ Plugin name: {}", name_str),
            Err(e) => println!("⚠️ Failed to read plugin name: {}", e),
        }
    } else {
        println!("⚠️ Plugin name function returned null");
    }

    println!();
    println!("🎯 Testing Event Dispatch");
    println!("=========================");

    // Test the event dispatch system
    test_event_dispatch().await;

    // Clean up
    let destroy_plugin: Symbol<unsafe extern "C" fn(*mut PluginWrapper)> = 
        match unsafe { lib.get(b"destroy_plugin") } {
            Ok(func) => func,
            Err(e) => {
                println!("⚠️ Failed to find destroy_plugin function: {}", e);
                return;
            }
        };

    unsafe { destroy_plugin(plugin_ptr) };
    println!("✅ Plugin destroyed successfully");
}

async fn test_event_dispatch() {
    println!("📋 Registered handlers:");
    for handler in get_global_registry().list_handlers() {
        println!("   • {}", handler);
    }
    println!();

    // Test VM list operation
    println!("🔍 Testing VM list operation...");
    match dispatch_event("VirtualBox Provider", "VmManagement", "list", json!({})).await {
        Ok(result) => println!("✅ List VMs result: {}", result),
        Err(e) => println!("❌ List VMs failed: {}", e),
    }

    // Test VM create operation
    println!("🆕 Testing VM create operation...");
    let create_payload = json!({
        "name": "test-vm-from-dll",
        "os_type": "Ubuntu_64"
    });
    
    match dispatch_event("VirtualBox Provider", "VmManagement", "create", create_payload).await {
        Ok(result) => println!("✅ Create VM result: {}", result),
        Err(e) => println!("❌ Create VM failed: {}", e),
    }

    // Test VM control operation
    println!("▶️ Testing VM start operation...");
    let start_payload = json!({
        "vm_name": "test-vm-from-dll"
    });
    
    match dispatch_event("VirtualBox Provider", "VmControl", "start", start_payload).await {
        Ok(result) => println!("✅ Start VM result: {}", result),
        Err(e) => println!("❌ Start VM failed: {}", e),
    }

    // Test VM monitoring operation
    println!("ℹ️ Testing VM info operation...");
    let info_payload = json!({
        "vm_name": "test-vm-from-dll"
    });
    
    match dispatch_event("VirtualBox Provider", "VmMonitoring", "info", info_payload).await {
        Ok(result) => println!("✅ VM Info result: {}", result),
        Err(e) => println!("❌ VM Info failed: {}", e),
    }

    println!();
    println!("🎉 Event dispatch testing complete!");
    println!("✨ Notice: All operations used the central event registry!");
    println!("🚫 NO match statements were executed in the CPI!");
}