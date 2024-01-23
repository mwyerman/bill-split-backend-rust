pub mod memory;

use crate::models::bill::{Bill, BillWithId};
use uuid::Uuid;

pub trait Data {
    fn add_bill(&self, bill: &Bill) -> Uuid;
    fn delete_bill(&self, id: Uuid) -> bool;
    fn get_bill(&self, id: Uuid) -> Option<Bill>;
    fn get_bills(&self) -> Vec<BillWithId>;
}

#[derive(Clone)]
pub enum DataProvider {
    Memory(memory::Memory)
}

impl Data for DataProvider {
    fn add_bill(&self, bill: &Bill) -> Uuid {
        match self {
            DataProvider::Memory(memory) => memory.add_bill(bill)
        }
    }

    fn delete_bill(&self, id: Uuid) -> bool {
        match self {
            DataProvider::Memory(memory) => memory.delete_bill(id)
        }
    }

    fn get_bill(&self, id: Uuid) -> Option<Bill> {
        match self {
            DataProvider::Memory(memory) => memory.get_bill(id)
        }
    }

    fn get_bills(&self) -> Vec<BillWithId> {
        match self {
            DataProvider::Memory(memory) => memory.get_bills()
        }
    }
}
