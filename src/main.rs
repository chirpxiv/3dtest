// Dependencies

mod structs;
mod controls;
mod render;

// Main

pub fn main() {
	render::Renderer::new().run();
}