use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::{prelude::*, time::FixedTimestep};
use iyes_loopless::prelude::IntoConditionalSystem;
use rand::random;

use crate::{
    food::FoodEatendEvent,
    utils::{Position, Size},
};

#[derive(Reflect, Default, Component, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Reflect, Default, Component, PartialEq)]
#[reflect(Component)]
pub struct SnakeBodyPart;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct Snake(Vec<Entity>);

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
            .insert_resource(Snake::default())
            .add_startup_system(snake_setup)
            .add_system(get_movement.before(head_movement))
            .add_system(snake_growth.run_on_event::<FoodEatendEvent>())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.200))
                    .with_system(head_movement),
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

fn snake_setup(mut commands: Commands, mut snake: ResMut<Snake>) {
    *snake = Snake(vec![
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
            .insert(Name::new("SnakeHead"))
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::SEA_GREEN,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeBodyPart)
            .insert(Position { x: 3, y: 2 })
            .insert(Size::square(0.8))
            .insert(Name::new("SnakeBodyPart (1)"))
            .id(),
    ]);
}

//changes the direction of the snake if the player presses the respective key
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

    // if the snake cannot move in the opposite direction
    if dir != snake_head.direction.opposite() {
        snake_head.direction = dir;
    }
}

fn snake_movement(
    snake: ResMut<Snake>,
    mut positions: Query<&mut Position>,
    mut head: Query<(Entity, &SnakeHead)>,
) {
    for (head_ent, head) in head.iter_mut() {
        let segments_positions = snake
            .iter()
            .map(|seg| *positions.get_mut(*seg).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_ent).unwrap();

        match head.direction {
            Direction::Up => head_pos.y += 1,
            Direction::Down => head_pos.y -= 1,
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
        }

        // for every snake part, except the head, chages its position to the
        // one's which is in front, so as to flow it
        segments_positions
            .iter()
            .zip(snake.iter().skip(1))
            .for_each(|(pos, seg)| *positions.get_mut(*seg).unwrap() = *pos);
    }
}

// spawn a snake segment when the food is eaten
fn snake_growth(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<&Position>,
    mut food_eaten_event: EventReader<FoodEatendEvent>,
) {
    let tail_pos = *positions.get(snake[0]).unwrap();

    snake.push(
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::SEA_GREEN,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeBodyPart)
            .insert(Position {
                x: tail_pos.x,
                y: tail_pos.y,
            })
            .insert(Size::square(0.8))
            .insert(Name::new("SnakeBodyPart (1)"))
            .id(),
    );
}
