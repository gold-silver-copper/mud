use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

use ratatui::text::Text;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RatatuiPlugins {
                enable_input_forwarding: true,
                ..default()
            },
        ))
        .add_systems(Update, draw_system)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}
fn keyboard_input_system(keys: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        app_exit.write_default();
    }
    if keys.just_pressed(KeyCode::KeyP) {
        panic!("Panic!");
    }
}
