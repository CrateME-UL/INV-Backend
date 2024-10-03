use crate::models::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemId {
    pub(crate) m_id: i32,
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
    pub(crate) m_id: &'a ItemId,
    pub(crate) m_name: String,
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

    pub fn new(id: &'a ItemId, name: &String) -> Result<Self, DomainError> {
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
    const VALID_NAME: &str = "Alice";

    trait MockItemId {
        fn new_valid(id: i32) -> ItemId;
    }

    impl MockItemId for ItemId {
        fn new_valid(id: i32) -> ItemId {
            ItemId { m_id: id }
        }
    }

    #[test]
    fn given_invalid_item_name_when_defining_item_should_reject_item() {
        const INVALID_NAME_EMPTY: &str = " ";
        const INVALID_NAME_30_OVER_FLOW_LIMIT: &str = "012345678901234567890123456789";
        let valid_id: ItemId = ItemId::new_valid(VALID_ID_NUMBER);

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
    fn given_invalid_id_when_defining_item_should_reject_item() {
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
    //i want to create an object -> but first, i need to validate the parameters for defining an object are valid -> valid name, valid id 
    //-> valid name implies that i verify the buisness logic such as constraints of name, types
    //-> it also implies if the name of the item already exists -> i should not create it
    //-> it also implies that i need a repository reference 
    //-> it also implies that we need a factory to manage the creation of those objects from the api server so the factory can reference the repository with an interface
    //-> it also implies that whenever i create an item in memory, it stores the item in the repository
    //-> when the item is stored, the lifecycle of the object ends in memory? or we could keep it in memory and fetch the data at start

    //-> we will need an agregate object to handle all of the inventory logic to manage collections of Items, Places, InventoryItems
    //-> items and places are frequently access, if the application grows, this is important to have access quickcly, it reduces the number of querry the database wich is the most costly
    //-> for InventoryItems, it's different, they are only accessed at certain times and querrying the database makes sense because the number of different items and places is far lower than the InventoryItems
    //-> conclusion: create a Repository (for persistant storage and querrying) and an Inventory[using Repository] (for common accessed objects, object constraints, aggregates, processing)

    // #[test]
    // fn given_name_when_fetching_item_by_name_should_return_item() {
    //     const TAKEN_NAME: &str = "Laurence";
    //     let valid_id: ItemId = ItemId::new_valid(VALID_ID_NUMBER);

    //     assert!(matches!(
    //         Item::new(&valid_id, &TAKEN_NAME.to_string()),
    //         Err(DomainError::ItemError(_))
    //     ));

    // }
    // #[test]
    // fn given_taken_name_when_defining_item_should_reject_item() {
    //     const TAKEN_NAME: &str = "Laurence";
    //     let valid_id: ItemId = ItemId::new_valid(VALID_ID_NUMBER);

    //     assert!(matches!(
    //         Item::new(&valid_id, &TAKEN_NAME.to_string()),
    //         Err(DomainError::ItemError(_))
    //     ));

    // }
}
