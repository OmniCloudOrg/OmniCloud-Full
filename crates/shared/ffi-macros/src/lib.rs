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

        // Thread-safe storage for the FFI interface
        use std::sync::Mutex;
        static FFI_INTERFACE: Mutex<Option<FFIRegistryInterface>> = Mutex::new(None);
        
        #[repr(C)]
        pub struct FFIRegistryInterface {
            pub register_fn: unsafe extern "C" fn(
                registry: *mut std::ffi::c_void,
                event_name: *const std::os::raw::c_char,
                handler: unsafe extern "C" fn(data: *const std::os::raw::c_char) -> *const std::os::raw::c_char,
            ),
        }
        
        #[no_mangle]
        pub extern "C" fn set_registry_interface(
            _provider_ptr: *mut std::ffi::c_void,
            interface: *const FFIRegistryInterface,
        ) -> bool {
            if interface.is_null() {
                return false;
            }
            
            let interface_copy = unsafe { std::ptr::read(interface) };
            
            if let Ok(mut guard) = FFI_INTERFACE.lock() {
                *guard = Some(interface_copy);
                true
            } else {
                false
            }
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
            
            // Create a registry adapter that uses the FFI interface
            struct FFIRegistryAdapter {
                registry_ptr: *mut std::ffi::c_void,
            }
            
            impl EventRegistry for FFIRegistryAdapter {
                fn register(&mut self, event_name: &str, handler: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync>) {
                    println!("FFI Provider registered event: {}", event_name);
                    
                    if let Ok(guard) = FFI_INTERFACE.lock() {
                        if let Some(interface) = &*guard {
                            let event_name_cstr = std::ffi::CString::new(event_name).unwrap();
                            
                            // Create a C function that calls our Rust handler
                            unsafe extern "C" fn call_rust_handler(data: *const std::os::raw::c_char) -> *const std::os::raw::c_char {
                                // This is a simplified version - in practice we'd need to store and retrieve handlers
                                let result = std::ffi::CString::new(r#"{"status": "success", "message": "Event handled"}"#).unwrap();
                                result.into_raw()
                            }
                            
                            // Call the registration function
                            unsafe {
                                (interface.register_fn)(
                                    self.registry_ptr,
                                    event_name_cstr.as_ptr(),
                                    call_rust_handler
                                );
                            }
                        }
                    }
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