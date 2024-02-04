use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowTheme};

#[derive(Component)]
struct Snake {}

#[derive(Component, PartialEq)]
enum Direction {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 255., 0.),
                custom_size: Some(Vec2::new(20., 20.)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Snake {},
        Direction::NONE,
    ));
}

fn move_snake(mut transform_query: Query<&mut Transform, With<Snake>>, direction_query: Query<&Direction, With<Snake>>) {
    let direction = direction_query.get_single().unwrap();

    if let Ok(mut transform) = transform_query.get_single_mut() {
        if *direction == Direction::UP {
            transform.translation += Vec3::new(0., 20., 0.);
        }

        if *direction == Direction::DOWN {
            transform.translation += Vec3::new(0., -20., 0.);
        }

        if *direction == Direction::LEFT {
            transform.translation += Vec3::new(-20., 0., 0.);
        }

        if *direction == Direction::RIGHT {
            transform.translation += Vec3::new(20., 0., 0.);
        }
    }
}

fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut snake_query: Query<&mut Direction, With<Snake>>) {
    if let Ok(mut direction) = snake_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Left) && *direction != Direction::RIGHT {
            *direction = Direction::LEFT;
        }

        if keyboard_input.just_pressed(KeyCode::Right) && *direction != Direction::LEFT {
            *direction = Direction::RIGHT;
        }

        if keyboard_input.just_pressed(KeyCode::Up) && *direction != Direction::DOWN {
            *direction = Direction::UP;
        }

        if keyboard_input.just_pressed(KeyCode::Down) && *direction != Direction::UP {
            *direction = Direction::DOWN;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_snake))
        .add_systems(Update, (move_snake, handle_input))
        .run();
}
