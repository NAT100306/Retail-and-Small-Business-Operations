use crate::models::{SupplyChainRecord, SupplyChainAction, Product};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

pub struct SupplyChainManager {
    records: HashMap<Uuid, Vec<SupplyChainRecord>>,
    products: HashMap<Uuid, Product>,
}

impl SupplyChainManager {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    pub fn record_movement(
        &mut self,
        product_id: Uuid,
        location: String,
        handler: String,
        action: SupplyChainAction,
        metadata: serde_json::Value,
    ) -> Result<SupplyChainRecord, SupplyChainError> {
        if !self.products.contains_key(&product_id) {
            return Err(SupplyChainError::ProductNotFound);
        }

        let record = SupplyChainRecord {
            product_id,
            location: location.clone(),
            handler: handler.clone(),
            timestamp: Utc::now(),
            action: action.clone(),
            metadata,
        };

        self.records
            .entry(product_id)
            .or_insert_with(Vec::new)
            .push(record.clone());

        println!("📦 Recorded {:?} for product {} at {}", action, product_id, location);
        
        Ok(record)
    }

    #[allow(dead_code)]
    pub fn get_product_history(&self, product_id: Uuid) -> Option<&Vec<SupplyChainRecord>> {
        self.records.get(&product_id)
    }

    pub fn verify_authenticity(&self, product_id: Uuid) -> Result<bool, SupplyChainError> {
        let history = self.records.get(&product_id)
            .ok_or(SupplyChainError::ProductNotFound)?;

        let has_manufacture = history.iter()
            .any(|record| matches!(record.action, SupplyChainAction::Manufactured));

        Ok(has_manufacture && history.len() > 0)
    }

    #[allow(dead_code)]
    pub fn get_current_location(&self, product_id: Uuid) -> Option<String> {
        self.records.get(&product_id)?
            .last()
            .map(|record| record.location.clone())
    }

    #[allow(dead_code)]
    pub fn get_product_movements_count(&self, product_id: Uuid) -> Option<usize> {
        self.records.get(&product_id).map(|records| records.len())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SupplyChainError {
    #[error("Product not found")]
    ProductNotFound,
    #[allow(dead_code)]
    #[error("Invalid movement")]
    InvalidMovement,
}