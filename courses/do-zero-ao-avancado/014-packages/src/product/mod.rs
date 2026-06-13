pub mod promo;

use promo::calculate_discounted_price;

pub struct Product {
    pub name: String,
    pub price: f64,
}

pub fn calculate_total_price(products: &[Product], promo_code: &str) -> f64 {
    products
        .iter()
        .map(|product| calculate_discounted_price(product, promo_code))
        .sum()
}
