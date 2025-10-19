use retailchain::{
    PaymentProcessor, SupplyChainManager, InventoryManager, Blockchain,
    models::{Currency, SupplyChainAction},
};
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("🚀 Khởi chạy RetailChain...");

    // Khởi tạo các module
    let mut payment_processor = PaymentProcessor::new();
    let mut supply_chain = SupplyChainManager::new();
    let mut inventory = InventoryManager::new(10);
    let mut blockchain = Blockchain::new();

    // Demo: Thêm sản phẩm mới
    println!("\n📦 Thêm sản phẩm vào kho...");
    let product = inventory.add_product(
        "iPhone 14 Pro".to_string(),
        "IP14P-256".to_string(),
        "Latest Apple smartphone".to_string(),
        999.99,
        50,
        "Apple Inc.".to_string(),
    );

    println!("✅ Đã thêm sản phẩm: {} (SKU: {})", product.name, product.sku);

    // Demo: Theo dõi chuỗi cung ứng
    println!("\n📋 Ghi nhận chuỗi cung ứng...");
    supply_chain.add_product(product.clone());

    let _ = supply_chain.record_movement(
        product.id,
        "Factory China".to_string(),
        "Manufacturer".to_string(),
        SupplyChainAction::Manufactured,
        json!({"batch": "BATCH-001"}),
    );

    let _ = supply_chain.record_movement(
        product.id,
        "Warehouse Vietnam".to_string(),
        "Logistics Co.".to_string(),
        SupplyChainAction::Shipped,
        json!({"shipping_id": "SHIP-123"}),
    );

    // Demo: Xử lý thanh toán
    println!("\n💳 Xử lý thanh toán...");
    match payment_processor.process_payment(
        "customer_wallet_123".to_string(),
        "retailer_wallet_456".to_string(),
        999.99,
        Currency::USDT,
    ) {
        Ok(transaction) => {
            println!("✅ Thanh toán thành công: {} {}", transaction.amount, "USDT");
            
            // Thêm giao dịch vào blockchain
            blockchain.add_transaction(transaction.clone());
            println!("📝 Đã thêm giao dịch vào blockchain");
        }
        Err(e) => println!("❌ Lỗi thanh toán: {}", e),
    }

    // Demo thanh toán với loyalty points
    println!("\n🎫 Xử lý thanh toán với Loyalty Points...");
    match payment_processor.process_payment_with_loyalty(
        "customer_wallet_123".to_string(),
        "retailer_wallet_456".to_string(),
        50.0,
        100,
    ) {
        Ok(transaction) => {
            println!("✅ Thanh toán loyalty thành công: {} RETAIL", transaction.amount);
            blockchain.add_transaction(transaction);
        }
        Err(e) => println!("❌ Lỗi thanh toán loyalty: {}", e),
    }

    // Demo: Bán sản phẩm
    println!("\n🛒 Bán sản phẩm...");
    match inventory.sell_product(product.id, 1) {
        Ok(()) => {
            let updated_product = inventory.get_product(product.id).unwrap();
            println!("✅ Đã bán 1 {} - Tồn kho còn: {}", product.name, updated_product.quantity);
            
            // Ghi nhận bán hàng trong chuỗi cung ứng
            let _ = supply_chain.record_movement(
                product.id,
                "Retail Store HCM".to_string(),
                "Customer".to_string(),
                SupplyChainAction::Sold,
                json!({"sale_id": "SALE-001"}),
            );
        }
        Err(e) => println!("❌ Lỗi bán hàng: {}", e),
    }

    // Demo: Đào block mới
    println!("\n⛏️  Đào block mới...");
    match blockchain.mine_block() {
        Ok(block) => {
            println!("✅ Đã đào block #{} với {} giao dịch", 
                     block.index, block.transactions.len());
            println!("🔗 Hash: {}...", &block.hash[..16]);
        }
        Err(e) => println!("❌ Lỗi đào block: {}", e),
    }

    // Demo: Kiểm tra tính xác thực
    println!("\n🔍 Kiểm tra tính xác thực sản phẩm...");
    match supply_chain.verify_authenticity(product.id) {
        Ok(true) => println!("✅ Sản phẩm xác thực"),
        Ok(false) => println!("❌ Sản phẩm không xác thực"),
        Err(e) => println!("❌ Lỗi xác thực: {}", e),
    }

    // Demo: Chuyển đổi tiền tệ
    println!("\n💱 Chuyển đổi tiền tệ...");
    match payment_processor.convert_currency(100.0, &Currency::USDT, &Currency::BTC) {
        Ok(converted) => println!("✅ 100 USDT = {} BTC", converted),
        Err(e) => println!("❌ Lỗi chuyển đổi: {}", e),
    }

    // Demo: Sử dụng các method mới
    println!("\n📈 Thống kê nâng cao:");
    
    // Số lượng movements của sản phẩm
    if let Some(movements_count) = supply_chain.get_product_movements_count(product.id) {
        println!("• Số lần di chuyển của sản phẩm: {}", movements_count);
    }
    
    // Tổng giá trị kho
    println!("• Tổng số sản phẩm trong kho: {}", inventory.get_all_products().len());
    
    // Tổng số giao dịch đã xử lý
    println!("• Tổng số giao dịch: {}", payment_processor.get_all_transactions().len());

    // Hiển thị thông tin tổng quan
    println!("\n📊 THỐNG KÊ HỆ THỐNG:");
    println!("• Số block trong chain: {}", blockchain.get_chain_length());
    println!("• Blockchain hợp lệ: {}", blockchain.is_chain_valid());
    
    // Kiểm tra sản phẩm sắp hết hàng
    let low_stock = inventory.get_low_stock_products();
    if !low_stock.is_empty() {
        println!("⚠️  Cảnh báo: {} sản phẩm sắp hết hàng", low_stock.len());
        for product in low_stock {
            println!("   - {} (SKU: {}): {} sản phẩm", product.name, product.sku, product.quantity);
        }
    }

    println!("\n🎉 RetailChain hoạt động thành công!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use retailchain::models::Currency;

    #[test]
    fn test_payment_processing() {
        let mut processor = PaymentProcessor::new();
        let result = processor.process_payment(
            "addr1".to_string(),
            "addr2".to_string(),
            100.0,
            Currency::USDT,
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_blockchain_validity() {
        let blockchain = Blockchain::new();
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_currency_conversion() {
        let processor = PaymentProcessor::new();
        let result = processor.convert_currency(100.0, &Currency::USDT, &Currency::BTC);
        assert!(result.is_ok());
    }

    #[test]
    fn test_currency_equality() {
        let btc1 = Currency::BTC;
        let btc2 = Currency::BTC;
        let eth = Currency::ETH;
        
        assert_eq!(btc1, btc2);
        assert_ne!(btc1, eth);
    }
}