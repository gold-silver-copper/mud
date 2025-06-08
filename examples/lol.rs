use bevy::prelude::*;

use bevy_ratatui::RatatuiPlugins;

use mud::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CharacterControllerPlugin,
            UIPlugin,
            RatatuiPlugins {
                enable_input_forwarding: true,
                ..default()
            },
        ))
        .run();
}
