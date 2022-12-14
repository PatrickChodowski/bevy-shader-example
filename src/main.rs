use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::PresentMode;
use bevy::window::WindowPlugin;


mod camera;
mod shaders;

// use camera::CameraPlugin;
use shaders::ShadersPlugin;

mod camera2;

use camera2::{CameraControllerPlugin};

pub const HEIGHT: f32 = 900.0;
pub const STARTING_SCENE: u8 = 1;
pub const RESOLUTION: f32 = 16.0 / 9.0;


// Main loop
fn main() {



  App::new()
      .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
          width: HEIGHT * RESOLUTION,
          height: HEIGHT,
          title: "Bevy Shaders".to_string(),
          present_mode: PresentMode::AutoVsync,
          resizable: true,
          mode: WindowMode::Windowed,
          ..default()
      }, ..default()}
      ))
      .add_plugin(CameraControllerPlugin)
      // .add_plugin(CameraPlugin)
      .add_plugin(ShadersPlugin)
      .run();
}

