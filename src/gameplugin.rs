use bevy::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(Update, keyboard_input)
            .add_systems(Startup, spawn_player)
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

impl Default for GameLocation {
    fn default() -> Self {
        GameLocation {}
    }
}

#[derive(Component, Debug)]
pub enum LocationTypes {}

/// The entity that this entity is targeting.
///
/// This is the source of truth for the relationship,
/// and can be modified directly to change the target.
#[derive(Component, Debug)]
#[relationship(relationship_target = ContainedBy)]
struct Contains(Entity);

/// All entities that are targeting this entity.
///
/// This component is updated reactively using the component hooks introduced by deriving
/// the [`Relationship`] trait. We should not modify this component directly,
/// but can safely read its field. In a larger project, we could enforce this through the use of
/// private fields and public getters.
#[derive(Component, Debug)]
#[relationship_target(relationship = Contains)]
struct ContainedBy(Vec<Entity>);

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
    commands.spawn((
        PlayerBundle::default(), // or .new(initial_location)
    ));
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity>, // All controllable entities
) {
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
