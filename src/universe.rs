use crate::types::FactLibrary;
use crate::types::Types;
use crate::goals::Goal;

pub struct Universe{
    fact_library: FactLibrary,
    goals: Vec<Goal>
}