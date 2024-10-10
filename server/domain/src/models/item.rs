use crate::models::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemNo {
    pub(crate) number: i32,
}

impl ItemNo {
    fn validate(number: i32) -> Result<(), DomainError> {
        if number <= 0 {
            return Err(DomainError::ItemNumberError(
                "ItemNo should be strictly positive.".to_string(),
            ));
        }
        Ok(())
    }
    fn new(number: i32) -> Result<Self, DomainError> {
        ItemNo::validate(number)?;
        Ok(Self { number })
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub(crate) number: ItemNo,
    pub(crate) name: String,
}
impl Item {
    fn validate(name: &String) -> Result<(), DomainError> {
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
    pub fn get_number(&self) -> ItemNo {
        self.number.clone()
    }

    pub fn new(number: ItemNo, name: &String) -> Result<Self, DomainError> {
        Item::validate(&name)?;
        Ok(Self {
            number,
            name: name.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_NUMBER: i32 = 42;

    trait StubItemNo {
        fn mock(number: i32) -> ItemNo;
    }

    impl StubItemNo for ItemNo {
        fn mock(number: i32) -> ItemNo {
            ItemNo { number }
        }
    }

    #[test]
    fn given_invalid_item_name_when_defining_item_should_reject_item() {
        const INVALID_NAME_EMPTY: &str = " ";
        const INVALID_ANY_NAME_30_OR_MORE_OVER_FLOW_LIMIT: &str = "012345678901234567890123456789";
        let valid_number: ItemNo = ItemNo::mock(VALID_NUMBER);

        assert!(matches!(
            Item::new(valid_number.clone(), &INVALID_NAME_EMPTY.to_string()),
            Err(DomainError::ItemError(_))
        ));
        assert!(matches!(
            Item::new(
                valid_number.clone(),
                &INVALID_ANY_NAME_30_OR_MORE_OVER_FLOW_LIMIT.to_string()
            ),
            Err(DomainError::ItemError(_))
        ));
    }

    #[test]
    fn given_invalid_item_number_when_defining_item_should_reject_item() {
        const INVALID_ANY_NEGATIVE_ITEM_NUMBER: i32 = -1;
        const INVALID_ITEM_NUMBER_ZERO: i32 = 0;

        assert!(matches!(
            &ItemNo::new(INVALID_ANY_NEGATIVE_ITEM_NUMBER),
            Err(DomainError::ItemNumberError(_))
        ));
        assert!(matches!(
            &ItemNo::new(INVALID_ITEM_NUMBER_ZERO),
            Err(DomainError::ItemNumberError(_))
        ));
    }
}
