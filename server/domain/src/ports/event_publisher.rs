use crate::events::events::OrderPlaced;

pub trait EventPublisherPort {
    fn publish_order_placed(&self, event: OrderPlaced);
}
