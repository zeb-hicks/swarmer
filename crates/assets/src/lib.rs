use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct AssetPlugin {
    asset_server: AssetServer,
    asset_handles: HashMap<String, UntypedHandle>,
}

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let asset_handles: HashMap<String, UntypedHandle> = HashMap::new();
    let asset_plugin = AssetPlugin {
        asset_server: asset_server.clone(),
        asset_handles,
    };
    commands.insert_resource(asset_plugin);
}

impl AssetPlugin {
    pub fn get_asset_by_path(&self, asset_path: String) -> UntypedHandle {
        let extension = asset_path.split('.').last().unwrap();
        match extension {
            "png" => {
                // Return existing asset if we have it else fetch and return it.
                match self.asset_handles.contains_key(&asset_path) {
                    true => self.asset_handles.get(&asset_path).unwrap().clone(),
                    false => self.asset_server.load::<Image>(asset_path).untyped(),
                }
            }
            _ => {
                panic!("Unsupported asset type: {}", extension);
            }
        }
    }
}

pub fn sprite_sheet_bundle(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    transform: Transform,
    sprite_index: usize,
) -> SpriteSheetBundle {
    let spritesheet = asset_server.load("tilemap_packed.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 12, 11, None, None);
    let atlas_layout = atlas_layouts.add(texture_atlas_layout);
    
    SpriteSheetBundle {
        texture: spritesheet,
        atlas: TextureAtlas {
            layout: atlas_layout,
            index: sprite_index,
        },
        transform,
        ..default()
    }
}
