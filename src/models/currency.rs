use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct Currency {
    pub dollars: u16,
    pub cents: u16,
}

impl Currency {
    pub fn new() -> Self {
        Self {
            dollars: 0,
            cents: 0,
        }
    }

    pub fn from(
        dollars: u16,
        cents: u16,
    ) -> Self {
        Self {
            dollars,
            cents,
        }
    }

    pub fn to_cents(&self) -> u16 {
        self.dollars * 100 + self.cents
    }

    pub fn from_cents(cents: u16) -> Self {
        Self {
            dollars: cents / 100,
            cents: cents % 100,
        }
    }

    pub fn to_dollars(&self) -> f64 {
        self.dollars as f64 + self.cents as f64 / 100.0
    }

    pub fn from_dollars(dollars: f64) -> Self {
        let cents = (dollars * 100.0).round() as u16;
        Self {
            dollars: cents / 100,
            cents: cents % 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let currency = Currency::new();
        assert_eq!(currency.dollars, 0);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn test_from() {
        let currency = Currency::from(
            1,
            50,
        );
        assert_eq!(currency.dollars, 1);
        assert_eq!(currency.cents, 50);
    }

    #[test]
    fn test_to_cents() {
        let currency = Currency::from(
            1,
            50,
        );
        assert_eq!(currency.to_cents(), 150);
    }

    #[test]
    fn test_from_cents() {
        let currency = Currency::from_cents(150);
        assert_eq!(currency.dollars, 1);
        assert_eq!(currency.cents, 50);
    }

    #[test]
    fn test_to_dollars() {
        let currency = Currency::from(
            1,
            50,
        );
        assert_eq!(currency.to_dollars(), 1.50);
    }

    #[test]
    fn test_from_dollars() {
        let currency = Currency::from_dollars(1.50);
        assert_eq!(currency.dollars, 1);
        assert_eq!(currency.cents, 50);
    }
}
