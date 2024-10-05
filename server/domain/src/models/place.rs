use crate::models::domain_error::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub enum PlaceType {
    EXTERIOR,
    INTERIOR,
    INVENTORY
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceId {
    pub(crate) id: i32,
}

impl PlaceId {
    fn validate(id: i32) -> Result<(), DomainError> {
        if id <= 0 {
            return Err(DomainError::PlaceIdError(
                "PlaceId should be strictly positive.".to_string(),
            ));
        }
        Ok(())
    }
    fn new(id: i32) -> Result<Self, DomainError> {
        PlaceId::validate(id)?;
        Ok(Self { id })
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    pub(crate) id: PlaceId,
    pub(crate) name: String,
    pub(crate) place_type: PlaceType
}
impl Place {
    fn validate(name: &String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::PlaceError(
                "Name should not be empty.".to_string(),
            ));
        }
        if name.trim().len() >= 30 {
            return Err(DomainError::PlaceError(
                "Name should be less then 30 characters.".to_string(),
            ));
        }
        Ok(())
    }
    pub fn get_id(&self) -> PlaceId {
        self.id.clone()
    }

    pub fn new(id: PlaceId, name: &String, place_type: &PlaceType) -> Result<Self, DomainError> {
        Place::validate(&name)?;
        Ok(Self {
            id,
            name: name.trim().to_string(),
            place_type: place_type.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_PLACE_ID_NUMBER: i32 = 42;

    trait MockPlaceId {
        fn mock(id: i32) -> PlaceId;
    }

    impl MockPlaceId for PlaceId {
        fn mock(id: i32) -> PlaceId {
            PlaceId { id }
        }
    }

    #[test]
    fn given_invalid_place_name_when_defining_place_should_reject_place() {
        const INVALID_NAME_EMPTY: &str = " ";
        const INVALID_NAME_30_OVER_FLOW_LIMIT: &str = "012345678901234567890123456789";
        let valid_id: PlaceId = PlaceId::mock(VALID_PLACE_ID_NUMBER);
        let valid_place_type = PlaceType::INVENTORY;

        assert!(matches!(
            Place::new(valid_id.clone(), &INVALID_NAME_EMPTY.to_string(), &valid_place_type),
            Err(DomainError::PlaceError(_))
        ));
        assert!(matches!(
            Place::new(
                valid_id.clone(),
                &INVALID_NAME_30_OVER_FLOW_LIMIT.to_string(),
                &valid_place_type
            ),
            Err(DomainError::PlaceError(_))
        ));
    }

    #[test]
    fn given_invalid_place_id_when_defining_place_should_reject_place() {
        const INVALID_PLACE_ID_NUMBER_NEGATIVE: i32 = -1;
        const INVALID_PLACE_ID_NUMBER_ZERO: i32 = 0;

        assert!(matches!(
            &PlaceId::new(INVALID_PLACE_ID_NUMBER_NEGATIVE),
            Err(DomainError::PlaceIdError(_))
        ));
        assert!(matches!(
            &PlaceId::new(INVALID_PLACE_ID_NUMBER_ZERO),
            Err(DomainError::PlaceIdError(_))
        ));
    }
}
