//! # VirtualBox Plugin for OmniDirector - Main Entry Point
//!
//! This plugin provides VM_Manage feature implementation using VirtualBox.
//! NO MATCH STATEMENTS - pure event-driven architecture!

use std::sync::Arc;
use async_trait::async_trait;
use omni_director::cpis::{LogLevel, Plugin, PluginError, ServerContext};

mod config;
mod error;
mod events;
mod handlers;
mod operations;
mod utils;

use config::VirtualBoxConfig;
use handlers::EventHandlers;
use operations::VBoxOperations;

#[derive(Clone)]
pub struct VirtualBoxPlugin {
    config: VirtualBoxConfig,
    operations: VBoxOperations,
    handlers: EventHandlers,
}

impl VirtualBoxPlugin {
    pub fn new() -> Self {
        let config = VirtualBoxConfig::new();
        let operations = VBoxOperations::new(config.clone());
        let handlers = EventHandlers::new(operations.clone());
        
        Self {
            config,
            operations,
            handlers,
        }
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

        self.handlers.register_all_handlers(context.clone()).await
            .map_err(|e| PluginError::EventError(e.to_string()))?;

        context.log(
            LogLevel::Info,
            "VirtualBox Plugin: All event handlers registered successfully!",
        );
        Ok(())
    }

    async fn init(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(LogLevel::Info, "VirtualBox Plugin: Initializing...");

        self.operations.verify_installation(context.clone()).await
            .map_err(|e| PluginError::InitializationFailed(e.to_string()))?;

        context.log(LogLevel::Info, "VirtualBox Plugin: Initialization complete");
        Ok(())
    }

    async fn shutdown(&mut self, context: Arc<dyn ServerContext>) -> Result<(), PluginError> {
        context.log(LogLevel::Info, "VirtualBox Plugin: Shutting down...");
        context.log(LogLevel::Info, "VirtualBox Plugin: Shutdown complete");
        Ok(())
    }
}

// Plugin factory functions for dynamic loading
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