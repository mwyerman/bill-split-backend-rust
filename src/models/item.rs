use serde::{Deserialize, Serialize};
use crate::models::currency::Currency;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct LineItem {
    pub name: String,
    pub price: Currency,
}

impl LineItem {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            price: Currency::new(),
        }
    }

    pub fn from(
        name: String,
        price: f64,
    ) -> Self {
        Self {
            name,
            price: Currency::from_dollars(price),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = LineItem::new();
        assert_eq!(item.name, "");
        assert_eq!(item.price.to_cents(), 0);
    }

    #[test]
    fn test_from() {
        let item = LineItem::from(
            "test".to_string(),
            1.0,
        );
        assert_eq!(item.name, "test");
        assert_eq!(item.price.to_cents(), 100);
    }
}
