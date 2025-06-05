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

/// The world location of a character.
#[derive(Component, Debug)]
pub struct WorldLocation(pub i64);

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct PlayerBundle {
    world_location: WorldLocation,
}

impl PlayerBundle {
    pub const fn new(world_location: i64) -> Self {
        Self {
            world_location: WorldLocation(world_location),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new(30)
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
    query: Query<Entity, With<WorldLocation>>, // All controllable entities
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
    mut locations: Query<&mut WorldLocation>,
) {
    for event in movement_event_reader.read() {
        match event {
            MovementAction::Move(entity, direction) => {
                if let Ok(mut location) = locations.get_mut(*entity) {
                    location.0 += direction;
                    println!("Moved entity {:?} to {}", entity, location.0);
                }
            }
            MovementAction::Jump(entity) => {
                if let Ok(location) = locations.get_mut(*entity) {
                    println!("Entity {:?} jumped at location {}", entity, location.0);
                }
            }
        }
    }
}
