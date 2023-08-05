use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "2048".to_string(),
            ..default()
        }),
        ..default()
    }))
        .add_startup_system(setup)
        .run()

}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 40.0;

struct Board {
    size: u8,
}

fn spawn_board(mut commands: Commands) {
    let board = Board {size: 4};
    let physical_board_size = f32::from(board.size) * TILE_SIZE;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    physical_board_size,
                    physical_board_size,
                )),
                ..default()
            },
            ..default()
        })
        .insert(board);
}