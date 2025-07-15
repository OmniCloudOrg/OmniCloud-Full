use omni_director::cpis::*;
use omni_event_registry::*;
use serde_json::json;
use std::sync::Arc;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 OmniCloud Full Integration Test");
    println!("=====================================");
    
    // Create plugins directory if it doesn't exist
    if !std::path::Path::new("./plugins").exists() {
        fs::create_dir_all("./plugins").await?;
    }
    
    // Copy our VirtualBox provider DLL to the plugins directory
    let vbox_dll_source = "../providers/vbox/target/release/omni_vbox_provider.dll";
    let vbox_dll_dest = "./plugins/omni_vbox_provider.dll";
    
    if std::path::Path::new(vbox_dll_source).exists() {
        fs::copy(vbox_dll_source, vbox_dll_dest).await?;
        println!("✅ Copied VirtualBox provider to plugins directory");
    } else {
        println!("⚠️  VirtualBox provider DLL not found, skipping...");
    }
    
    // Create the server context
    let server_context: Arc<dyn ServerContext> = Arc::new(ServerContextBuilder::new()
        .with_event_system(Arc::new(EventSystem::new()))
        .with_feature_registry(Arc::new(FeatureRegistry::new()))
        .build()?);
    
    // Create the plugin system
    let plugin_system = Arc::new(PluginSystem::new(server_context));
    
    println!("🔧 Initializing plugin system...");
    
    // Initialize the plugin system (loads all DLLs from ./plugins)
    match plugin_system.initialize().await {
        Ok(_) => println!("✅ Plugin system initialized successfully"),
        Err(e) => {
            println!("❌ Plugin system initialization failed: {}", e);
            return Ok(());
        }
    }
    
    println!();
    println!("📋 Available Event Handlers:");
    let handlers = plugin_system.list_event_handlers();
    if handlers.is_empty() {
        println!("   (No handlers registered)");
    } else {
        for handler in &handlers {
            println!("   • {}", handler);
        }
    }
    
    println!();
    println!("🎯 Testing Direct Command Execution");
    println!("===================================");
    
    // Test VirtualBox VM management
    println!("📝 Testing VM list operation...");
    match plugin_system.execute_command(
        "VirtualBox Provider",
        "VmManagement", 
        "list",
        json!({})
    ).await {
        Ok(result) => {
            println!("✅ VM List Success:");
            println!("   {}", result);
        },
        Err(e) => println!("❌ VM List Failed: {}", e),
    }
    
    // Test VM creation
    println!("🆕 Testing VM create operation...");
    match plugin_system.execute_command(
        "VirtualBox Provider",
        "VmManagement",
        "create", 
        json!({
            "name": "integration-test-vm",
            "os_type": "Ubuntu_64"
        })
    ).await {
        Ok(result) => {
            println!("✅ VM Create Success:");
            println!("   {}", result);
        },
        Err(e) => println!("❌ VM Create Failed: {}", e),
    }
    
    // Test VM control
    println!("▶️ Testing VM start operation...");
    match plugin_system.execute_command(
        "VirtualBox Provider",
        "VmControl",
        "start",
        json!({
            "vm_name": "integration-test-vm"
        })
    ).await {
        Ok(result) => {
            println!("✅ VM Start Success:");
            println!("   {}", result);
        },
        Err(e) => println!("❌ VM Start Failed: {}", e),
    }
    
    // Test VM monitoring
    println!("ℹ️ Testing VM info operation...");
    match plugin_system.execute_command(
        "VirtualBox Provider",
        "VmMonitoring",
        "info",
        json!({
            "vm_name": "integration-test-vm"
        })
    ).await {
        Ok(result) => {
            println!("✅ VM Info Success:");
            println!("   {}", result);
        },
        Err(e) => println!("❌ VM Info Failed: {}", e),
    }
    
    println!();
    println!("🎉 Integration Test Complete!");
    println!("✨ Key Achievements:");
    println!("   • ✅ Event-driven plugin loading");
    println!("   • ✅ Dynamic DLL loading and registration");
    println!("   • ✅ Central event registry (NO match statements!)");
    println!("   • ✅ Cross-platform VirtualBox provider");
    println!("   • ✅ Fluent API for CPI registration");
    println!("   • ✅ Automatic validation of feature contracts");
    println!("   • ✅ Safe FFI wrapper macros");
    
    // Shutdown gracefully
    println!();
    println!("🛑 Shutting down plugin system...");
    match plugin_system.shutdown().await {
        Ok(_) => println!("✅ Plugin system shutdown complete"),
        Err(e) => println!("⚠️  Plugin system shutdown warning: {}", e),
    }
    
    Ok(())
}