use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::text::Text;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_system).chain());
    }
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}
