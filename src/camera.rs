
use bevy::prelude::*;


pub struct CameraPlugin;


impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
      app.add_startup_system(spawn_camera_3d);
        //  .add_system_set(move_camera)
  }
}

#[derive(Component)]
pub struct MainCamera;


fn spawn_camera_2d(mut commands: Commands) {
    let camera = Camera2dBundle{
      transform: Transform::from_scale(Vec3{x: 1.0, y: 1.0, z: 1.0 })
      .with_translation(Vec3{x: 0.0, y:0.0, z:999.9}),
      ..default()
    };
    commands.spawn(camera).insert(MainCamera);
}


fn spawn_camera_3d(mut commands: Commands) {
  let camera = Camera3dBundle{
    transform: Transform::from_xyz(5.0, 2.0, 5.0 )
                         .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ..default()
  };
  commands.spawn(camera).insert(MainCamera);
}



// Move camera according to the view resource
// fn move_camera(view: Res<View>, mut camera_query: Query<&mut Transform, With<MainCamera>>){
//   let mut transform = camera_query.single_mut();
//   transform.scale = Vec3{x: view.scale, y: view.scale, z:1.0};
//   let new_x: f32 = transform.translation.x + view.move_x;
//   let new_y: f32 = transform.translation.y + view.move_y;
//   transform.translation = Vec3{x: new_x, 
//                                y: new_y, 
//                                z: transform.translation.z};
// }

// Resets camera to original position
// pub fn reset_camera(mut view: ResMut<View>, mut camera_query: Query<&mut Transform, With<MainCamera>>){
//   let mut transform = camera_query.single_mut();
//   transform.scale = Vec3{x: 1.0, y: 1.0, z:1.0};
//   transform.translation = Vec3{x: 0.0, y: 0.0, z: transform.translation.z};
//   view.move_x = 0.0;
//   view.move_y = 0.0;
//   view.scale = 1.0;
// }
