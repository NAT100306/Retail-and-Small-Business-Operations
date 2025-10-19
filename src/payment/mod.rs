use crate::models::{Transaction, TransactionStatus, Currency, RetailToken};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

pub struct PaymentProcessor {
    transactions: HashMap<Uuid, Transaction>,
    exchange_rates: HashMap<Currency, f64>,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        rates.insert(Currency::BTC, 45000.0);
        rates.insert(Currency::ETH, 3000.0);
        rates.insert(Currency::USDT, 1.0);
        
        rates.insert(
            Currency::RETAIL(RetailToken {
                symbol: "RETAIL".to_string(),
                amount: 0.0,
                loyalty_points: 0,
            }), 
            1.0
        );
        
        Self {
            transactions: HashMap::new(),
            exchange_rates: rates,
        }
    }

    pub fn process_payment(
        &mut self,
        from_address: String,
        to_address: String,
        amount: f64,
        currency: Currency,
    ) -> Result<Transaction, PaymentError> {
        self.validate_payment(from_address.as_str(), amount, &currency)?;

        let transaction = Transaction {
            id: Uuid::new_v4(),
            from_address: from_address.clone(),
            to_address: to_address.clone(),
            amount,
            currency: currency.clone(),
            timestamp: Utc::now(),
            status: TransactionStatus::Completed,
        };

        self.transactions.insert(transaction.id, transaction.clone());
        
        println!("✅ Payment processed: {} {:?} from {} to {}", 
                 amount, currency, from_address, to_address);
        
        Ok(transaction)
    }

    pub fn convert_currency(
        &self,
        amount: f64,
        from: &Currency,
        to: &Currency,
    ) -> Result<f64, PaymentError> {
        let from_rate = self.exchange_rates.get(from)
            .ok_or(PaymentError::UnsupportedCurrency)?;
        let to_rate = self.exchange_rates.get(to)
            .ok_or(PaymentError::UnsupportedCurrency)?;

        let converted_amount = (amount * from_rate) / to_rate;
        println!("💱 Currency converted: {} {:?} = {} {:?}", 
                 amount, from, converted_amount, to);
        
        Ok(converted_amount)
    }

    fn validate_payment(
        &self,
        from_address: &str,
        amount: f64,
        _currency: &Currency,
    ) -> Result<(), PaymentError> {
        if amount <= 0.0 {
            return Err(PaymentError::InvalidAmount);
        }

        if from_address.is_empty() {
            return Err(PaymentError::InvalidAddress);
        }

        let simulated_balance = 1000.0;
        if amount > simulated_balance {
            return Err(PaymentError::InsufficientFunds);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_transaction(&self, id: Uuid) -> Option<&Transaction> {
        self.transactions.get(&id)
    }

    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    pub fn process_payment_with_loyalty(
        &mut self,
        from_address: String,
        to_address: String,
        amount: f64,
        loyalty_points: u32,
    ) -> Result<Transaction, PaymentError> {
        let retail_token = RetailToken {
            symbol: "RETAIL".to_string(),
            amount,
            loyalty_points,
        };

        self.process_payment(from_address, to_address, amount, Currency::RETAIL(retail_token))
    }

    pub fn get_exchange_rate(&self, currency: &Currency) -> Option<f64> {
        self.exchange_rates.get(currency).copied()
    }

    #[allow(dead_code)]
    pub fn get_total_processed_amount(&self) -> f64 {
        self.transactions.values()
            .map(|transaction| transaction.amount)
            .sum()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Unsupported currency")]
    UnsupportedCurrency,
}