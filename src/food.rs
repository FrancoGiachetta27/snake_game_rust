use crate::{
    snake::SnakeHead,
    utils::{Position, Size},
    ARENA_HEIGHT, ARENA_WIDTH,
};
use bevy::prelude::*;
use rand::random;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Food;

pub struct FoodEatendEvent;

impl Plugin for Food {
    fn build(&self, app: &mut App) {
        app.register_type::<Food>()
            .add_event::<FoodEatendEvent>()
            .add_startup_system(spawn_food)
            .add_system(food_eaten);
    }
}

fn spawn_food(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8))
        .insert(Name::new("Food"));
}

// checks wether position of the snake is equal to the food's, despawns that entity and sends the
// event
fn food_eaten(
    mut commands: Commands,
    snake: Query<&Position, With<SnakeHead>>,
    food: Query<(Entity, &Position), With<Food>>,
    mut food_eaten_event: EventWriter<FoodEatendEvent>,
) {
    let snake_position = snake.single();

    for (food, pos) in &food {
        if snake_position == pos {
            food_eaten_event.send(FoodEatendEvent);
            commands.entity(food).despawn_recursive();
        }
    }
}
