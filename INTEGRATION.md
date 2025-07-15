# OmniCloud Integration Guide

## 🚀 Quick Start

This guide shows you how to integrate and deploy the new event-driven plugin system with your VirtualBox provider and other CPIs.

## 📁 Directory Structure

```
OmniCloud-Full/
├── crates/
│   ├── OmniDirector/           # Main director application
│   ├── providers/
│   │   └── vbox/               # VirtualBox provider source
│   │       └── target/release/
│   │           └── omni_vbox_provider.dll  # Built DLL
│   └── shared/
│       ├── event-registry/     # Central event system
│       └── registration-macros/ # Clean API macros
├── plugins/                    # 🎯 PUT YOUR DLLs HERE
│   ├── omni_vbox_provider.dll  # VirtualBox provider
│   ├── your_custom_provider.dll
│   └── another_provider.dll
└── features/                   # Feature definition files (optional)
    ├── vm_manage.json
    └── custom_features.json
```

## 🎯 Where to Put Your DLLs

### Primary Plugin Directory
```bash
# The director looks for plugins in this directory:
./plugins/

# Copy your DLLs here:
cp providers/vbox/target/release/omni_vbox_provider.dll ./plugins/
cp your_custom_provider.dll ./plugins/
```

### Alternative Plugin Directories
The director can also load from:
- `./crates/OmniDirector/plugins/`
- Environment variable: `OMNI_PLUGINS_DIR`
- Command line: `--plugins-dir /path/to/plugins`

### 📋 Required DLL Structure

Your DLL **must export** these C functions:
```c
// Required exports
extern "C" void register_handlers();
extern "C" void* create_plugin();
extern "C" void destroy_plugin(void* plugin);
extern "C" char* get_plugin_name(void* plugin);
```

## 🔧 Building Your Provider DLL

### 1. Create Your Provider

```rust
// src/lib.rs
use omni_registration_macros::{register_feature, register_cpi};
use omni_event_registry::*;
use serde_json::json;

// Define your features
register_feature! {
    pub enum MyProviderFeatures {
        DataProcessing,
        FileManagement,
        NetworkOps,
    }
}

// Create your CPI
register_cpi! {
    pub struct MyProviderCPI {}
}

// Setup your provider with the fluent API
pub fn setup_plugin() -> MyProviderCPI {
    MyProviderCPI::new()
        .with_name("My Custom Provider")
        .with_version("1.0.0")
        .add_feature(MyProviderFeatures::DataProcessing)
        .add_method("DataProcessing", "process", |input| {
            // Your handler logic here
            Ok(json!({"status": "processed", "result": "success"}))
        })
        .add_feature(MyProviderFeatures::FileManagement)
        .add_method("FileManagement", "list", |input| {
            // Your file listing logic
            Ok(json!({"files": ["file1.txt", "file2.txt"]}))
        })
        .add_method("FileManagement", "create", |input| {
            let filename = input.get("filename").unwrap().as_str().unwrap();
            // Your file creation logic
            Ok(json!({"created": filename}))
        })
}
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-provider"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Important: Must be cdylib for DLL

[dependencies]
omni-registration-macros = { path = "../shared/registration-macros" }
omni-event-registry = { path = "../shared/event-registry" }
serde_json = "1.0"
```

### 3. Build Your DLL

```bash
# Build in release mode
cargo build --release

# Your DLL will be in:
# target/release/my_provider.dll (Windows)
# target/release/libmy_provider.so (Linux)
# target/release/libmy_provider.dylib (macOS)
```

## 🚀 Running the System

### 1. Copy DLLs to Plugin Directory

```bash
# Create plugins directory
mkdir -p plugins

# Copy your built DLLs
cp providers/vbox/target/release/omni_vbox_provider.dll plugins/
cp your_provider/target/release/my_provider.dll plugins/
```

### 2. Start the Director

```bash
# Run from the OmniCloud-Full root directory
cd crates/OmniDirector
cargo run

# Or with custom plugin directory
cargo run -- --plugins-dir /custom/path/to/plugins
```

### 3. Test Your Integration

```rust
// integration_test.rs
use omni_director::cpis::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the system
    let server_context: Arc<dyn ServerContext> = Arc::new(
        ServerContextBuilder::new()
            .with_event_system(Arc::new(EventSystem::new()))
            .with_feature_registry(Arc::new(FeatureRegistry::new()))
            .build()?
    );
    
    let plugin_system = Arc::new(PluginSystem::new(server_context));
    plugin_system.initialize().await?;
    
    // Test your provider
    let result = plugin_system.execute_command(
        "My Custom Provider",    // Provider name
        "DataProcessing",        // Feature
        "process",              // Method
        json!({"data": "test"}) // Payload
    ).await?;
    
    println!("Result: {}", result);
    Ok(())
}
```

