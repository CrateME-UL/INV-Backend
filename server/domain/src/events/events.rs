#[derive(Debug, Clone)]
pub struct OrderPlaced {
    pub order_id: u64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct DeliverEvent {
    pub order_id: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    OrderPlaced(OrderPlaced),
    DeliverEvent(DeliverEvent),
}
