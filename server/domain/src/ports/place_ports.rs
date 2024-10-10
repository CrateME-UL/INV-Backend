use std::{error::Error, future::Future, pin::Pin};

use crate::Place;

pub trait PlaceRepository: Send + Sync {
    fn fetch_place_by_name(
        &self,
        place_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Place>, Box<dyn Error>>> + Send>>;
}
