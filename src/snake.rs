use bevy::{prelude::*, time::FixedTimestep};

use crate::utils::{Position, Size};

#[derive(Reflect, Default, Component, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Reflect, Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Right,
    Left,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SnakeHead>()
            .add_startup_system(snake_setup)
            .add_system(get_movement.before(snake_movement))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.200))
                    .with_system(snake_movement),
            );
    }
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

fn snake_setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::SEA_GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead {
            direction: Direction::default(),
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8))
        .insert(Name::new("SnakeHaed"));
}

fn get_movement(mut snake_head: Query<&mut SnakeHead>, input: Res<Input<KeyCode>>) {
    let mut snake_head = snake_head.single_mut();

    let dir: Direction = if input.pressed(KeyCode::W) {
        Direction::Up
    } else if input.pressed(KeyCode::S) {
        Direction::Down
    } else if input.pressed(KeyCode::A) {
        Direction::Left
    } else if input.pressed(KeyCode::D) {
        Direction::Right
    } else {
        snake_head.direction
    };

    if dir != snake_head.direction.opposite() {
        snake_head.direction = dir;
    }
}

fn snake_movement(mut snake: Query<(&mut SnakeHead, &mut Position)>) {
    for (snake_head, mut pos) in snake.iter_mut() {
        match snake_head.direction {
            Direction::Up => pos.y += 1,
            Direction::Down => pos.y -= 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
    }
}
