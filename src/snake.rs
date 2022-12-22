use bevy::{prelude::*, transform};

#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct SnakeHead;

pub struct SnakePlugin;

const SNAKE_SIZE: f32 = 25.0;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SnakeHead>()
            .add_startup_system(snake_setup)
            .add_system(snake_movement);
    }
}

fn snake_setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::SEA_GREEN,
                custom_size: Some(Vec2::new(SNAKE_SIZE, SNAKE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.5, 0.5, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Name::new("SnakeHaed"));
}

fn snake_movement(mut snake: Query<&mut Transform, With<SnakeHead>>, input: Res<Input<KeyCode>>) {
    let mut snake_head = snake.single_mut();

    if input.just_pressed(KeyCode::W) {
        snake_head.translation.y += SNAKE_SIZE;
    }
    if input.just_pressed(KeyCode::S) {
        snake_head.translation.y -= SNAKE_SIZE;
    }
    if input.just_pressed(KeyCode::A) {
        snake_head.translation.x -= SNAKE_SIZE;
    }
    if input.just_pressed(KeyCode::D) {
        snake_head.translation.x += SNAKE_SIZE;
    }
}
