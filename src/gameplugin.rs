use bevy::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(Update, keyboard_input)
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, debug_relationships)
            .add_systems(FixedUpdate, movement);
    }
}

/// An event sent for a movement input action, targeting a specific entity.
#[derive(Event)]
pub enum MovementAction {
    Move(Entity, i64),
    Jump(Entity),
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
#[relationship(relationship_target = ContainedBy)]
struct Contains(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = Contains)]
struct ContainedBy(Vec<Entity>);

#[derive(Component, Debug)]
#[relationship(relationship_target = DualWayFrom)]
struct DualWayTo(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = DualWayTo)]
struct DualWayFrom(Vec<Entity>);

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct PlayerBundle {
    //  world_location: WorldLocation,
}

impl PlayerBundle {
    pub const fn new() -> Self {
        Self {
        //    world_location: WorldLocation(Entity::PLACEHOLDER),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new()
    }
}

fn spawn_player(mut commands: Commands) {
    let player = commands
        .spawn((
            PlayerBundle::default(), // or .new(initial_location)
        ))
        .id();
    let place = commands
        .spawn((
            GameLocation::default(), // or .new(initial_location)
            LocationName("hi".to_string()),
        ))
        .id();
    commands.spawn((
        GameLocation::default(), // or .new(initial_location)
        Contains(player),
        DualWayTo(place),
        LocationName("bye".to_string()),
    ));
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity>, // All controllable entities
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        panic!("Panic!");
    }
    // Send input to all entities with WorldLocation
    for entity in &query {
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let horizontal = right as i8 - left as i8;
        let vertical = up as i8 - down as i8;

        let direction = (horizontal + vertical) as i64 * 5;

        if direction != 0 {
            movement_event_writer.send(MovementAction::Move(entity, direction));
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            movement_event_writer.send(MovementAction::Jump(entity));
        }
    }
}

fn debug_relationships(
    // Not all of our entities are targeted by something, so we use `Option` in our query to handle this case.
    relations_query: Query<(&LocationName, &Contains)>,
) {
    let mut relationships = String::new();

    for (location_name, contains) in relations_query.iter() {
        let targeted_by_string = &location_name.0;

        relationships.push_str(&format!("{targeted_by_string} contains {}", contains.0));
    }

    println!("{}", relationships);
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    mut movement_event_reader: EventReader<MovementAction>,
    //  mut locations: Query<&mut MovementAction>,
) {
    for event in movement_event_reader.read() {
        match event {
            MovementAction::Move(entity, direction) => {
                if let location = 1 {
                    //  location.0 += direction;
                    println!("Moved entity {:?} to {}", entity, location);
                }
            }
            MovementAction::Jump(entity) => {
                if let location = 1 {
                    println!("Entity {:?} jumped at location {}", entity, location);
                }
            }
        }
    }
}
