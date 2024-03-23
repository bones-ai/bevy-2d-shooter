use bevy::prelude::*;
use bevy::window::close_on_esc;

use hell_game::animation::AnimationPlugin;
use hell_game::camera::FollowCameraPlugin;
use hell_game::collision::CollisionPlugin;
use hell_game::enemy::EnemyPlugin;
use hell_game::gui::GuiPlugin;
use hell_game::gun::GunPlugin;
use hell_game::player::PlayerPlugin;
use hell_game::state::GameState;
use hell_game::world::WorldPlugin;
use hell_game::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Update, close_on_esc)
        .run();
}
