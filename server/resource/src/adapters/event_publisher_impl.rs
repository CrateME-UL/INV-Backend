use domain::{EventPublisherPort, OrderPlaced};
use tokio::sync::broadcast;

pub struct EventPublisherAdapter {
    pub sender: broadcast::Sender<OrderPlaced>,
}

impl EventPublisherPort for EventPublisherAdapter {
    fn process_order_placed(&self, event: OrderPlaced) {
        let _ = self.sender.send(event);
    }
}
