use bevy::{prelude::*, reflect::TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef,RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::render::mesh::{MeshVertexBufferLayout};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};


// https://bevyengine.org/examples/shader/shader-material/
// https://github.com/bevyengine/bevy/blob/latest/examples/shader/shader_material_glsl.rs
// https://www.youtube.com/watch?v=O6A_nVmpvhc

pub struct ShadersPlugin;


impl Plugin for ShadersPlugin {
  fn build(&self, app: &mut App) {
      app
      .add_plugin(MaterialPlugin::<GlowyMaterial>::default())
      .add_startup_system(init);
  }
}


// initialize shaders
fn init(mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut glowy_materials: ResMut<Assets<GlowyMaterial>>,
        asset_server: Res<AssetServer>){
    println!(" [SHADERS] Initialize Shaders");

    let env_texture = asset_server.load("textures/stone_alley_02_1k.hdr");
    let material = glowy_materials.add(GlowyMaterial { env_texture: Some(env_texture) });

    commands.spawn(MaterialMeshBundle{
      mesh: meshes.add(Mesh::from(shape::UVSphere{radius: 1.0, ..default()})),
      material: material.clone(),
      ..default()
    });




}


#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct GlowyMaterial {
  #[texture(0)]
  #[sampler(1)]
  pub env_texture: Option<Handle<Image>>,
}

// The Material trait has many optional functions for configuration.
// In this case, all we need to set is a fragment shader. Ergonomics!
impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy_material.wgsl".into()
    }
}
