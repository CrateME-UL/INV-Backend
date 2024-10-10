use std::{collections::HashMap, error::Error};

use domain::{Place, PlaceRepository};

#[derive(Debug)]
pub struct InMemoryPlaceRepository {
    place_list: HashMap<String, Place>,
}

impl InMemoryPlaceRepository {
    pub fn new(place_list: HashMap<String, Place>) -> Self {
        Self { place_list }
    }
}

impl PlaceRepository for InMemoryPlaceRepository {
    fn fetch_place_by_name(
        &self,
        place_name: String,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Place>, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }
}
