use std::collections::HashMap;
use crate::models::bill::{Bill, BillWithId};
use uuid::Uuid;
use crate::data::Data;
use std::sync::Mutex;


lazy_static! {
    static ref DATA: Mutex<HashMap<Uuid, Bill>> = Mutex::new(HashMap::new());
}


#[derive(Clone)]
pub struct Memory {}

impl Memory {
    pub fn new() -> Self {
        Self { }
    }
}

impl Data for Memory {

    fn add_bill(&self, bill: &Bill) -> Uuid {
        let mut data = DATA.lock().unwrap();
        let id = Uuid::new_v4();
        data.insert(id, bill.clone());
        id
    }

    fn delete_bill(&self, id: Uuid) -> Result<Uuid, String> {
        let mut data = DATA.lock().unwrap();
        match data.remove(&id) {
            Some(_) => Ok(id),
            None => Err("Bill not found".to_string())
        }
    }

    fn get_bill(&self, id: Uuid) -> Option<Bill> {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(bill) => Some(bill.clone()),
            None => None
        }
    }

    fn get_bills(&self) -> Vec<BillWithId> {
        let data = DATA.lock().unwrap();
        let vec = data.iter().map(|(id, bill)| {
            BillWithId {
                id: *id,
                bill: bill.clone()
            }
        }).collect::<Vec<BillWithId>>();
        vec
    }

    fn update_bill(&self, id: Uuid, bill: &Bill) -> Result<Uuid, String> {
        let mut data = DATA.lock().unwrap();
        match data.get_mut(&id) {
            Some(existing_bill) => {
                *existing_bill = bill.clone();
                Ok(id)
            },
            None => Err("Bill not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_bill() {
        let data = Memory::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.get_bill(id).unwrap(), bill);
    }

    #[test]
    fn test_delete_bill() {
        let data = Memory::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.delete_bill(id), Ok(id));
        assert_eq!(data.delete_bill(id), Err("Bill not found".to_string()));
    }

    #[test]
    fn test_get_bill() {
        let data = Memory::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.get_bill(id).unwrap(), bill);
    }

    #[test]
    fn test_get_bills() {
        let data = Memory::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        let bills = data.get_bills();
        let bill_with_id = BillWithId {
            id,
            bill
        };
        assert_eq!(bills.contains(&bill_with_id), true);
    }

    #[test]
    fn test_update_bill() {
        let data = Memory::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        let bill = Bill::new("test2".to_string());
        assert_eq!(data.update_bill(id, &bill), Ok(id));
        assert_eq!(data.get_bill(id).unwrap(), bill);
    }
}
