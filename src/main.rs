use bevy::prelude::*;

const CELL_SIZE: f32 = 20.0; // the grid cell size; also the snake size
const BOX_WIDTH: u32 = 30; // width of the game box in cells
const BOX_HEIGHT: u32 = 20; // height of the game box in cells
const BORDER_WIDTH: f32 = 10.0;
const GAME_SPEED: f64 = 10.0;

fn main() {
    println!("Firing up..."); // hello comment

    let mut app = App::new(); // create new app
    app.add_plugins(DefaultPlugins); // add default plugins
    app.add_message::<SnakeDirectionChanged>();
    app.insert_resource(Time::<Fixed>::from_hz(GAME_SPEED));
    app.add_systems(Startup, (setup, snake_startup, spawn_snake)); // add systems
    app.add_systems(Update, (listen_for_direction_input, update_snake_direction));
    app.add_systems(FixedUpdate, process_snake_movement);
    app.run(); // run the app
}

///
fn update_snake_direction(
    mut snake_direction_events: MessageReader<SnakeDirectionChanged>,
    mut snake_query: Query<&mut SnakeDirection, With<Snake>>,
) {
    for msg in snake_direction_events.read() {
        for mut snake_direction in snake_query.iter_mut() {
            snake_direction.0 = msg.direction;
        }
    }
}

/// General setup
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); // spawn camera
}

/// General setup for the snake game
fn snake_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // render game bounds
    let box_width = BOX_WIDTH as f32 * CELL_SIZE;
    let box_height = BOX_HEIGHT as f32 * CELL_SIZE;
    let margin = BORDER_WIDTH;
    let inner_box = meshes.add(Rectangle::new(box_width, box_height));
    let inner_box_colour = Color::hsl(0.0, 0.0, 0.5);

    let outer_box = meshes.add(Rectangle::new(
        box_width + 2.0 * margin,
        box_height + 2.0 * margin,
    ));
    let outer_box_colour = Color::hsl(180.0, 0.5, 0.5);

    commands.spawn((
        Mesh2d(inner_box),
        MeshMaterial2d(materials.add(inner_box_colour)),
        Transform::from_xyz(-CELL_SIZE / 2.0, -CELL_SIZE / 2.0, 1.0), // higher z-index so spawns on top
    ));
    commands.spawn((
        Mesh2d(outer_box),
        MeshMaterial2d(materials.add(outer_box_colour)),
        Transform::from_xyz(-CELL_SIZE / 2.0, -CELL_SIZE / 2.0, 0.0),
    ));
}

/// Send a message to change the snake movement direction based on user input
fn listen_for_direction_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_direction_events: MessageWriter<SnakeDirectionChanged>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        info!("Updating snake direction to: Up");
        snake_direction_events.write(SnakeDirectionChanged {
            direction: Direction::Up,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        info!("Updating snake direction to: Down");
        snake_direction_events.write(SnakeDirectionChanged {
            direction: Direction::Down,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        info!("Updating snake direction to: Left");
        snake_direction_events.write(SnakeDirectionChanged {
            direction: Direction::Left,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        info!("Updating snake direction to: Right");
        snake_direction_events.write(SnakeDirectionChanged {
            direction: Direction::Right,
        });
    }
}

#[derive(Message)]
pub struct SnakeDirectionChanged {
    /// The new direction
    pub direction: Direction,
}

/// Move the snake head based on the snake direction
fn process_snake_movement(mut snake_query: Query<(&mut Transform, &SnakeDirection), With<Snake>>) {
    for (mut transform, direction) in &mut snake_query {
        match direction.0 {
            Direction::Up => transform.translation.y += CELL_SIZE,
            Direction::Down => transform.translation.y -= CELL_SIZE,
            Direction::Left => transform.translation.x -= CELL_SIZE,
            Direction::Right => transform.translation.x += CELL_SIZE,
        }
        info!("Snake transform is now: {:?}", transform);
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snake_head = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let snake_head_colour = Color::hsl(90.0, 0.5, 0.5);

    commands.spawn((
        Snake,
        Mesh2d(snake_head),
        MeshMaterial2d(materials.add(snake_head_colour)),
        Transform::from_xyz(0.0, 0.0, 2.0), // higher z-index so spawns on to
        SnakeDirection(Direction::Left),
    ));
}

/// Snake component
#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SnakeDirection(Direction);

/// Movement Direction component
#[derive(Component, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
