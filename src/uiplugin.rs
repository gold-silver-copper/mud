use crate::components::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::text::Text;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_system).chain());
    }
}

fn draw_system(mut context: ResMut<RatatuiContext>, mut ui_state: ResMut<UIState>) -> Result {
    match ui_state.active_element {
        UIElement::Base => {
            context.draw(|frame| {
                let text = Text::raw("hello world\npress 'q' to quit");
                frame.render_widget(text, frame.area());
            })?;
        }
        UIElement::Travel => {
            let textik = format!(
                "connections here are {:#?}",
                &ui_state.entity_selection_list,
            );

            context.draw(|frame| {
                let text = Text::raw(textik);
                frame.render_widget(text, frame.area());
            })?;
        }
    }

    Ok(())
}
