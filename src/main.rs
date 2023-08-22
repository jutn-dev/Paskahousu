mod card;
mod game;
mod player;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ok".into(),
                        resolution: (1280.0, 720.0).into(),
                        //resolution: (1920.0, 1080.0).into(),
                        //resolution: (3840.0,2160.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();

    println!("ok");
    let mut game1 = game::Game::new();
    game1.game_loop();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas: ResMut<Assets<TextureAtlas>>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 2160.0,
        max_height: 3840.0,
    }; 
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 1080.0,
        max_height: 1940.0,
    };
    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);



    commands.spawn(camera);
    
    let background_texture = asset_server.load("background.png");

    commands.spawn(SpriteBundle {
        texture: background_texture,
        ..default()
    });


    let cards_texture = asset_server.load("cards.png");
//       commands.spawn(SpriteSheetBundle {
//        texture_atlas: texture_atlas.add(TextureAtlas::from_grid(cards_texture, Vec2::new(106.0, 156.0),13,5,None,None)),
//        sprite: TextureAtlasSprite {index: 1, ..default()},
//        transform: Transform{translation: Vec3::new(0.0,300.0,0.0),scale: Vec3::new(1.0,1.0,1.0),..default()},
//        ..default()
//    });
    commands.spawn(AtlasImageBundle{
        texture_atlas: texture_atlas.add(TextureAtlas::from_grid(cards_texture, Vec2::new(106.0,156.0), 13, 5, None, None)),
        style: Style {
            align_items: AlignItems::End,
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        ..default()
    });

}
