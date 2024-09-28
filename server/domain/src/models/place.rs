#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceId {
    id: i32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceType {
    In,
    Out,
    Inv,
}

impl PlaceType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "IN" => Some(PlaceType::In),
            "OUT" => Some(PlaceType::Out),
            "INV" => Some(PlaceType::Inv),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PlaceType::In => "IN",
            PlaceType::Out => "OUT",
            PlaceType::Inv => "INV",
        }
    }
}

pub struct Place {
    place_id: PlaceId,
    place_name: String,
    place_type: PlaceType,
}

impl Place {
    pub fn new(place_id: PlaceId, place_name: String, place_type: PlaceType) -> Self {
        Self {
            place_id,
            place_name,
            place_type,
        }
    }
}

impl PlaceId {
    pub fn new(id: i32) -> Self {
        Self { id }
    }

    pub fn value(&self) -> &i32 {
        &self.id
    }
}
