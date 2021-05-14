use bevy::prelude::{Plugin as BevyPlugin, *};

mod types;

pub(super) struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_materials.system())
            .add_startup_stage(
                "rummikub_setup",
                SystemStage::single_threaded().with_system(setup_tiles.system()),
            );
    }
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(types::Materials {
        tile_material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
    });
}

fn setup_tiles(mut commands: Commands) {
    let mut possible_tiles: Vec<Entity> = (0..(4 * 13))
        .map(|val| {
            commands
                .spawn()
                .insert(types::Tile)
                .insert(types::TileColor::from_u8((val % 4) + 1))
                .insert(types::TileNumber((val % 13) + 1))
                .id()
        })
        .collect();
    let mut player_hand_vec: Vec<Entity> = Vec::new();
    for _ in 1..14 {
        let index = (rand::random::<f32>() * possible_tiles.len() as f32).floor() as usize;
        let chosen_tile = possible_tiles.remove(index);
        // TODO: do we want this?
        // commands.entity(chosen_tile).insert(types::InPlayerHand {})
        player_hand_vec.push(chosen_tile);
    }
    commands.spawn().insert(types::PlayerHand(player_hand_vec));
}