## 📝 Event-Driven API Examples

### ✅ Clean Method Registration (NEW)

```rust
// NO MORE MATCH STATEMENTS! 🎉
cpi.add_method("VmManagement", "list", |_input| {
    list_vms().map_err(|e| EventError::ExecutionFailed(e.to_string()))
})
.add_method("VmManagement", "create", |input| {
    let name = input.get("name").unwrap().as_str().unwrap();
    create_vm(name).map_err(|e| EventError::ExecutionFailed(e.to_string()))
})
```

### ❌ Old Bloated Approach (AVOID)

```rust
// DON'T DO THIS ANYMORE!
fn handle_request(feature: &str, method: &str, input: Value) -> Result<Value, Error> {
    match feature {
        "VmManagement" => match method {
            "list" => list_vms(),
            "create" => create_vm(input),
            _ => Err("Unknown method"),
        },
        _ => Err("Unknown feature"),
    }
}
```

## 🔍 Debugging Plugin Loading

### Check Plugin Directory

```bash
# List plugins directory
ls -la plugins/

# Should see your DLLs:
# omni_vbox_provider.dll
# my_provider.dll
```

### View Loaded Handlers

```rust
// In your integration test
let handlers = plugin_system.list_event_handlers();
for handler in handlers {
    println!("Registered: {}", handler);
}

// Expected output:
// Registered: VirtualBox Provider::VmManagement::list
// Registered: VirtualBox Provider::VmManagement::create
// Registered: My Custom Provider::DataProcessing::process
```

### Common Issues

| Issue | Solution |
|-------|----------|
| DLL not found | Check `plugins/` directory path |
| Handler not registered | Ensure `register_handlers()` is called |
| Function not found | Verify C exports in your DLL |
| Plugin name mismatch | Check `.with_name()` matches dispatch calls |

## 🎯 Command Execution

### Direct Command Execution

```rust
// Execute commands directly through the event system
let result = plugin_system.execute_command(
    "VirtualBox Provider",  // Provider name (from .with_name())
    "VmManagement",         // Feature name
    "list",                 // Method name
    json!({})              // Payload
).await?;
```

### Available VirtualBox Commands

| Feature | Method | Payload | Description |
|---------|--------|---------|-------------|
| VmManagement | list | `{}` | List all VMs |
| VmManagement | create | `{"name": "vm-name", "os_type": "Ubuntu_64"}` | Create new VM |
| VmManagement | delete | `{"name": "vm-name"}` | Delete VM |
| VmControl | start | `{"vm_name": "vm-name"}` | Start VM |
| VmControl | stop | `{"vm_name": "vm-name"}` | Stop VM |
| VmControl | pause | `{"vm_name": "vm-name"}` | Pause VM |
| VmControl | resume | `{"vm_name": "vm-name"}` | Resume VM |
| VmMonitoring | info | `{"vm_name": "vm-name"}` | Get VM info |

## 🔐 Production Deployment

### Security Considerations

```bash
# Set proper permissions on plugin directory
chmod 755 plugins/
chmod 644 plugins/*.dll

# Use a dedicated plugins directory
export OMNI_PLUGINS_DIR=/opt/omnicloud/plugins
```

### Docker Deployment

```dockerfile
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates

# Copy application
COPY target/release/omni-director /usr/local/bin/
COPY plugins/ /opt/omnicloud/plugins/

# Set plugin directory
ENV OMNI_PLUGINS_DIR=/opt/omnicloud/plugins

# Run
CMD ["omni-director"]
```

### Systemd Service

```ini
[Unit]
Description=OmniCloud Director
After=network.target

[Service]
Type=simple
User=omnicloud
WorkingDirectory=/opt/omnicloud
Environment=OMNI_PLUGINS_DIR=/opt/omnicloud/plugins
ExecStart=/usr/local/bin/omni-director
Restart=always

[Install]
WantedBy=multi-user.target
```

## 🎉 Summary

1. **📁 Put DLLs in**: `./plugins/` directory
2. **🔧 Use macros**: `register_feature!` and `register_cpi!`
3. **✨ Clean API**: No match statements, pure event-driven
4. **🚀 Auto-loading**: Director finds and loads DLLs automatically
5. **🎯 Direct execution**: Commands dispatch through central registry

The new system eliminates ALL match statement bloat and provides a clean, event-driven architecture that scales effortlessly! 🚀