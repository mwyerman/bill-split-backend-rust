use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::item::LineItem;
use crate::models::currency::Currency;


#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Bill {
    name: String,
    total: Option<Currency>,
    items: HashMap<u16, LineItem>,
    counter: u16,
}

impl Bill {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total: None,
            items: HashMap::new(),
            counter: 0,
        }
    }

    pub fn from(
        name: String,
        total: Currency,
    ) -> Self {
        let mut bill = Self::new(name);
        bill.total = Some(total);
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

    pub fn calculate_subtotal(&self) -> Currency {
        let mut total = 0;
        for (_, item) in &self.items {
            total += item.price;
        }
        total
    }

    pub fn get_bill_for(&self, orderer: &str) -> Bill {
        let mut bill = Bill::new(self.name.clone());
        for (_, item) in &self.items {
            if item.orderer == Some(orderer.to_string()) {
                bill.add_item(item.clone());
            }
        }

        // split the total proportionally
        if !self.total.is_none() {
            let my_subtotal = bill.calculate_subtotal();
            let all_subtotal = self.calculate_subtotal();
            let ratio = my_subtotal as f64 / all_subtotal as f64;
            let total = self.total.unwrap() as f64;
            bill.total = Some((total * ratio) as Currency);
        }
        bill
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
        assert_eq!(bill.total, None);
    }

    #[test]
    fn test_from() {
        let bill = Bill::from(
            "test".to_string(),
            120,
        );
        assert_eq!(bill.name, "test");
        assert_eq!(bill.items.len(), 0);
        assert_eq!(bill.total, Some(120));
    }

    #[test]
    fn test_add_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 100, None);
        let id = bill.add_item(item.clone());
        assert_eq!(bill.items.get(&id).unwrap(), &item);
    }

    #[test]
    fn test_delete_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 100, None);
        let id = bill.add_item(item.clone());
        assert_eq!(bill.delete_item(id), Ok(id));
        assert_eq!(bill.delete_item(id), Err("Item not found".to_string()));
    }

    #[test]
    fn test_update_item() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 100, None);
        let id = bill.add_item(item.clone());
        let item = LineItem::from("test2".to_string(), 200, None);
        assert_eq!(bill.update_item(id, item.clone()), Ok(id));
        assert_eq!(bill.items.get(&id).unwrap(), &item);
        assert_eq!(bill.update_item(100, item.clone()), Err("Item not found".to_string()));
    }

    #[test]
    fn test_calculate_subtotal() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 100, None);
        bill.add_item(item.clone());
        let item = LineItem::from("test2".to_string(), 200, None);
        bill.add_item(item.clone());
        assert_eq!(bill.calculate_subtotal(), 300);
    }

    #[test]
    fn test_get_bill_for() {
        let mut bill = Bill::new("test".to_string());
        let item = LineItem::from("test".to_string(), 100, Some("test".to_string()));
        bill.add_item(item.clone());
        let item = LineItem::from("test2".to_string(), 200, Some("test2".to_string()));
        bill.add_item(item.clone());

        let bill1 = bill.get_bill_for("test");
        assert_eq!(bill1.total, None);
        assert_eq!(bill1.items.len(), 1);
        assert_eq!(bill1.calculate_subtotal(), 100);

        let bill2 = bill.get_bill_for("test2");
        assert_eq!(bill2.total, None);
        assert_eq!(bill2.items.len(), 1);
        assert_eq!(bill2.calculate_subtotal(), 200);
    }

    #[test]
    fn test_get_bill_for_with_total() {
        let mut bill = Bill::new("test".to_string());
        bill.total = Some(330);
        let item = LineItem::from("test".to_string(), 100, Some("test".to_string()));
        bill.add_item(item.clone());
        let item = LineItem::from("test2".to_string(), 200, Some("test2".to_string()));
        bill.add_item(item.clone());

        let bill1 = bill.get_bill_for("test");
        assert_eq!(bill1.total, Some(110));
        assert_eq!(bill1.items.len(), 1);
        assert_eq!(bill1.calculate_subtotal(), 100);

        let bill2 = bill.get_bill_for("test2");
        assert_eq!(bill2.total, Some(220));
        assert_eq!(bill2.items.len(), 1);
        assert_eq!(bill2.calculate_subtotal(), 200);
    }
}
