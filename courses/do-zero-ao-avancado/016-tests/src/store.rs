pub struct Store {
    balance: i32,
}

impl Store {
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub fn sell(&mut self, price: i32) -> Result<(), String> {
        if price < 0 {
            return Err("price must be positive".to_string());
        }
        self.balance += price;
        Ok(())
    }
    pub fn balance(&self) -> i32 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sell_success() {
        let mut store = Store::new();
        assert_eq!(store.balance(), 0);
        match store.sell(20) {
            Ok(_) => assert_eq!(store.balance(), 20),
            Err(_) => panic!("expected success"),
        }
    }

    #[test]
    fn test_sell_failure() {
        let mut store = Store::new();
        assert_eq!(store.balance(), 0);
        match store.sell(-5) {
            Ok(_) => panic!("expected failure"),
            Err(e) => {
                assert_eq!(store.balance(), 0);
                assert_eq!(e, "price must be positive")
            }
        }
    }
}
