use bevy::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(Update, keyboard_input)
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, debug_relationships)
            .init_resource::<UIState>()
            .add_systems(FixedUpdate, movement);
    }
}

/// An event sent for a movement input action, targeting a specific entity.
#[derive(Event)]
pub enum MovementAction {
    Move(Entity),
}

#[derive(Resource, Debug)]
pub struct UIState {
    active_element: UIElement,
    entity_selection_list: Vec<Entity>,
    entity_selection_index: usize,
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
struct LocationName(String);
#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
#[relationship(relationship_target = Contains)]
struct ContainedBy(Entity);

#[derive(Component, Debug, Deref)]
#[relationship_target(relationship = ContainedBy)]
struct Contains(Vec<Entity>);

#[derive(Component, Debug)]
#[relationship(relationship_target = Connections)]
struct ConnectionTo(Entity);

#[derive(Component, Debug, Deref)]
#[relationship_target(relationship =ConnectionTo)]
struct Connections(Vec<Entity>);

fn spawn_player(mut commands: Commands) {
    let place = commands
        .spawn((
            GameLocation::default(), // or .new(initial_location)
            LocationName("hi".to_string()),
        ))
        .id();
    let place2 = commands
        .spawn((
            GameLocation::default(), // or .new(initial_location)
            ConnectionTo(place),
            LocationName("bye".to_string()),
        ))
        .id();
    let player = commands
        .spawn((
            Player {}, // or .new(initial_location)
            ContainedBy(place),
        ))
        .id();
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UIState>,
    query: Query<(Entity, &Player)>,
) {
    let player_id = query.single().unwrap().0;

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        panic!("Panic!");
    }

    let key_t = keyboard_input.any_pressed([KeyCode::KeyT]);

    match ui_state.active_element {
        UIElement::Base => {
            if key_t {
                movement_event_writer.send(MovementAction::Move(player_id));
            }
        }
        UIElement::Travel => (),
    }
}

fn debug_relationships(
    // Not all of our entities are targeted by something, so we use `Option` in our query to handle this case.
    relations_query: Query<(&LocationName, &Contains)>,
) {
    let mut relationships = String::new();

    for (location_name, contains) in relations_query.iter() {
        let targeted_by_string = &location_name.0;

        relationships.push_str(&format!("{targeted_by_string} contains {:#?}", contains));
    }

    println!("{}", relationships);
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    mut movement_event_reader: EventReader<MovementAction>,
    mut locations: Query<(&GameLocation, &Connections, &Contains)>,
    mut commands: Commands,
) {
    for event in movement_event_reader.read() {
        match event {
            MovementAction::Move(entity) => {
                for (a, b, c) in locations.iter() {
                    if c.contains(entity) {
                        commands.entity(entity.clone()).remove::<ContainedBy>();
                        commands.entity(entity.clone()).insert(ContainedBy(b[0]));
                        println!("Moved entity {:?} to ", entity);
                    }
                }
            }
        }
    }
}
