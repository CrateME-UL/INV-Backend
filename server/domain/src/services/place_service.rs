use std::sync::Arc;

use crate::{models::domain_error::DomainError, ports::place_ports::PlaceRepository, Place};

#[derive(Clone)]
pub struct PlaceService {
    place_repository: Arc<dyn PlaceRepository>,
}

impl PlaceService {
    pub fn new(place_repository: Arc<dyn PlaceRepository>) -> Self {
        Self { place_repository }
    }

    pub async fn fetch_place_by_name(&self, item_name: String) -> Result<Place, DomainError> {
        match self.place_repository.fetch_place_by_name(item_name).await {
            Ok(place) => match place {
                Some(_) => Ok(place.unwrap()),
                None => Err(DomainError::PlaceError("Place not found.".to_string())),
            },
            _ => Err(DomainError::PlaceError(
                "Unhandled error while fetching the place with inventory from repository."
                    .to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::place::PlaceType, PlaceNo};

    use super::*;

    const ANY_PLACE_NUMBER: i32 = 42;

    trait StubPlaceNo {
        fn stub(number: i32) -> PlaceNo;
    }

    impl StubPlaceNo for PlaceNo {
        fn stub(number: i32) -> PlaceNo {
            PlaceNo { number }
        }
    }

    trait StubPlace {
        fn stub(number: PlaceNo, name: &str, place_type: PlaceType) -> Self;
    }

    impl StubPlace for Place {
        fn stub(number: PlaceNo, name: &str, place_type: PlaceType) -> Self {
            Self {
                number,
                name: name.trim().to_string(),
                place_type,
            }
        }
    }

    pub struct MockPlaceRepository {
        pub stub_place: Option<Place>,
    }

    impl MockPlaceRepository {
        fn mock_with_place(stub_data: &Option<Place>) -> Self {
            Self {
                stub_place: stub_data.clone(),
            }
        }
    }

    impl PlaceRepository for MockPlaceRepository {
        fn fetch_place_by_name(
            &self,
            _place_name: String,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Option<Place>, Box<dyn std::error::Error>>>
                    + Send,
            >,
        > {
            let result = self.stub_place.clone();
            Box::pin(async move { Ok(result) })
        }
    }

    #[tokio::test]
    async fn given_existing_item_name_when_fetching_item_by_name_then_return_corresponding_item() {
        const ANY_PLACE_NAME: &str = "Bob's Palace";
        let valid_id: PlaceNo = PlaceNo::stub(ANY_PLACE_NUMBER);
        let expected_id: PlaceNo = PlaceNo::stub(ANY_PLACE_NUMBER);
        let valid_item: Place = Place::stub(valid_id, ANY_PLACE_NAME, PlaceType::INTERIOR);
        let inventory: PlaceService = PlaceService::new(Arc::new(
            MockPlaceRepository::mock_with_place(&Option::Some(valid_item)),
        ));

        let actual_id = inventory
            .fetch_place_by_name(ANY_PLACE_NAME.to_string())
            .await
            .unwrap()
            .get_number();

        assert_eq!(expected_id, actual_id);
    }

    #[tokio::test]
    async fn given_not_existing_place_name_when_fetching_place_by_name_then_reject_it() {
        const NOT_EXISTING_ANY_PLACE_NAME: &str = "Bob's Place";
        let not_existing_place = None;
        let inventory: PlaceService = PlaceService::new(Arc::new(
            MockPlaceRepository::mock_with_place(&not_existing_place),
        ));

        assert!(matches!(
            inventory
                .fetch_place_by_name(NOT_EXISTING_ANY_PLACE_NAME.to_string())
                .await,
            Err(DomainError::PlaceError(_))
        ));
    }
}
