use crate::core::world::World;

pub enum InteractionType {
    Action(fn (&mut World) -> Result<String, String>),
    Option(dyn Interactable),
}

pub enum InteractionResult {
    InteractionList(Vec<InteractionType>),
    ActionInteraction(Result<String, String>)
}

pub trait Interactable {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn interaction(&self) -> Vec<InteractionType>;
    fn interact(&self, world: &mut World) -> InteractionType;
}
