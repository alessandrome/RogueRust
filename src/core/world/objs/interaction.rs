use std::fmt::Debug;
use crate::core::world::World;

pub enum InteractionType<'a> {
    Action(fn (&mut World) -> Result<String, String>),
    Option(&'a dyn Interactable),
}

pub enum InteractionResult<'a> {
    InteractionList(Vec<InteractionType<'a>>),
    ActionInteraction(Result<String, String>)
}

pub trait Interactable: Debug {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn interaction(&self) -> Vec<InteractionType>;
    fn interact(&self, world: &mut World) -> InteractionType;
}
