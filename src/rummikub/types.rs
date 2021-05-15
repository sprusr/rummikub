use std::collections::{BTreeSet, HashSet};

use bevy::{prelude::*, text::Text2dSize};

pub(super) struct UiFont(pub(super) Handle<Font>);

pub(super) struct Materials {
    pub(super) tile_material: Handle<ColorMaterial>,
}

pub(super) struct Tile;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub(super) enum TileColor {
    Black,
    Blue,
    Red,
    Yellow,
}

impl TileColor {
    pub(super) fn from_u8(value: u8) -> Self {
        match value {
            1 => TileColor::Black,
            2 => TileColor::Blue,
            3 => TileColor::Red,
            4 => TileColor::Yellow,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct TileNumber(pub(super) u8);

#[derive(Bundle)]
pub(super) struct TileBundle {
    tile: Tile,
    color: TileColor,
    number: TileNumber,
    #[bundle]
    sprite: SpriteBundle,
    text: Text,
    text_size: Text2dSize,
}

impl TileBundle {
    pub(super) fn new(
        color: TileColor,
        number: TileNumber,
        materials: &Res<Materials>,
        font: &Res<UiFont>,
    ) -> Self {
        TileBundle {
            tile: Tile {},
            color,
            number,
            sprite: SpriteBundle {
                material: materials.tile_material.clone(),
                sprite: Sprite::new(Vec2::new(50.0, 50.0)),
                transform: Transform {
                    translation: Vec3::new(10.0, 10.0, 0.0),
                    ..Default::default()
                },
                visible: Visible {
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            },
            text: Text::with_section(
                number.0.to_string(),
                TextStyle {
                    color: match color {
                        TileColor::Black => Color::BLACK,
                        TileColor::Blue => Color::BLUE,
                        TileColor::Red => Color::RED,
                        TileColor::Yellow => Color::YELLOW,
                    },
                    font_size: 24.0,
                    font: font.0.clone(),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            text_size: Text2dSize {
                size: Size {
                    width: 2.0,
                    height: 2.0,
                },
            },
        }
    }
}

pub(super) struct TileJoker;

#[derive(Bundle)]
pub(super) struct TileJokerBundle {
    tile: Tile,
    color: TileColor,
    joker: TileJoker,
    #[bundle]
    sprite: SpriteBundle,
    text: Text,
    text_size: Text2dSize,
}

impl TileJokerBundle {
    pub(super) fn new(color: TileColor, materials: &Res<Materials>, font: &Res<UiFont>) -> Self {
        TileJokerBundle {
            tile: Tile {},
            color,
            joker: TileJoker {},
            sprite: SpriteBundle {
                material: materials.tile_material.clone(),
                sprite: Sprite::new(Vec2::new(50.0, 50.0)),
                transform: Transform {
                    translation: Vec3::new(10.0, 10.0, 0.0),
                    ..Default::default()
                },
                visible: Visible {
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            },
            text: Text::with_section(
                "J".to_string(),
                TextStyle {
                    color: match color {
                        TileColor::Black => Color::BLACK,
                        TileColor::Blue => Color::BLUE,
                        TileColor::Red => Color::RED,
                        TileColor::Yellow => Color::YELLOW,
                    },
                    font_size: 24.0,
                    font: font.0.clone(),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            text_size: Text2dSize::default(),
        }
    }
}

pub(super) struct TileInHand;

pub(super) struct PlayerHand(pub(super) Vec<Entity>);

pub(super) struct TileSet(pub(super) Vec<Entity>);

impl TileSet {
    // TODO: account for jokers
    pub(super) fn is_valid(&self, query: &Query<(&TileColor, &TileNumber), With<Tile>>) -> bool {
        // Must be min 3, max 13 tiles
        if self.0.len() < 3 || self.0.len() > 13 {
            return false;
        }

        let tiles: Vec<(TileColor, u8)> = self
            .0
            .iter()
            .map(|tile| {
                (
                    query.get(*tile).unwrap().0.clone(),
                    query.get(*tile).unwrap().1 .0,
                )
            })
            .collect();

        let (colors, numbers) = tiles.iter().fold(
            (HashSet::<TileColor>::new(), BTreeSet::<u8>::new()),
            |(mut colors, mut numbers), (color, number)| {
                colors.insert(*color);
                numbers.insert(*number);
                (colors, numbers)
            },
        );

        // Same number, different colors
        if numbers.len() == 1 && colors.len() == tiles.len() {
            return true;
        }

        // Same color, consecutive numbers
        if colors.len() == 1 && numbers.len() == tiles.len() {
            let first = numbers.iter().nth(1).unwrap().clone();
            let (consecutive, _) = numbers
                .iter()
                .cloned()
                .fold((true, first), |(acc, prev), num| {
                    (acc == true && num == prev + 1, num)
                });
            return consecutive;
        }

        return false;
    }

    pub(super) fn fmt(
        &self,
        query: &Query<(&TileColor, &TileNumber, &TileJoker), With<Tile>>,
    ) -> String {
        self.0
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (i, tile)| {
                format!(
                    "{}{}{:?} {}",
                    acc,
                    if i == 0 { "" } else { ", " },
                    query.get(*tile).unwrap().0,
                    query.get(*tile).unwrap().1 .0
                )
            })
    }
}
