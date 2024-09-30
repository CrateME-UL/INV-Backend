use crate::models::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemId {
    m_id: i32,
}

impl ItemId {
    fn validate(id: i32) -> Result<(), DomainError> {
        if id <= 0 {
            return Err(DomainError::ItemIdError(
                "ItemId should be strictly positive.".to_string(),
            ));
        }
        Ok(())
    }
    fn new(id: i32) -> Result<Self, DomainError> {
        ItemId::validate(id)?;
        Ok(Self { m_id: id })
    }
}
#[derive(Debug, PartialEq)]
pub struct Item<'a> {
    m_id: &'a ItemId,
    m_name: String,
}
impl<'a> Item<'a> {
    fn validate(id: &ItemId, name: &String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ItemError(
                "Name should not be empty.".to_string(),
            ));
        }
        if name.trim().len() >= 30 {
            return Err(DomainError::ItemError(
                "Name should be less then 30 characters.".to_string(),
            ));
        }
        Ok(())
    }

    fn new(id: &'a ItemId, name: &String) -> Result<Self, DomainError> {
        Item::validate(&id, &name)?;
        Ok(Self {
            m_id: id,
            m_name: name.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_ID_NUMBER: i32 = 42;
    // const VALID_NAME: &str = "Laurence";

    trait ItemMockable {
        fn new_unchecked(id: i32) -> ItemId;
    }

    impl ItemMockable for ItemId {
        fn new_unchecked(id: i32) -> ItemId {
            Self { m_id: id }
        }
    }

    #[test]
    fn given_invalid_item_name_when_defining_item_should_reject_definition() {
        const INVALID_NAME_EMPTY: &str = " ";
        const INVALID_NAME_30_OVER_FLOW_LIMIT: &str = "012345678901234567890123456789";
        let valid_id: ItemId = ItemId::new_unchecked(VALID_ID_NUMBER);

        assert!(matches!(
            Item::new(&valid_id, &INVALID_NAME_EMPTY.to_string()),
            Err(DomainError::ItemError(_))
        ));
        assert!(matches!(
            Item::new(&valid_id, &INVALID_NAME_30_OVER_FLOW_LIMIT.to_string()),
            Err(DomainError::ItemError(_))
        ));
    }

    #[test]
    fn given_invalid_id_when_defining_item_should_reject_definition() {
        const INVALID_ID_NUMBER_NEGATIVE: i32 = -1;
        const INVALID_ID_NUMBER_ZERO: i32 = 0;

        assert!(matches!(
            &ItemId::new(INVALID_ID_NUMBER_NEGATIVE),
            Err(DomainError::ItemIdError(_))
        ));
        assert!(matches!(
            &ItemId::new(INVALID_ID_NUMBER_ZERO),
            Err(DomainError::ItemIdError(_))
        ));
    }
}
