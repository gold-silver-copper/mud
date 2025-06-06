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
    Move(Entity),
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
#[relationship(relationship_target = DualWayFrom)]
struct DualWayTo(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = DualWayTo)]
struct DualWayFrom(Vec<Entity>);

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
            DualWayTo(place),
            LocationName("bye".to_string()),
        ))
        .id();
    let player = commands
        .spawn((
            Player {}, // or .new(initial_location)
            ContainedBy(place2),
        ))
        .id();
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Player)>,
) {
    let eid = query.single().unwrap().0;
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        panic!("Panic!");
    }

    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;

    let direction = (horizontal + vertical) as i64 * 5;

    if direction != 0 {
        movement_event_writer.send(MovementAction::Move(eid));
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
    mut locations: Query<(&GameLocation, &Contains)>,
    mut commands: Commands,
) {
    for event in movement_event_reader.read() {
        match event {
            MovementAction::Move(entity) => {
                for (a, c) in locations.iter() {
                    if c.contains(entity) {
                        commands.entity(entity.clone()).remove::<ContainedBy>();
                        println!("Moved entity {:?} to ", entity);
                    }
                }
            }
        }
    }
}
