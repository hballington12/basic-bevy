use bevy::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut app = App::new(); // create new app

    app.add_plugins(DefaultPlugins); // add default plugins

    app.add_systems(Startup, setup);

    app.run(); // run the app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(50.0));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::hsl(180.0, 1.0, 0.5))),
    ));
}
