use bevy::prelude::{Plugin as BevyPlugin, *};

mod types;

pub(super) struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_materials.system())
            .add_startup_stage(
                "rummikub_setup",
                SystemStage::single_threaded().with_system(setup_tiles.system()),
            )
            .add_system(layout_tiles.system());
    }
}

fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
) {
    commands.insert_resource(types::Materials {
        tile_material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
    });
    commands.insert_resource(types::UiFont(server.load("OpenSans-Regular.ttf")))
}

fn setup_tiles(mut commands: Commands, materials: Res<types::Materials>, font: Res<types::UiFont>) {
    // Generate number tiles
    let mut possible_tiles: Vec<Entity> = (0..(4 * 13 * 2))
        .map(|val| {
            commands
                .spawn_bundle(types::TileBundle::new(
                    types::TileColor::from_u8((val % 4) + 1),
                    types::TileNumber((val % 13) + 1),
                    &materials,
                    &font,
                ))
                .insert(Visible {
                    is_visible: false,
                    is_transparent: false,
                })
                .id()
        })
        .collect::<Vec<Entity>>();

    // Add jokers
    possible_tiles.push(
        commands
            .spawn_bundle(types::TileJokerBundle::new(
                types::TileColor::Black,
                &materials,
                &font,
            ))
            .id(),
    );
    possible_tiles.push(
        commands
            .spawn_bundle(types::TileJokerBundle::new(
                types::TileColor::Red,
                &materials,
                &font,
            ))
            .id(),
    );

    // Randomly pick player hand tiles
    let mut player_hand_vec: Vec<Entity> = Vec::new();
    for _ in 1..=14 {
        let index = (rand::random::<f32>() * possible_tiles.len() as f32).floor() as usize;
        let chosen_tile = possible_tiles.remove(index);
        commands.entity(chosen_tile).insert(types::TileInHand {});
        player_hand_vec.push(chosen_tile);
    }
    commands.spawn().insert(types::PlayerHand(player_hand_vec));
}

fn layout_tiles(mut tiles: Query<(&mut Transform, &mut Visible), With<types::TileInHand>>) {
    for (i, (mut transform, mut visible)) in tiles.iter_mut().enumerate() {
        transform.translation.x = (70 * i) as f32 - 460.0;
        visible.is_visible = true;
    }
}
