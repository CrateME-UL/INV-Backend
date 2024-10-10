use crate::models::domain_error::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub enum PlaceType {
    EXTERIOR,
    INTERIOR,
    INVENTORY,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceNo {
    pub(crate) number: i32,
}

impl PlaceNo {
    fn validate(number: i32) -> Result<(), DomainError> {
        if number <= 0 {
            return Err(DomainError::PlaceIdError(
                "PlaceNo should be strictly positive.".to_string(),
            ));
        }
        Ok(())
    }
    fn new(number: i32) -> Result<Self, DomainError> {
        PlaceNo::validate(number)?;
        Ok(Self { number })
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    pub(crate) number: PlaceNo,
    pub(crate) name: String,
    pub(crate) place_type: PlaceType,
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
    pub fn get_number(&self) -> PlaceNo {
        self.number.clone()
    }

    pub fn new(
        number: PlaceNo,
        name: &String,
        place_type: &PlaceType,
    ) -> Result<Self, DomainError> {
        Place::validate(&name)?;
        Ok(Self {
            number,
            name: name.trim().to_string(),
            place_type: place_type.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_PLACE_NUMBER: i32 = 42;

    trait StubPlaceNumber {
        fn mock(number: i32) -> PlaceNo;
    }

    impl StubPlaceNumber for PlaceNo {
        fn mock(number: i32) -> PlaceNo {
            PlaceNo { number }
        }
    }

    #[test]
    fn given_invalid_place_name_when_defining_place_should_reject_place() {
        const INVALID_NAME_EMPTY: &str = " ";
        const INVALID_ANY_NAME_30_OR_MORE_OVER_FLOW_LIMIT: &str = "012345678901234567890123456789";
        let valid_number: PlaceNo = PlaceNo::mock(VALID_PLACE_NUMBER);
        let valid_place_type = PlaceType::INVENTORY;

        assert!(matches!(
            Place::new(
                valid_number.clone(),
                &INVALID_NAME_EMPTY.to_string(),
                &valid_place_type
            ),
            Err(DomainError::PlaceError(_))
        ));
        assert!(matches!(
            Place::new(
                valid_number.clone(),
                &INVALID_ANY_NAME_30_OR_MORE_OVER_FLOW_LIMIT.to_string(),
                &valid_place_type
            ),
            Err(DomainError::PlaceError(_))
        ));
    }

    #[test]
    fn given_invalid_place_number_when_defining_place_should_reject_place() {
        const INVALID_ANY_NEGATIVE_PLACE_NUMBER: i32 = -1;
        const INVALID_PLACE_NUMBER_ZERO: i32 = 0;

        assert!(matches!(
            &PlaceNo::new(INVALID_ANY_NEGATIVE_PLACE_NUMBER),
            Err(DomainError::PlaceIdError(_))
        ));
        assert!(matches!(
            &PlaceNo::new(INVALID_PLACE_NUMBER_ZERO),
            Err(DomainError::PlaceIdError(_))
        ));
    }
}
