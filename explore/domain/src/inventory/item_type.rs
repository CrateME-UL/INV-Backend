use std::fmt::Error;

use uuid::Uuid;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ItemTypeNumber {
    number: Uuid,
}

impl ItemTypeNumber {
    fn new() -> Result<Self, Error> {
        Ok(Self {
            number: Uuid::new_v4(),
        })
    }

    fn get_number(&self) -> Uuid {
        self.number
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct ItemType {
    pub(crate) number: ItemTypeNumber,
}

impl ItemType {
    pub(crate) fn get_number(&self) -> ItemTypeNumber {
        self.number
    }
}
