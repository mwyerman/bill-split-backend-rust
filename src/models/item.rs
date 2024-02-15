use serde::{Deserialize, Serialize};
use crate::models::currency::Currency;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct LineItem {
    pub name: String,
    pub price: Currency,
    pub orderer: Option<String>,
}

impl LineItem {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            price: 0,
            orderer: None,
        }
    }

    pub fn from(
        name: String,
        price: u64,
        orderer: Option<String>,
    ) -> Self {
        Self {
            name,
            price,
            orderer,
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
        assert_eq!(item.price, 0);
    }

    #[test]
    fn test_from() {
        let item = LineItem::from(
            "test".to_string(),
            100,
            Some("test".to_string()),
        );
        assert_eq!(item.name, "test");
        assert_eq!(item.price, 100);
        assert_eq!(item.orderer, Some("test".to_string()));
    }
}
