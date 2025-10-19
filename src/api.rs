use warp::Filter;
use std::convert::Infallible;

pub async fn run_api_server() {
    println!("🚀 Starting RetailChain API Server...");
    
    // Simple routes
    let hello = warp::path!("hello")
        .map(|| "Hello from RetailChain API!");

    let blockchain = warp::path!("api" / "blockchain")
        .map(|| {
            r#"{
                "status": "running", 
                "blocks": 1,
                "message": "Blockchain API is working!"
            }"#
        });

    let products = warp::path!("api" / "products")
        .map(|| {
            r#"{
                "products": [
                    {"id": 1, "name": "iPhone 14", "price": 999.99},
                    {"id": 2, "name": "Samsung Galaxy", "price": 899.99}
                ]
            }"#
        });

    let routes = hello
        .or(blockchain)
        .or(products)
        .with(warp::cors().allow_any_origin());

    println!("🌐 Server running at: http://localhost:8080");
    println!("📡 Available endpoints:");
    println!("   http://localhost:8080/hello");
    println!("   http://localhost:8080/api/blockchain"); 
    println!("   http://localhost:8080/api/products");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}