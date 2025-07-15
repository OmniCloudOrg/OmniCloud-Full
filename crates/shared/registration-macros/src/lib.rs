use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, ItemStruct};

#[proc_macro]
pub fn register_feature(input: TokenStream) -> TokenStream {
    let feature_enum = parse_macro_input!(input as ItemEnum);
    let enum_name = &feature_enum.ident;
    
    let variants: Vec<_> = feature_enum.variants.iter().map(|v| &v.ident).collect();
    let variant_strings: Vec<_> = feature_enum.variants.iter().map(|v| v.ident.to_string()).collect();
    
    let expanded = quote! {
        pub trait FeatureDefinition {
            fn name(&self) -> &'static str;
            fn required_methods(&self) -> Vec<&'static str>;
        }
        
        #feature_enum
        
        impl FeatureDefinition for #enum_name {
            fn name(&self) -> &'static str {
                match self {
                    #(Self::#variants => #variant_strings,)*
                }
            }
            
            fn required_methods(&self) -> Vec<&'static str> {
                match self {
                    #(Self::#variants => vec![stringify!(#variants)],)*
                }
            }
        }
        
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
        
        unsafe impl Send for #enum_name {}
        unsafe impl Sync for #enum_name {}
    };
    
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn register_cpi(input: TokenStream) -> TokenStream {
    let cpi_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &cpi_struct.ident;
    
    let expanded = quote! {
        use std::sync::Arc;
        use omni_feature_traits::*;
        use omni_event_registry::*;
        
        pub struct #struct_name {
            pub name: String,
            pub version: String,
            pub declared_features: Vec<String>,
        }
        
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    version: String::new(),
                    declared_features: Vec::new(),
                }
            }
            
            pub fn with_name(mut self, name: impl Into<String>) -> Self {
                self.name = name.into();
                self
            }
            
            pub fn with_version(mut self, version: impl Into<String>) -> Self {
                self.version = version.into();
                self
            }
            
            pub fn add_feature<F: FeatureDefinition + 'static>(
                mut self,
                feature: F
            ) -> Self {
                let feature_name = feature.name().to_string();
                self.declared_features.push(feature_name);
                self
            }
            
            pub fn add_method<F>(
                self,
                feature: &str,
                method: &str,
                handler: F
            ) -> Self 
            where
                F: Fn(serde_json::Value) -> Result<serde_json::Value, EventError> + Send + Sync + 'static,
            {
                // Register the handler directly with the global event registry
                register_event_handler(&self.name, feature, method, handler);
                self
            }
            
            pub fn validate(&self) -> Result<(), String> {
                // Validation is now handled by the event registry
                get_global_registry().validate_provider(&self.name, &self.declared_features)
                    .map_err(|e| e.to_string())
            }
            
            pub fn build(self) -> Result<Box<dyn Plugin>, String> {
                self.validate()?;
                Ok(Box::new(CPIPlugin {
                    inner: self,
                }))
            }
        }
        
        pub struct CPIPlugin {
            inner: #struct_name,
        }
        
        #[async_trait]
        impl Plugin for CPIPlugin {
            fn name(&self) -> &str {
                &self.inner.name
            }
            
            fn version(&self) -> &str {
                &self.inner.version
            }
            
            fn declared_features(&self) -> Vec<String> {
                self.inner.declared_features.clone()
            }
            
            async fn pre_init(
                &mut self,
                _context: Arc<dyn ServerContext>
            ) -> Result<(), PluginError> {
                // No match statements needed - all events are centrally registered!
                Ok(())
            }
            
            async fn init(
                &mut self,
                _context: Arc<dyn ServerContext>
            ) -> Result<(), PluginError> {
                // No match statements needed - all events are centrally registered!
                Ok(())
            }
            
            async fn shutdown(
                &mut self,
                _context: Arc<dyn ServerContext>
            ) -> Result<(), PluginError> {
                // No match statements needed - all events are centrally registered!
                Ok(())
            }
        }
        
        static mut PLUGIN_BUILDER: Option<#struct_name> = None;
        static PLUGIN_INIT: std::sync::Once = std::sync::Once::new();
        
        pub fn initialize_plugin() -> #struct_name {
            crate::setup_plugin()
        }
        
        struct PluginWrapper {
            plugin: Box<dyn Plugin>,
        }
        
        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut PluginWrapper {
            // Initialize the plugin and register all handlers
            let builder = initialize_plugin();
            match builder.build() {
                Ok(plugin) => {
                    let wrapper = PluginWrapper { plugin };
                    Box::into_raw(Box::new(wrapper))
                },
                Err(_) => core::ptr::null_mut(),
            }
        }
        
        #[no_mangle]
        pub extern "C" fn register_handlers() {
            // Call setup_plugin to trigger handler registration
            let _ = initialize_plugin();
        }
        
        #[no_mangle]
        pub extern "C" fn destroy_plugin(plugin_ptr: *mut PluginWrapper) {
            if !plugin_ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(plugin_ptr);
                }
            }
        }
        
        #[no_mangle]
        pub extern "C" fn get_plugin_name(plugin_ptr: *mut PluginWrapper) -> *const std::os::raw::c_char {
            if plugin_ptr.is_null() {
                return core::ptr::null();
            }
            
            static mut PLUGIN_NAME: [u8; 256] = [0; 256];
            
            unsafe {
                let wrapper = &*plugin_ptr;
                let name = wrapper.plugin.name();
                let name_bytes = name.as_bytes();
                
                if name_bytes.len() >= 255 {
                    return core::ptr::null();
                }
                
                PLUGIN_NAME[..name_bytes.len()].copy_from_slice(name_bytes);
                PLUGIN_NAME[name_bytes.len()] = 0;
                
                PLUGIN_NAME.as_ptr() as *const std::os::raw::c_char
            }
        }
        
        pub type CPIBuilder = #struct_name;
    };
    
    TokenStream::from(expanded)
}