use packages::{Product, calculate_total_price};

fn main() {
    let products = vec![
        Product {
            name: "Product 1".to_string(),
            price: 10.0,
        },
        Product {
            name: "Product 2".to_string(),
            price: 20.0,
        },
        Product {
            name: "Product 3".to_string(),
            price: 30.0,
        },
    ];
    let total_price = calculate_total_price(&products, "SAVE10");
    println!("Total price: {}", total_price);
    println!("");
    println!("Products:");
    for product in &products {
        println!("{}: {}", product.name, product.price);
    }
}
