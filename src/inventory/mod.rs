use crate::models::Product;
use uuid::Uuid;
use std::collections::HashMap;

pub struct InventoryManager {
    products: HashMap<Uuid, Product>,
    low_stock_threshold: u32,
}

impl InventoryManager {
    pub fn new(low_stock_threshold: u32) -> Self {
        Self {
            products: HashMap::new(),
            low_stock_threshold,
        }
    }

    pub fn add_product(
        &mut self,
        name: String,
        sku: String,
        description: String,
        price: f64,
        quantity: u32,
        manufacturer: String,
    ) -> Product {
        let product = Product {
            id: Uuid::new_v4(),
            sku,
            name,
            description,
            price,
            quantity,
            manufacturer,
            created_at: chrono::Utc::now(),
        };

        self.products.insert(product.id, product.clone());
        product
    }

    pub fn update_stock(&mut self, product_id: Uuid, new_quantity: u32) -> Result<(), InventoryError> {
        let product = self.products.get_mut(&product_id)
            .ok_or(InventoryError::ProductNotFound)?;

        product.quantity = new_quantity;
        Ok(())
    }

    pub fn sell_product(&mut self, product_id: Uuid, quantity: u32) -> Result<(), InventoryError> {
        let product = self.products.get_mut(&product_id)
            .ok_or(InventoryError::ProductNotFound)?;

        if product.quantity < quantity {
            return Err(InventoryError::InsufficientStock);
        }

        product.quantity -= quantity;
        Ok(())
    }

    pub fn get_low_stock_products(&self) -> Vec<&Product> {
        self.products.values()
            .filter(|product| product.quantity <= self.low_stock_threshold)
            .collect()
    }

    pub fn get_product(&self, product_id: Uuid) -> Option<&Product> {
        self.products.get(&product_id)
    }

    pub fn get_all_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("Product not found")]
    ProductNotFound,
    #[error("Insufficient stock")]
    InsufficientStock,
}