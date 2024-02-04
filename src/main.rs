use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{PresentMode, PrimaryWindow, WindowTheme};
use bevy_framepace;
use bevy_framepace::Limiter;
use rand::Rng;

#[derive(Component)]
struct Snake {}

#[derive(Component)]
struct Food {}

#[derive(Component)]
struct SnakeSegment {}

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

fn set_framerate(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(30.);
}

fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 255., 0.),
                custom_size: Some(Vec2::new(20., 20.)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Snake {},
        Direction::NONE,
    ));

    for i in 1..3 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 255., 0.),
                    custom_size: Some(Vec2::new(20., 20.)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_xyz(
                    window.width() / 2.,
                    window.height() / 2. - (20. * i as f32),
                    0.,
                ),
                ..default()
            },
            SnakeSegment {},
        ));
    }
}

fn spawn_food(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..(window.width() / 20.) as i32) as f32;
    let y = rng.gen_range(0..(window.height() / 20.) as i32) as f32;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(255., 0., 0.),
                custom_size: Some(Vec2::new(20., 20.)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            transform: Transform::from_xyz(
                x * 20., y * 20., 0.
            ),
            ..default()
        },
        Food {},
    ));
}

fn move_snake(
    mut transform_query: Query<&mut Transform, (With<Snake>, Without<SnakeSegment>)>,
    direction_query: Query<&Direction, With<Snake>>,
    mut segment_query: Query<&mut Transform, (With<SnakeSegment>, Without<Snake>)>,
) {
    let direction = direction_query.get_single().unwrap();

    if let Ok(mut transform) = transform_query.get_single_mut() {
        let current_head_position = transform.translation;

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

        let mut prev_translation = current_head_position;
        for mut segment in segment_query.iter_mut() {
            let prev = segment.clone();
            segment.translation = prev_translation;

            prev_translation = prev.translation;
        }

    }
}

fn handle_food_eaten(mut commands: Commands, mut food_query: Query<&mut Transform, (With<Food>, Without<Snake>)>, mut snake_query: Query<&mut Transform, (With<Snake>, Without<Food>)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let mut food = food_query.get_single_mut().unwrap();
    let snake = snake_query.get_single_mut().unwrap();
    let window = window_query.get_single().unwrap();

    if food.translation == snake.translation {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..(window.width() / 20.) as i32) as f32;
        let y = rng.gen_range(0..(window.height() / 20.) as i32) as f32;
        food.translation.x = x * 20.;
        food.translation.y = y * 20.;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 255., 0.),
                    custom_size: Some(Vec2::new(20., 20.)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_xyz(
                    snake.translation.x,
                    snake.translation.y,
                    0.
                ),
                ..default()
            },
            SnakeSegment {},
        ));
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<&mut Direction, With<Snake>>,
) {
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
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            bevy_framepace::FramepacePlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_snake, spawn_food, set_framerate))
        .add_systems(Update, (move_snake, handle_input, handle_food_eaten))
        .run();
}
