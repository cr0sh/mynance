use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;

#[derive(Clone, Copy, Debug)]
pub struct OrderId(u64);

#[derive(Clone, Copy, Debug)]
pub struct Order {
    id: OrderId,
    original_quantity: Decimal,
    executed_quatnity: Decimal,
}

impl Order {
    pub fn new(id: OrderId, original_quantity: Decimal) -> Self {
        Self {
            id,
            original_quantity,
            executed_quatnity: Decimal::ZERO,
        }
    }

    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn original_quantity(&self) -> Decimal {
        self.original_quantity
    }

    pub fn executed_quatnity(&self) -> Decimal {
        self.executed_quatnity
    }
}

#[derive(Debug)]
pub struct Orderbook {
    bids: BTreeMap<Decimal, VecDeque<Order>>,
    asks: BTreeMap<Decimal, VecDeque<Order>>,
}

impl Orderbook {
    /// Returns an iterator yielding [`VecDeque`] references and their prices of each bid level from **the highest** price.
    pub fn bids(&self) -> impl Iterator<Item = (Decimal, &VecDeque<Order>)> + '_ {
        self.bids.iter().map(|(x, y)| (*x, y))
    }

    /// Returns an iterator yielding [`VecDeque`] references and their prices of each bid level from **the lowest** price.
    pub fn asks(&self) -> impl Iterator<Item = (Decimal, &VecDeque<Order>)> + '_ {
        self.asks.iter().map(|(x, y)| (*x, y))
    }
}

#[derive(Clone, Debug)]
pub struct OrderbookView {
    pub bids: Vec<OrderbookViewUnit>,
    pub asks: Vec<OrderbookViewUnit>,
}

#[derive(Clone, Copy, Debug)]
pub struct OrderbookViewUnit {
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Clone, Copy, Debug)]
pub struct Trade {
    pub price: Decimal,
    pub quantity: Decimal,
}
