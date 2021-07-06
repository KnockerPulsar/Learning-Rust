use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::transform::Transform,
	ecs::{Component, DenseVecStorage},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};

// Constants
pub const ARENA_HEIGHT: 	f32 = 100.0;
pub const ARENA_WIDTH: 		f32 = 100.0;
pub const PADDLE_HEIGHT: 	f32 = 16.0;
pub const PADDLE_WIDTH: 	f32 = 4.0;
pub const BALL_VELOCITY_X: 	f32 = 75.0;
pub const BALL_VELOCITY_Y: 	f32 = 50.0;
pub const BALL_RADIUS: 		f32 = 2.0;

// Structs and enums 
#[derive(PartialEq,Eq)]
pub enum Side {
	Left,
	Right
}

pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32
}

pub struct Ball {
	pub velocity: [f32; 2],
	pub radius: f32,
}

pub struct Pong;

// Amethyst Component implementations
impl Component for Ball {
	type Storage = DenseVecStorage<Self>;
}

impl Component for Paddle {
	type Storage = DenseVecStorage<Self>;
}

// Methods
impl SimpleState for Pong {
	// Whoa....
	fn on_start(&mut self, data: StateData<'_, GameData<'_,'_>>) {
		let world = data.world;
		let sprite_sheet_handle = load_sprite_sheet(world);
		
		// Have to register the ball since no systems use it yet
		world.register::<Ball>();
		
		// Note the use of sprite_sheet_handle.clone()
		// This is because both init functions consume the handle
		// If we pass the handle to the ball init and the paddle init
		// We'll probably get an error since the handle was consume for the ball
		initialise_ball(world, sprite_sheet_handle.clone());
		initialise_paddles(world, sprite_sheet_handle);
		initialise_camera(world);
	}
}

// Adds an entity to carry our camera
// It's a 2D camera with an orthographic projection the size of our arena
// The entity also has a Transform component to represent its worldspace position
fn initialise_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.5);
	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.with(transform)
		.build();
}

// Paddle "constructor"
impl Paddle {
	fn new(side: Side) -> Paddle {
		Paddle { 
			side,
			width: PADDLE_WIDTH,
			height: PADDLE_HEIGHT
		}
	}
}

fn initialise_ball( world: &mut World, sprite_sheet_handle: Handle<SpriteSheet> ) {
	let mut local_transform = Transform::default();
	
	//	Sets the ball's position to be in the middle of the screen
	local_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, 0.0);
	
	// Creates a sprite renderer with the second sprite in our sprite sheet
	let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

	world
		.create_entity()
		.with(sprite_render)
		.with( Ball { 
			radius: BALL_RADIUS,
			velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
		})
		.with(local_transform)
		.build();
}	

fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

	let sprite_render = SpriteRender::new(sprite_sheet_handle,0);
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();

	let y = ARENA_HEIGHT / 2.0;
	left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
	right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

	world
		.create_entity()
		.with(sprite_render.clone())
		.with(Paddle::new(Side::Left))
		.with(left_transform)
		.build();

	world
		.create_entity()
		.with(sprite_render)
		.with(Paddle::new(Side::Right))
		.with(right_transform)
		.build();
}

// We create a texture loader and allocate some storage in texture_storage
// Then we load the texture data into the storage, the function returns
// a handle to the loaded texture
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();

		loader.load(
			"texture/pong_spritesheet.png",
			ImageFormat::default(),
			(),
			&texture_storage
		)
	};

	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	let loader = world.read_resource::<Loader>();
	loader.load(
		"texture/pong_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_sheet_store
	)
}


