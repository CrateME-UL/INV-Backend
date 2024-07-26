mod service_handling;
pub use crate::service_handling::db_handler::{
    get_inventory_items_service, get_inventory_places_service, get_items_service,
    get_places_service,
};
pub use crate::service_handling::error::handle_rejection;

pub use crate::service_handling::auth::{create_jwt, with_auth};
pub use crate::service_handling::auth_handler::login_handler;
pub use crate::service_handling::crypto::{hash_pass, verify_hash};

use service_handling::error;
use warp::Rejection;

pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;
