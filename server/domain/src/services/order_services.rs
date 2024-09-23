use crate::events::events::OrderPlaced;

#[derive(Clone)]
pub struct OrderService;

impl OrderService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn place_order(&self, order_id: u64, amount: f64) -> OrderPlaced {
        println!("Order placed: ID = {}, amount = {}", order_id, amount);
        OrderPlaced { order_id, amount }
    }
}
