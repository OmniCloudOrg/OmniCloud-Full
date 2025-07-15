use dashmap::DashMap;
use serde_json::Value;
use std::sync::Arc;
use thiserror::Error;

pub type EventHandler = Arc<dyn Fn(Value) -> Result<Value, EventError> + Send + Sync>;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Handler not found for event: {0}")]
    HandlerNotFound(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
}

#[derive(Debug, Clone)]
pub struct EventKey {
    pub provider: String,
    pub feature: String,
    pub method: String,
}

impl EventKey {
    pub fn new(provider: &str, feature: &str, method: &str) -> Self {
        Self {
            provider: provider.to_string(),
            feature: feature.to_string(),
            method: method.to_string(),
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}::{}::{}", self.provider, self.feature, self.method)
    }
}

pub struct EventRegistry {
    handlers: DashMap<String, EventHandler>,
    feature_registry: DashMap<String, Vec<String>>, // provider -> features
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            handlers: DashMap::new(),
            feature_registry: DashMap::new(),
        }
    }
    
    pub fn register_handler<F>(&self, key: EventKey, handler: F) 
    where
        F: Fn(Value) -> Result<Value, EventError> + Send + Sync + 'static,
    {
        let handler_key = key.to_string();
        self.handlers.insert(handler_key, Arc::new(handler));
        
        // Track features for validation
        self.feature_registry
            .entry(key.provider.clone())
            .or_insert_with(Vec::new)
            .push(key.feature);
    }
    
    pub async fn dispatch_event(&self, key: &EventKey, payload: Value) -> Result<Value, EventError> {
        let handler_key = key.to_string();
        
        let handler = self.handlers
            .get(&handler_key)
            .ok_or_else(|| EventError::HandlerNotFound(handler_key.clone()))?;
        
        // Execute handler (no match statements needed!)
        handler(payload).map_err(|e| EventError::ExecutionFailed(e.to_string()))
    }
    
    pub fn get_provider_features(&self, provider: &str) -> Vec<String> {
        self.feature_registry
            .get(provider)
            .map(|entry| entry.clone())
            .unwrap_or_default()
    }
    
    pub fn validate_provider(&self, provider: &str, declared_features: &[String]) -> Result<(), EventError> {
        let registered_features = self.get_provider_features(provider);
        
        for declared in declared_features {
            if !registered_features.contains(declared) {
                return Err(EventError::ExecutionFailed(format!(
                    "Provider '{}' declared feature '{}' but no handlers registered",
                    provider, declared
                )));
            }
        }
        
        Ok(())
    }
    
    pub fn list_handlers(&self) -> Vec<String> {
        self.handlers.iter().map(|entry| entry.key().clone()).collect()
    }
}

// Global registry instance
static GLOBAL_REGISTRY: std::sync::OnceLock<EventRegistry> = std::sync::OnceLock::new();

pub fn get_global_registry() -> &'static EventRegistry {
    GLOBAL_REGISTRY.get_or_init(|| EventRegistry::new())
}

// Convenience functions for registration
pub fn register_event_handler<F>(provider: &str, feature: &str, method: &str, handler: F)
where
    F: Fn(Value) -> Result<Value, EventError> + Send + Sync + 'static,
{
    let key = EventKey::new(provider, feature, method);
    get_global_registry().register_handler(key, handler);
}

pub async fn dispatch_event(provider: &str, feature: &str, method: &str, payload: Value) -> Result<Value, EventError> {
    let key = EventKey::new(provider, feature, method);
    get_global_registry().dispatch_event(&key, payload).await
}