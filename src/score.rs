use bevy::prelude::*;

use crate::{GameState, ui::ScoreUi};

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Score>()
			.add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_points_ui));
    }
}

#[derive(Component, Reflect, Clone, Eq, PartialEq, Debug, Hash, Default, Deref, DerefMut)]
pub struct Score(pub u32);

fn update_points_ui(mut ui: Query<&mut Text, With<ScoreUi>>, score: Query<&Score>) {
    let mut text = ui.single_mut();

    text.sections[0].value = score.single().to_string();
}
