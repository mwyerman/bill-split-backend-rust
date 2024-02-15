use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::item::LineItem;
use crate::models::currency::Currency;


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
            Some(Currency::from_dollars(1.0)),
            Some(Currency::from_dollars(0.1)),
            Some(Currency::from_dollars(0.1)),
            Some(Currency::from_dollars(1.2)),
        );
        assert_eq!(bill.name, "test");
        assert_eq!(bill.items.len(), 0);
        assert_eq!(bill.subtotal, Some(Currency::from_dollars(1.0)));
        assert_eq!(bill.tax, Some(Currency::from_dollars(0.1)));
        assert_eq!(bill.tip, Some(Currency::from_dollars(0.1)));
        assert_eq!(bill.total, Some(Currency::from_dollars(1.2)));
    }

    #[test]
    fn test_add_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 1.0);
        let id = bill.add_item(item.clone());
        assert_eq!(bill.items.get(&id).unwrap(), &item);
    }

    #[test]
    fn test_delete_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 1.0);
        let id = bill.add_item(item.clone());
        assert_eq!(bill.delete_item(id), Ok(id));
        assert_eq!(bill.delete_item(id), Err("Item not found".to_string()));
    }

    #[test]
    fn test_update_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 1.0);
        let id = bill.add_item(item.clone());
        let item = LineItem::from("test2".to_string(), 2.0);
        assert_eq!(bill.update_item(id, item.clone()), Ok(id));
        assert_eq!(bill.items.get(&id).unwrap(), &item);
        assert_eq!(bill.update_item(100, item.clone()), Err("Item not found".to_string()));
    }
}
