//! Pong tutorial 1+2

mod pong;
mod systems;

use crate::pong::Pong;

use amethyst::{
	core::transform::TransformBundle,
	prelude::*, // Includes all necessary stuff to define a basic rendering pipeline
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
	// Start logger
	amethyst::start_logger(Default::default());

	// Set up display config
	let app_root = application_root_dir()?;

	let display_config_path = app_root.join("config").join("display.ron");
	let assets_dir = app_root.join("assets");

	
	use amethyst::input::{InputBundle, StringBindings};
	let binding_path = app_root.join("config").join("bindings.ron");

	// To handle inputs 
	let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

	// Setting up the application
	let game_data = GameDataBuilder::default()
		.with_bundle(
			// Add the rendering bundle to enable the render functionality
			RenderingBundle::<DefaultBackend>::new()
				// Add the RenderToWindow plugin to RenderingBundle
				// To enable rendering to a window
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						.with_clear([0.0, 0.0, 0.0, 1.0]),
				)
				// Add the RenderFlat2D plugin to enable drawing 2D sprites
				.with_plugin(RenderFlat2D::default()),
		)?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?  

		// Adding our paddle movement system
		// The first parameter is an instance of the system
		// The second is the name of the system
		// The third is a list of dependencies for the system, theses must run before it
		.with(systems::PaddleSystem, "paddle_system", &["input_system"]);

	let mut game = Application::new(assets_dir, Pong, game_data)?;
	// Show time
	game.run();

	Ok(())
} 
