use super::Product;

pub fn calculate_discounted_price(product: &Product, promo_code: &str) -> f64 {
    match promo_code {
        "SAVE10" => product.price * 0.9,
        "SAVE20" => product.price * 0.8,
        _ => product.price,
    }
}
