use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use rand::prelude::random;
use std::time::Duration;

const ARENA_WIDTH: u32 = 120;
const ARENA_HEIGHT: u32 = 120;
const SNAKE_HEAD_COLOR: Color = Color::srgb(0., 0., 0.);
const SNAKE_SEGMENT_COLOR: Color = Color::srgba(0., 0., 0., 0.6);
const FOOD_COLOR: Color = Color::srgb(0., 0., 0.);

#[derive(Component, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component)]
struct Food;

#[derive(Event)]
struct GrowthEvent;

#[derive(Resource, Default)]
struct LastTailPosition(Option<Position>);

#[derive(Event)]
struct GameOverEvent;

#[derive(Component)]
struct Board;

#[derive(Default, Resource)]
struct Score(u32);

#[derive(Event)]
struct AddScoreEvent;

struct Snake;

//
impl Plugin for Snake {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_snake).chain());
        app.add_systems(
            FixedUpdate,
            (
                food_spawner.run_if(on_timer(Duration::from_secs(5))),
                (
                    snake_movement.run_if(on_timer(Duration::from_millis(100))),
                    snake_eating,
                    snake_growth,
                    game_over,
                )
                    .chain(),
            ),
        );

        // update
        app.add_systems(Update, snake_movement_input);
        app.add_systems(PostUpdate, (position_translation, size_scaling).chain());

        // camera
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                resolution: (500., 500.).into(),
                ..default()
            }),
            ..default()
        }));

        //
        app.insert_resource(ClearColor(Color::srgb(1., 1., 1.))); // background color
        app.insert_resource(LastTailPosition::default());
        app.insert_resource(SnakeSegments::default());
        app.add_event::<GrowthEvent>();
        app.add_event::<GameOverEvent>();
    }
}

// camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// snake
fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 2 })
            .insert(Size::square(0.6))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

// segment
fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.6))
        .id()
}

// food
fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.6));
}

// scaling
fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    if let Ok(window) = windows.get_single() {
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }
}

// position
fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }
    if let Ok(window) = windows.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0,
            )
        }
    }
}

// moviment
fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA)
            || keyboard_input.pressed(KeyCode::Numpad4)
            || keyboard_input.pressed(KeyCode::KeyJ)
        {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::KeyS)
            || keyboard_input.pressed(KeyCode::Numpad5)
            || keyboard_input.pressed(KeyCode::KeyK)
        {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::KeyW)
            || keyboard_input.pressed(KeyCode::Numpad8)
            || keyboard_input.pressed(KeyCode::KeyI)
        {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::KeyD)
            || keyboard_input.pressed(KeyCode::Numpad6)
            || keyboard_input.pressed(KeyCode::KeyL)
        {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir; // Atualiza a direção da cabeça
        }
    }
}

// snake moviment
fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
    mut game_over_write: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
            Direction::Up => head_pos.y += 1,
            Direction::Down => head_pos.y -= 1,
        }

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.x as u32 >= ARENA_HEIGHT
        {
            game_over_write.send(GameOverEvent);
        }

        if segment_positions.contains(&head_pos) {
            game_over_write.send(GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| *positions.get_mut(*segment).unwrap() = *pos);
    }
}

//
fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

//
fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    mut score_write: EventWriter<AddScoreEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (food_entity, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(food_entity).despawn();
                growth_writer.send(GrowthEvent);
                score_write.send(AddScoreEvent);
            }
        }
    }
}

// GameOver
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    //
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<Board>>,
    //
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        for mut text in query.iter_mut() {
            score.reset();
            text.sections[1].value = format!("{}", score.get());
        }

        spawn_snake(commands, segments_res);
    }
}

struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>();
        app.insert_resource(Score::default())
            .add_event::<AddScoreEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, update_board);
    }
}

impl Score {
    fn reset(&mut self) {
        self.0 = 0;
    }

    fn increment(&mut self) {
        self.0 += 1;
    }

    fn get(&self) -> u32 {
        self.0
    }
}

fn setup_board(mut commands: Commands) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0., 0., 0.),
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0., 0., 0.),
                        ..default()
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(Board);
}

fn update_board(
    mut score: ResMut<Score>,
    mut reader: EventReader<AddScoreEvent>,
    mut query: Query<&mut Text, With<Board>>,
) {
    if reader.read().next().is_some() {
        score.increment();
    }

    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", score.get());
    }
}

fn main() {
    App::new().add_plugins(Snake).add_plugins(BoardPlugin).run();
}
