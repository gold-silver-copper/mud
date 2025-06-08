use crate::components::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::{
    prelude::Stylize,
    style::Style,
    text::Text,
    widgets::{Block, List, ListDirection, ListState},
};
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
            let items: Vec<String> = ui_state
                .entity_selection_list
                .iter()
                .map(|e| format!("{:?}", e))
                .collect();
            let list = List::new(items)
                .block(Block::bordered().title("List"))
                .highlight_style(Style::new().reversed())
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

            // This should be stored outside of the function in your application state.
            let mut state = ListState::default();

            ui_state.entity_selection_index = ui_state
                .entity_selection_index
                .clamp(0, ui_state.entity_selection_list.len() - 1);

            state.select(Some(ui_state.entity_selection_index)); // select the forth item (0-indexed)

            context.draw(|frame| {
                frame.render_stateful_widget(list, frame.area(), &mut state);
            })?;
        }
    }

    Ok(())
}
