use bevy::prelude::*;

/// An event sent for a movement input action, targeting a specific entity.
#[derive(Event)]
pub enum MovementAction {
    Move(Entity),
}

#[derive(Event)]
pub enum PopulateAction {
    LocationConnections(Entity),
}

#[derive(Resource, Debug)]
pub struct UIState {
    pub active_element: UIElement,
    pub entity_selection_list: Vec<Entity>,
    pub entity_selection_index: usize,
}
impl Default for UIState {
    fn default() -> Self {
        UIState {
            active_element: UIElement::Base,
            entity_selection_list: Vec::new(),
            entity_selection_index: 0,
        }
    }
}

#[derive(Debug)]
pub enum UIElement {
    Base,
    Travel,
}

#[derive(Component, Debug)]
pub struct GameLocation {}
#[derive(Component, Debug)]
pub struct Creature {}

impl Default for GameLocation {
    fn default() -> Self {
        GameLocation {}
    }
}

impl Default for Creature {
    fn default() -> Self {
        Creature {}
    }
}

#[derive(Component, Debug)]
pub struct LocationName(pub String);
#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
#[relationship(relationship_target = Contains)]
pub struct ContainedBy(pub Entity);

#[derive(Component, Debug, Deref)]
#[relationship_target(relationship = ContainedBy)]
pub struct Contains(Vec<Entity>);

#[derive(Component, Debug)]
#[relationship(relationship_target = Connections)]
pub struct ConnectionTo(pub Entity);

#[derive(Component, Debug, Deref, DerefMut)]
#[relationship_target(relationship =ConnectionTo)]
pub struct Connections(Vec<Entity>);
