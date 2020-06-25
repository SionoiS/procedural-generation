/*
    Trying to enforce some logic with types.
    Ids are always 20 alphanumeric utf-8 chars in Firestore
    Some place restrict ids to one or more types.
    egg. Only TableId here but TableId & ChairId there.
*/

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum Module {
    Sampler(DatabaseId),
    Scanner(DatabaseId),
    Sensor(DatabaseId),
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum Resource {
    Metal(DatabaseId),
    Crystal(DatabaseId),
    Radioactive(DatabaseId),
    Organic(DatabaseId),
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum Ship {
    Starter(DatabaseId),
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub struct Asteroid {
    id: DatabaseId,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub struct User {
    id: DatabaseId,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
///20 alphanumeric characters uniquely identifying a document in Firestore
pub struct DatabaseId {
    data: [u8; 20],
}

impl DatabaseId {
    pub fn default() -> Self {
        Self { data: [0u8; 20] }
    }

    pub fn from_string(seed: &str) -> Result<Self, &'static str> {
        if seed.len() > 20 {
            return Err("Length must be 20");
        }
        let mut array: [u8; 20] = [128; 20];

        for (index, character) in seed.char_indices() {
            if !character.is_ascii_alphanumeric() {
                return Err("Only alphanumeric character allowed");
            } else {
                array[index] = character as u8;
            }
        }

        Ok(Self { data: array })
    }
}
