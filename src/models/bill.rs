use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct LineItem {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
    pub ordered_by: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct Bill {
    pub name: String,
    pub subtotal: Option<u32>,
    pub tax: Option<u32>,
    pub tip: Option<u32>,
    pub total: Option<u32>,
    pub items: Vec<LineItem>,
}

impl Bill {
    pub fn new(name: String) -> Self {
        Self {
            name,
            subtotal: None,
            tax: None,
            tip: None,
            total: None,
            items: Vec::new(),
        }
    }

    pub fn new_with_info(
        name: String,
        subtotal: Option<u32>,
        tax: Option<u32>,
        tip: Option<u32>,
        total: Option<u32>
    ) -> Self {
        let mut bill = Self::new(name);
        bill.subtotal = subtotal;
        bill.tax = tax;
        bill.tip = tip;
        bill.total = total;
        bill
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct BillWithId {
    pub id: Uuid,
    pub bill: Bill,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bill = Bill::new("test".to_string());
        assert_eq!(bill.name, "test");
        assert_eq!(bill.items.len(), 0);
        assert_eq!(bill.subtotal, None);
        assert_eq!(bill.tax, None);
        assert_eq!(bill.tip, None);
        assert_eq!(bill.total, None);
    }

    #[test]
    fn test_new_with_info() {
        let bill = Bill::new_with_info(
            "test".to_string(),
            Some(100),
            Some(10),
            Some(10),
            Some(120),
        );
        assert_eq!(bill.name, "test");
        assert_eq!(bill.items.len(), 0);
        assert_eq!(bill.subtotal, Some(100));
        assert_eq!(bill.tax, Some(10));
        assert_eq!(bill.tip, Some(10));
        assert_eq!(bill.total, Some(120));
    }
}
