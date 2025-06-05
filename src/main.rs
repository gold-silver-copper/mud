use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

mod gameplugin;
use gameplugin::CharacterControllerPlugin;
mod uiplugin;
use ratatui::text::Text;
use uiplugin::UIPlugin;

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

#[derive(Component)]
pub struct Character {
    pub character_id: i64,
}
