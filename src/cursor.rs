use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup).add_system(follow_cursor);
	}
}

#[derive(Component)]
pub struct FollowCursor;

fn setup(mut commands: Commands) {
	commands.spawn(SpatialBundle::default()).insert(FollowCursor).insert(Name::new("Cursor"));
}

fn follow_cursor(mut cursor_followers: Query<&mut Transform, With<FollowCursor>>, mut cursor_moved: EventReader<CursorMoved>, windows: Res<Windows>) {
	let cursor_position = if let Some(event) = cursor_moved.iter().next() {
		event.position.extend(0.0)
	} else {
		return;
	};

	let window = windows.get_primary().expect("No application window found!");

	for mut transform in cursor_followers.iter_mut() {
		transform.translation = cursor_position - Vec3::new(window.width() as f32, window.height() as f32, 0.) / 2.;
	}
}