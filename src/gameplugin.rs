use bevy::ecs::query::Has;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(Update, (keyboard_input).chain())
            .add_systems(FixedUpdate, (movement,).chain());
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(String),
    Jump,
}

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(i64);

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    jump_impulse: JumpImpulse,
}

impl MovementBundle {
    pub const fn new(jump_impulse: i64) -> Self {
        Self {
            jump_impulse: JumpImpulse(jump_impulse),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30)
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = "lol".to_string();
    movement_event_writer.write(MovementAction::Move(direction));

    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Space) {
        movement_event_writer.write(MovementAction::Jump);
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&JumpImpulse,)>,
) {
    //fixedupdate defaults to 64hz
    let delta_time = 1.0 / 64.0;

    // let xyz = transform.rotation.xyz();
    for event in movement_event_reader.read() {
        for (jump_impulse) in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    println!("hi");
                }
                MovementAction::Jump => {
                    println!("bye");
                }
            }
        }
    }
}
