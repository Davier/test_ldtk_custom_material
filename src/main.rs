use bevy::{
    asset::AssetMetaCheck, prelude::*, render::mesh::shape::Quad, window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(800., 600.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        // The Material2d version seems to work on Firefox but not Chromium (where even the map doesn't render most of the time)
        .register_ldtk_entity::<Material2dEntity>("MyEntityIdentifier")
        // The sprite sheet version works
        // .register_ldtk_entity::<SpriteSheetEntity>("MyEntityIdentifier")
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    materials.get_or_insert_with(ENTITY_MATERIAL, || ColorMaterial {
        color: Color::YELLOW,
        ..default()
    });
    meshes.get_or_insert_with(ENTITY_MESH, || Quad::new(Vec2::new(32., 32.)).into());

    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = 128.;
    camera.transform.translation.y = 128.;
    camera.projection.scale = 0.5;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

const ENTITY_MATERIAL: Handle<ColorMaterial> = Handle::weak_from_u128(4144467460296151171);
const ENTITY_MESH: Handle<Mesh> = Handle::weak_from_u128(11423988422760235427);

#[derive(Bundle, LdtkEntity)]
struct Material2dEntity {
    material: ColorMesh2dBundle,
}

impl Default for Material2dEntity {
    fn default() -> Self {
        Self {
            material: ColorMesh2dBundle {
                mesh: ENTITY_MESH.into(),
                material: ENTITY_MATERIAL.clone(),
                ..default()
            },
        }
    }
}

#[derive(Bundle, LdtkEntity)]
struct SpriteSheetEntity {
    #[sprite_sheet_bundle]
    sprite: SpriteSheetBundle,
}
