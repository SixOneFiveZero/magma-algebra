use crate::traits::{AddGroup, MultGroup};

// Abelian Group: Group with Commutativity
pub trait AddAbelianGroup: AddGroup {}
pub trait MultAbelianGroup: MultGroup {}