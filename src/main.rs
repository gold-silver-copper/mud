use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

mod gameplugin;
use gameplugin::CharacterControllerPlugin;
use ratatui::text::Text;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CharacterControllerPlugin,
            RatatuiPlugins {
                enable_input_forwarding: true,
                ..default()
            },
        ))
        .add_systems(Update, draw_system)
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

#[derive(Component)]
pub struct Character {
    pub character_id: i64,
}
