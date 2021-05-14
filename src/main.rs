use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod rummikub;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "rummikub".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(rummikub::Plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
