use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH, PADDLE_HEIGHT};

#[derive(SystemDesc)]
// SystemDesc specifies the logic to instantiate systems in the game's world
// Since this system does not require any special logic, we can derive its SystemDesc
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
	
	// Defining the data this system operates on
	type SystemData = (
		WriteStorage<'s, Transform>, 			// "This system mutates Transform components" 
		ReadStorage<'s, Paddle>, 				// "This system reads Paddle components"
		// Note that the parameter type here should be the same as the one used to create InputBundle in main.rs
		Read<'s, InputHandler<StringBindings>>, // 'This system accesses InputHandle<...> components

		);

	fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
		// Loops over all paddles with their respective transform
		// The join here returns all entities with both a Paddle and Transform attached to them, the paddle is immutable while the transform is mutable
		for(paddle, transform) in (&paddles, &mut transforms).join() {
			
			// Gets the movement amount for every paddle
			// For left, it checks <W> and <S>
			// For right, it checks <Up> and <Down>
			let movement = match paddle.side {
				Side::Left => input.axis_value("left_paddle"),
				Side::Right => input.axis_value("right_paddle")
			};
			
			// Moves the paddles and clamps the y position so that they don't go off screen
			if let Some(mv_amount) = movement {
				let scaled_amount = 1.2 * mv_amount as f32;
				let paddle_y = transform.translation().y;
				transform.set_translation_y(
					(paddle_y + scaled_amount)
						.min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
						.max(PADDLE_HEIGHT * 0.5),
				);
			}
		}
	}
}
