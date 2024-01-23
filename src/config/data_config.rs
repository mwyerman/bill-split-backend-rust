use crate::data::DataProvider;
use crate::data::memory::Memory;

#[derive(Clone)]
pub struct DataConfig {
    pub provider: DataProvider
}

impl DataConfig {
    pub fn new() -> Self {
        Self {
            provider: DataProvider::Memory(Memory::new())
        }
    }
}
