pub mod names;
pub mod quantity;
pub mod rarity;

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum ResouceType {
    Metal,
    Crystal,
    Radioactive,
    Organic,
}
