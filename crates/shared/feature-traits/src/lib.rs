//! # OmniCloud Feature Traits
//!
//! Shared traits and types for the enum-based feature system.
//! Each feature defines operations as enums with type-safe parameters.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

/// Value pool identifier - references values from various sources with type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueRef<T> {
    /// The key to look up in the value pool
    pub key: String,
    /// Phantom data to maintain type information
    _phantom: PhantomData<T>,
}

impl<T> ValueRef<T> {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            _phantom: PhantomData,
        }
    }
    
    /// Get the type name for debugging/serialization
    pub fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

/// Implement Serialize and Deserialize for ValueRef
impl<T> Serialize for ValueRef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ValueRef", 2)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("type_name", &self.type_name())?;
        state.end()
    }
}

impl<'de, T> Deserialize<'de> for ValueRef<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ValueRefData {
            key: String,
            type_name: String,
        }
        
        let data = ValueRefData::deserialize(deserializer)?;
        
        // Verify type matches (optional runtime check)
        if data.type_name != std::any::type_name::<T>() {
            return Err(serde::de::Error::custom(format!(
                "Type mismatch: expected {}, got {}",
                std::any::type_name::<T>(),
                data.type_name
            )));
        }
        
        Ok(ValueRef::new(data.key))
    }
}

/// Type-safe argument definition for feature operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedArg<T> {
    pub name: String,
    pub value_ref: ValueRef<T>,
}

impl<T> TypedArg<T> {
    pub fn new(name: impl Into<String>, value_ref: ValueRef<T>) -> Self {
        Self {
            name: name.into(),
            value_ref,
        }
    }
}

/// Helper macros for creating type-safe value references
#[macro_export]
macro_rules! value_ref {
    ($key:expr, $type:ty) => {
        $crate::ValueRef::<$type>::new($key)
    };
}

#[macro_export]
macro_rules! typed_arg {
    ($name:expr, $key:expr, $type:ty) => {
        $crate::TypedArg::new($name, $crate::value_ref!($key, $type))
    };
}

/// Base trait that all feature operation enums must implement
pub trait FeatureOperation: Send + Sync + Clone + std::fmt::Debug {
    /// Get the operation name
    fn operation_name(&self) -> &'static str;
    
    /// Get the type-safe arguments required for this operation
    fn get_typed_args(&self) -> Vec<(&'static str, &'static str)>; // (name, type_name)
    
    /// Execute the operation with resolved values
    fn execute(&self, values: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, FeatureError>;
}

/// Feature execution context for value resolution
#[async_trait::async_trait]
pub trait FeatureContext: Send + Sync {
    /// Resolve a value from the value pools
    async fn resolve_value(&self, key: &str) -> Result<serde_json::Value, FeatureError>;
    
    /// Resolve a value with type safety
    async fn resolve_typed<T>(&self, value_ref: &ValueRef<T>) -> Result<T, FeatureError>
    where
        T: for<'de> serde::Deserialize<'de> + Send;
    
    /// Get execution ID
    fn execution_id(&self) -> Uuid;
    
    /// Log a message
    async fn log(&self, level: LogLevel, message: &str);
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Feature execution errors
#[derive(thiserror::Error, Debug, Clone)]
pub enum FeatureError {
    #[error("Value not found: {0}")]
    ValueNotFound(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    #[error("Provider error: {0}")]
    ProviderError(String),
}

/// Macro to generate feature operation enums with type safety
#[macro_export]
macro_rules! define_feature_operations {
    (
        $(#[$enum_meta:meta])*
        enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident {
                    args: [$($arg_name:ident: $arg_type:ty),* $(,)?]
                }
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant {
                    $(
                        $arg_name: $crate::ValueRef<$arg_type>,
                    )*
                }
            ),*
        }
        
        impl $crate::FeatureOperation for $enum_name {
            fn operation_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant { .. } => stringify!($variant),
                    )*
                }
            }
            
            fn get_typed_args(&self) -> Vec<(&'static str, &'static str)> {
                match self {
                    $(
                        Self::$variant { .. } => {
                            vec![
                                $(
                                    (stringify!($arg_name), std::any::type_name::<$arg_type>()),
                                )*
                            ]
                        }
                    )*
                }
            }
            
            fn execute(&self, values: std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, $crate::FeatureError> {
                match self {
                    $(
                        Self::$variant { $($arg_name,)* } => {
                            // Type-safe value extraction
                            $(
                                let $arg_name = values.get(&$arg_name.key)
                                    .ok_or_else(|| $crate::FeatureError::ValueNotFound($arg_name.key.clone()))?;
                                
                                // Type validation would go here
                                // For now, we'll trust the JSON value matches the expected type
                            )*
                            
                            // Execute the operation with type-safe values
                            Ok(serde_json::json!({
                                "operation": stringify!($variant),
                                "status": "executed",
                                "args": {
                                    $(
                                        stringify!($arg_name): $arg_name,
                                    )*
                                }
                            }))
                        }
                    )*
                }
            }
        }
        
        // Helper functions for creating typed instances
        impl $enum_name {
            $(
                $(#[$variant_meta])*
                pub fn $variant(
                    $($arg_name: impl Into<String>,)*
                ) -> Self {
                    Self::$variant {
                        $(
                            $arg_name: $crate::ValueRef::new($arg_name.into()),
                        )*
                    }
                }
            )*
        }
    };
}

/// Trait for registering features with the system
pub trait FeatureRegistry {
    /// Register a feature operation handler
    fn register_feature<F>(&mut self, feature: F) -> Result<(), FeatureError>
    where
        F: FeatureOperation + 'static;
}

/// Type-safe value resolver
pub trait TypedValueResolver {
    /// Resolve a value with type safety
    fn resolve_typed<T>(&self, value_ref: &ValueRef<T>) -> Result<T, FeatureError>
    where
        T: for<'de> serde::Deserialize<'de>;
}

/// Plugin error types
#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for PluginError {
    fn from(error: std::io::Error) -> Self {
        PluginError::IoError(error.to_string())
    }
}

/// Server context trait for plugins
#[async_trait::async_trait]
pub trait ServerContext: Send + Sync + std::fmt::Debug {
    /// Get the event system
    fn events(&self) -> std::sync::Arc<dyn std::any::Any + Send + Sync>;
    
    /// Get feature registry
    fn features(&self) -> std::sync::Arc<dyn std::any::Any + Send + Sync>;
    
    /// Execute system command
    async fn execute_system_command(&self, command: &str, args: &[&str]) -> Result<String, PluginError>;
}

/// Base plugin trait
#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;
    
    /// Get plugin version
    fn version(&self) -> &str;
    
    /// Get declared features
    fn declared_features(&self) -> Vec<String>;
    
    /// Pre-initialization phase
    async fn pre_init(&mut self, context: std::sync::Arc<dyn ServerContext>) -> Result<(), PluginError>;
    
    /// Initialization phase
    async fn init(&mut self, context: std::sync::Arc<dyn ServerContext>) -> Result<(), PluginError>;
    
    /// Shutdown phase
    async fn shutdown(&mut self, context: std::sync::Arc<dyn ServerContext>) -> Result<(), PluginError>;
}

pub use async_trait::async_trait;