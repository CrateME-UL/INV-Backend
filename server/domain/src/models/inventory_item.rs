//TODO: start TDD with commented code to help if needed
//TODO: implement the builder pattern because of the number of optionals
// use crate::{ItemId, PlaceId};

// use super::place::PlaceType;

// #[derive(Debug)]
// pub enum NbOfItemsError {
//     InvalidQuantity(i32),
// }

// impl std::fmt::Display for NbOfItemsError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             NbOfItemsError::InvalidQuantity(qte) => {
//                 write!(
//                     f,
//                     "Invalid quantity: {}. Quantity must be greater than 0.",
//                     qte
//                 )
//             }
//         }
//     }
// }

// impl std::error::Error for NbOfItemsError {}
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct NbOfItems {
//     qte: i32,
// }

// impl NbOfItems {
//     pub fn new(qte: i32) -> Result<Self, NbOfItemsError> {
//         if qte > 0 {
//             Ok(Self { qte })
//         } else {
//             Err(NbOfItemsError::InvalidQuantity(qte))
//         }
//     }

//     pub fn value(&self) -> i32 {
//         self.qte
//     }
// }
// #[derive(Debug)]
// pub struct InventoryItem {
//     m_item_id: Option<ItemId>,
//     m_place_id: Option<PlaceId>,
//     m_place_type: Option<PlaceType>,
//     m_place_name: String,
//     m_item_name: String,
//     m_nb_of_items: Option<NbOfItems>,
// }

// impl InventoryItem {
//     pub fn new(place_name: String, item_name: String) -> Self {
//         Self {
//             m_item_id: None,
//             m_place_id: None,
//             m_place_type: None,
//             m_place_name: place_name,
//             m_item_name: item_name,
//             m_nb_of_items: None,
//         }
//     }

//     pub fn set_nb_of_items(&mut self, nb_of_items: i32) -> Result<(), NbOfItemsError> {
//         match NbOfItems::new(nb_of_items) {
//             Ok(nb) => {
//                 self.m_nb_of_items = Some(nb);
//                 Ok(())
//             }
//             Err(e) => Err(e),
//         }
//     }
// }
