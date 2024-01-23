use std::collections::HashMap;
use crate::bill::{Bill, BillWithId};
use std::sync::Mutex;
use uuid::Uuid;


pub struct Data {
    bills: Mutex<HashMap<Uuid, Bill>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            bills: Mutex::new((|| {
                let mut bills = HashMap::new();
                let bill = Bill::new("test".to_string());
                bills.insert(Uuid::new_v4(), bill);
                bills
            })())
        }
    }

    pub fn add_bill(&self, bill: &Bill) -> Uuid {
        let mut data = self.bills.lock().unwrap();
        let id = Uuid::new_v4();
        data.insert(id, bill.clone());
        id
    }


    pub fn delete_bill(&self, id: Uuid) -> bool {
        let mut data = self.bills.lock().unwrap();
        match data.remove(&id) {
            Some(_) => true,
            None => false
        }
    }


    pub fn get_bill(&self, id: Uuid) -> Option<Bill> {
        let data = self.bills.lock().unwrap();
        match data.get(&id) {
            Some(bill) => Some(bill.clone()),
            None => None
        }
    }


    pub fn get_bills(&self) -> Vec<BillWithId> {
        let data = self.bills.lock().unwrap();
        let vec = data.iter().map(|(id, bill)| {
            BillWithId {
                id: *id,
                bill: bill.clone()
            }
        }).collect::<Vec<BillWithId>>();
        vec
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_bill() {
        let data = Data::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.get_bill(id).unwrap(), bill);
    }

    #[test]
    fn test_delete_bill() {
        let data = Data::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.delete_bill(id), true);
        assert_eq!(data.delete_bill(id), false);
    }

    #[test]
    fn test_get_bill() {
        let data = Data::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        assert_eq!(data.get_bill(id).unwrap(), bill);
    }

    #[test]
    fn test_get_bills() {
        let data = Data::new();
        let bill = Bill::new("test".to_string());
        let id = data.add_bill(&bill);
        let bills = data.get_bills();
        assert_eq!(bills.len(), 1);
        assert_eq!(bills[0].id, id);
        assert_eq!(bills[0].bill, bill);
    }
}
