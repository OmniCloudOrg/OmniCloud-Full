use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, LitStr, Meta};

/// Macro to export a CPI plugin with all required FFI functions
/// 
/// Usage:
/// ```
/// #[export_cpi_plugin]
/// pub struct MyProvider {
///     // fields
/// }
/// ```
#[proc_macro_attribute]
pub fn export_cpi_plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    // Parse optional metadata from args
    let metadata = if args.is_empty() {
        quote! {
            serde_json::json!({
                "name": stringify!(#struct_name),
                "version": "1.0.0",
                "description": "CPI Plugin",
                "type": "cpi"
            })
        }
    } else {
        // Parse metadata from args if provided
        let metadata_expr = parse_macro_input!(args as Expr);
        quote! { #metadata_expr }
    };

    let expanded = quote! {
        #input

        // Export functions for CPI plugin
        #[no_mangle]
        pub extern "C" fn create_provider() -> *mut std::ffi::c_void {
            let provider = Box::new(#struct_name::new());
            Box::into_raw(provider) as *mut std::ffi::c_void
        }

        #[no_mangle]
        pub extern "C" fn get_provider_metadata() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let metadata = #metadata;
            let metadata_str = metadata.to_string();
            let c_str = CString::new(metadata_str).unwrap();
            c_str.into_raw()
        }

        // For backward compatibility with legacy CPI loader
        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut std::ffi::c_void {
            create_provider()
        }
    };

    TokenStream::from(expanded)
}

/// Macro to export a feature plugin with metadata
/// 
/// Usage:
/// ```
/// #[export_feature_plugin("vm-management")]
/// pub struct VmManagementFeature;
/// ```
#[proc_macro_attribute]
pub fn export_feature_plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    // Parse feature name from args
    let feature_name = if args.is_empty() {
        quote! { stringify!(#struct_name) }
    } else {
        let feature_name_lit = parse_macro_input!(args as LitStr);
        quote! { #feature_name_lit }
    };

    let expanded = quote! {
        #input

        // Export functions for feature plugin
        #[no_mangle]
        pub extern "C" fn get_feature_name() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let feature_name = #feature_name;
            let c_str = CString::new(feature_name).unwrap();
            c_str.into_raw()
        }

        #[no_mangle]
        pub extern "C" fn get_feature_metadata() -> *const std::os::raw::c_char {
            use std::ffi::CString;
            
            let metadata = serde_json::json!({
                "name": #feature_name,
                "type": "feature",
                "description": "Feature interface definition",
                "operations": [] // TODO: Extract from implementation
            });
            
            let metadata_str = metadata.to_string();
            let c_str = CString::new(metadata_str).unwrap();
            c_str.into_raw()
        }
    };

    TokenStream::from(expanded)
}

/// Macro to create a CPI plugin with metadata
/// 
/// Usage:
/// ```
/// cpi_plugin! {
///     name: "my_provider",
///     version: "1.0.0",
///     description: "My awesome provider",
///     features: ["vm-management", "vm-control"],
///     provider: MyProvider
/// }
/// ```
#[proc_macro]
pub fn cpi_plugin(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    
    // This is a simplified version - in practice you'd parse the input properly
    let expanded = quote! {
        // Generate the FFI exports based on the macro input
        // This would be expanded based on the parsed input
    };

    TokenStream::from(expanded)
}

/// Macro to create a feature plugin with operations
/// 
/// Usage:
/// ```
/// feature_plugin! {
///     name: "vm-management",
///     operations: [
///         list() -> Value,
///         create(name: String, os_type: String) -> Value,
///         delete(name: String) -> Value
///     ]
/// }
/// ```
#[proc_macro]
pub fn feature_plugin(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    
    // This is a simplified version - in practice you'd parse the input properly
    let expanded = quote! {
        // Generate the feature interface based on the macro input
        // This would be expanded based on the parsed input
    };

    TokenStream::from(expanded)
}

/// Helper macro to safely wrap FFI calls
/// 
/// Usage:
/// ```
/// safe_ffi_call! {
///     fn my_function() -> *const c_char {
///         let result = "Hello World";
///         CString::new(result).unwrap().into_raw()
///     }
/// }
/// ```
#[proc_macro]
pub fn safe_ffi_call(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    
    // This would wrap the function in proper error handling
    let expanded = quote! {
        // Generate safe FFI wrapper
    };

    TokenStream::from(expanded)
}