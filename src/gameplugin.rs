use crate::components::*;
use bevy::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_event::<PopulateAction>()
            .add_systems(Update, keyboard_input)
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, debug_relationships)
            .init_resource::<UIState>()
            .add_systems(FixedUpdate, populate_action_solver)
            .add_systems(FixedUpdate, movement);
    }
}

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
    mut populate_event_writer: EventWriter<PopulateAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UIState>,
    query: Query<(Entity, &Player)>,
) {
    let player_id = query.single().unwrap().0;

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        panic!("Panic!");
    }

    let key_t = keyboard_input.just_pressed(KeyCode::KeyT);
    let key_w = keyboard_input.just_pressed(KeyCode::KeyW);

    match ui_state.active_element {
        UIElement::Base => {
            if key_t {
                ui_state.active_element = UIElement::Travel;
                populate_event_writer.send(PopulateAction::LocationConnections(player_id));
                //populate entity list with connections
                // movement_event_writer.send(MovementAction::Move(player_id));
            }
            if key_w {}
        }
        UIElement::Travel => {
            if key_w {
                ui_state.active_element = UIElement::Base;
            }
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

fn populate_action_solver(
    mut populate_action_reader: EventReader<PopulateAction>,
    mut locations: Query<(&GameLocation, &Connections, &Contains)>,
    mut commands: Commands,
    mut ui_state: ResMut<UIState>,
) {
    for event in populate_action_reader.read() {
        match event {
            PopulateAction::LocationConnections(player_id) => {
                for (a, connections, contained_ents) in locations.iter() {
                    if contained_ents.contains(player_id) {
                        ui_state.entity_selection_list = connections.to_vec();
                    }
                }
            }
        }
    }
}
