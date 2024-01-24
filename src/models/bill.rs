use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Currency = u32;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct LineItem {
    pub name: String,
    pub price: Currency,
    pub quantity: u32,
    pub ordered_by: Vec<String>,
}

impl LineItem {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            price: 0,
            quantity: 0,
            ordered_by: vec![],
        }
    }

    pub fn from(
        name: String,
        price: Currency,
        quantity: u32,
        ordered_by: Vec<String>,
    ) -> Self {
        Self {
            name,
            price,
            quantity,
            ordered_by,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Bill {
    pub name: String,
    pub subtotal: Option<Currency>,
    pub tax: Option<Currency>,
    pub tip: Option<Currency>,
    pub total: Option<Currency>,
    pub items: HashMap<u16, LineItem>,
    counter: u16,
}

impl Bill {
    pub fn new(name: String) -> Self {
        Self {
            name,
            subtotal: None,
            tax: None,
            tip: None,
            total: None,
            items: HashMap::new(),
            counter: 0,
        }
    }

    pub fn from(
        name: String,
        subtotal: Option<Currency>,
        tax: Option<Currency>,
        tip: Option<Currency>,
        total: Option<Currency>
    ) -> Self {
        let mut bill = Self::new(name);
        bill.subtotal = subtotal;
        bill.tax = tax;
        bill.tip = tip;
        bill.total = total;
        bill
    }

    fn get_counter(&mut self) -> u16 {
        self.counter += 1;
        self.counter - 1
    }

    pub fn add_item(&mut self, item: LineItem) -> u16 {
        let id = self.get_counter();
        self.items.insert(id, item);
        id
    }

    pub fn delete_item(&mut self, id: u16) -> Result<u16, String> {
        match self.items.remove(&id) {
            Some(_) => Ok(id),
            None => Err("Item not found".to_string())
        }
    }

    pub fn update_item(&mut self, id: u16, item: LineItem) -> Result<u16, String> {
        match self.items.get_mut(&id) {
            Some(existing_item) => {
                *existing_item = item;
                Ok(id)
            },
            None => Err("Item not found".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
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
    fn test_from() {
        let bill = Bill::from(
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

    #[test]
    fn test_add_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem {
            name: "test".to_string(),
            price: 100,
            quantity: 1,
            ordered_by: vec!["test".to_string()],
        };
        let id = bill.add_item(item.clone());
        assert_eq!(bill.items.get(&id).unwrap(), &item);
    }

    #[test]
    fn test_delete_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem {
            name: "test".to_string(),
            price: 100,
            quantity: 1,
            ordered_by: vec!["test".to_string()],
        };
        let id = bill.add_item(item.clone());
        assert_eq!(bill.delete_item(id), Ok(id));
        assert_eq!(bill.delete_item(id), Err("Item not found".to_string()));
    }

    #[test]
    fn test_update_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem {
            name: "test".to_string(),
            price: 100,
            quantity: 1,
            ordered_by: vec!["test".to_string()],
        };
        let id = bill.add_item(item.clone());
        let item = LineItem {
            name: "test2".to_string(),
            price: 200,
            quantity: 2,
            ordered_by: vec!["test2".to_string()],
        };
        assert_eq!(bill.update_item(id, item.clone()), Ok(id));
        assert_eq!(bill.items.get(&id).unwrap(), &item);
        assert_eq!(bill.update_item(100, item.clone()), Err("Item not found".to_string()));
    }
}
