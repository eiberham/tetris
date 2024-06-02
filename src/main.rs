extern crate dotenv;
use dotenv::dotenv;

mod states;
use crate::states::{
  start::StartScene, play::PlayScene, over::OverScene
};

mod scene;
use crate::scene::{Scene, SceneStack};

mod factory;
mod block;
mod utils;
mod square;
mod board;
mod record;
mod config;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
    event::EventHandler,
    input::keyboard::KeyInput,
    Context, GameResult
};

use std::path;

pub struct MainState {
	pub stack: SceneStack<ggez::Context>,
}

impl MainState {
  pub fn new(ctx: &mut Context) -> Self {
    let mut stack: SceneStack<Context> = SceneStack::new(ctx);
    
    let scenes: Vec<Box<dyn Scene<Context>>> = vec![
        Box::new(OverScene::new(ctx)),
        Box::new(PlayScene::new(ctx)),
        Box::new(StartScene::new(ctx)),
    ];

    for scene in scenes {
        stack.push(scene);
    }
    
    Self { stack }
  }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      while ctx.time.check_update_time(8) {
        self.stack.update(ctx);
      }

      Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.stack.draw(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
      self.stack.input(ctx, input); 
      Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (mut ctx, event_loop) = ContextBuilder::new("tetris", "eiberham")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(WindowSetup::default().title("tetris"))
        .window_mode(WindowMode::default().dimensions(384.0, 960.0))
        .build()
        .expect("upsss, could not create ggez context!");

    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

