//! # FFI Macros for OmniCloud Plugins
//!
//! Provides macros to safely wrap FFI exports for CPI and Feature plugins

/// Registry trait for dynamic event registration
pub trait EventRegistry {
    fn register(&mut self, event_name: &str, handler: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>);
}

/// Provider trait for initialization and registration
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, registry: &mut dyn EventRegistry) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
}

/// Macro to export a CPI provider with all required FFI functions
/// 
/// # Usage
/// ```rust
/// use omni_ffi_macros::export_cpi_provider;
/// 
/// pub struct MyProvider;
/// 
/// export_cpi_provider! {
///     provider: MyProvider,
///     metadata: {
///         "name": "my_provider",
///         "version": "1.0.0",
///         "description": "My awesome provider",
///         "features": ["vm-management", "vm-control"]
///     }
/// }
/// ```
#[macro_export]
macro_rules! export_cpi_provider {
    (
        provider: $provider:ty,
        metadata: $metadata:tt
    ) => {
        #[no_mangle]
        pub extern "C" fn create_provider() -> *mut std::ffi::c_void {
            let provider = Box::new(<$provider>::new());
            Box::into_raw(provider) as *mut std::ffi::c_void
        }

        #[no_mangle]
        pub extern "C" fn get_provider_metadata() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let metadata = serde_json::json!($metadata);
            let metadata_str = metadata.to_string();
            let c_str = CString::new(metadata_str).unwrap();
            c_str.into_raw()
        }

        #[no_mangle]
        pub extern "C" fn initialize_provider(
            provider_ptr: *mut std::ffi::c_void,
            registry_ptr: *mut std::ffi::c_void,
        ) -> bool {
            if provider_ptr.is_null() || registry_ptr.is_null() {
                return false;
            }
            
            // Cast back to the provider type and initialize it
            let provider = unsafe { &mut *(provider_ptr as *mut $provider) };
            
            // Create a simple registry adapter that implements EventRegistry
            struct FFIRegistryAdapter {
                registry_ptr: *mut std::ffi::c_void,
            }
            
            impl EventRegistry for FFIRegistryAdapter {
                fn register(&mut self, event_name: &str, handler: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>) {
                    // For now, just print the registration
                    // In a full implementation, this would call back to the OmniDirector's event registry
                    println!("FFI Provider registered event: {}", event_name);
                }
            }
            
            let mut registry_adapter = FFIRegistryAdapter {
                registry_ptr,
            };
            
            match provider.initialize(&mut registry_adapter) {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Provider initialization failed: {}", e);
                    false
                }
            }
        }

        // For backward compatibility with legacy CPI loader
        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut std::ffi::c_void {
            create_provider()
        }
    };
}

/// Macro to export a feature plugin with metadata
/// 
/// # Usage
/// ```rust
/// use omni_ffi_macros::export_feature_plugin;
/// 
/// export_feature_plugin! {
///     name: "vm-management",
///     operations: [
///         "list",
///         "create", 
///         "delete"
///     ]
/// }
/// ```
#[macro_export]
macro_rules! export_feature_plugin {
    (
        name: $name:expr,
        operations: [$($op:expr),*]
    ) => {
        #[no_mangle]
        pub extern "C" fn get_feature_name() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let c_str = CString::new($name).unwrap();
            c_str.into_raw()
        }

        #[no_mangle]
        pub extern "C" fn get_feature_metadata() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let metadata = serde_json::json!({
                "name": $name,
                "type": "feature",
                "description": "Feature interface definition",
                "operations": [$($op),*]
            });
            
            let metadata_str = metadata.to_string();
            let c_str = CString::new(metadata_str).unwrap();
            c_str.into_raw()
        }
    };
}

/// Safe FFI string conversion helper
/// 
/// # Usage
/// ```rust
/// use omni_ffi_macros::ffi_string;
/// 
/// #[no_mangle]
/// pub extern "C" fn get_name() -> *const std::os::raw::c_char {
///     ffi_string!("my_provider")
/// }
/// ```
#[macro_export]
macro_rules! ffi_string {
    ($s:expr) => {
        {
            use std::ffi::CString;
            CString::new($s).unwrap().into_raw()
        }
    };
}

/// Safe FFI JSON conversion helper
/// 
/// # Usage
/// ```rust
/// use omni_ffi_macros::ffi_json;
/// 
/// #[no_mangle]
/// pub extern "C" fn get_metadata() -> *const std::os::raw::c_char {
///     ffi_json!({
///         "name": "my_provider",
///         "version": "1.0.0"
///     })
/// }
/// ```
#[macro_export]
macro_rules! ffi_json {
    ($json:tt) => {
        {
            use std::ffi::CString;
            let json_value = serde_json::json!($json);
            let json_str = json_value.to_string();
            CString::new(json_str).unwrap().into_raw()
        }
    };
}

/// Provider registration helper macro
/// 
/// # Usage
/// ```rust
/// use omni_ffi_macros::register_operations;
/// 
/// register_operations! {
///     registry: registry,
///     provider: provider,
///     operations: [
///         "vm-management.list" => |_data| provider.list_vms(),
///         "vm-management.create" => |data| provider.create_vm(data),
///         "vm-control.start" => |data| provider.start_vm(data)
///     ]
/// }
/// ```
#[macro_export]
macro_rules! register_operations {
    (
        registry: $registry:expr,
        provider: $provider:expr,
        operations: [
            $($event:expr => |$data:ident| $handler:expr),*
        ]
    ) => {
        $(
            {
                let provider_clone = $provider.clone();
                $registry.register($event, Box::new(move |$data: serde_json::Value| {
                    let provider = provider_clone.clone();
                    $handler.map_err(|e| e.to_string())
                }));
            }
        )*
    };
}