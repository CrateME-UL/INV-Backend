use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use domain::{DeliverEvent, Event, OrderPlaced, OrderService};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AxumServerAdapter {
    pub sender: broadcast::Sender<Event>,
    pub order_service: OrderService,
}

impl AxumServerAdapter {
    pub fn new(sender: broadcast::Sender<Event>, order_service: OrderService) -> Self {
        Self {
            sender,
            order_service,
        }
    }

    pub async fn listen_and_start_server(self, mut receiver: broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                match event {
                    Event::OrderPlaced(order) => {
                        println!(
                            "Received OrderPlaced event: ID = {}, Amount = {}",
                            order.order_id, order.amount
                        );
                    }
                    Event::DeliverEvent(deliver_event) => {
                        println!(
                            "Received DeliverEvent: Order ID = {}, Status = {}",
                            deliver_event.order_id, deliver_event.status
                        );
                    }
                }
            }
        });

        self.start_server().await;
    }

    async fn start_server(&self) {
        let app = Router::new()
            .route("/", get(health))
            .route("/order", post(place_order))
            .route("/deliver", post(deliver_order))
            .with_state(Arc::new(Mutex::new(self.clone())));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

async fn place_order(
    State(axum_server): State<Arc<Mutex<AxumServerAdapter>>>,
) -> Json<&'static str> {
    let server = axum_server.lock().unwrap();
    let order_event = Event::OrderPlaced(OrderPlaced {
        order_id: 1,
        amount: 99.99,
    });
    let _ = server.sender.send(order_event);

    Json("Order placed")
}

async fn deliver_order(
    State(axum_server): State<Arc<Mutex<AxumServerAdapter>>>,
) -> Json<&'static str> {
    let server = axum_server.lock().unwrap();
    let deliver_event = Event::DeliverEvent(DeliverEvent {
        order_id: 1,
        status: "Delivered".to_string(),
    });
    let _ = server.sender.send(deliver_event);

    Json("Order delivered")
}

pub async fn health() -> &'static str {
    "Hello, World!"
}
