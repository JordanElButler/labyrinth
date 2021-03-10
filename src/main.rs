pub mod input;
pub mod timer;
pub mod shader;
pub mod math;
pub mod camera;
pub mod transform;
pub mod mesh;
pub mod loader;
pub mod texture;
pub mod render_object;
pub mod vertex;
pub mod gl_util;
pub mod geo;
pub mod resources;
pub mod load_assets;
pub mod light;
pub mod framebuffer;
pub mod material;
pub mod renderer;
pub mod game;
pub mod managers;
pub mod game_objects;
pub mod boundaries;

pub fn main() {
    let mut game_app = game::GameApp::new();
    game_app.start();
}