use ggez::{
  graphics::{Canvas, Color, DrawParam, FontData, Image, Rect},
  input::keyboard::{KeyCode, KeyInput},
  Context
};

use crate::scene::{Scene, SceneSwitch};
use crate::utils::*;

pub struct StartScene {
  done: bool
}

impl StartScene {
  pub fn new(_ctx: &mut Context) -> Self {
    Self { done: false }
  }
}

impl DrawableText for StartScene {}

impl<Ev> Scene<Ev> for StartScene {
  fn update(&mut self, _ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    if self.done {
        SceneSwitch::Pop::<Ev>
    } else {
        SceneSwitch::None::<Ev>
    }
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    let background = Image::from_path(ctx, "/logo.png").unwrap();
    
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
    
    self.draw_text(&mut canvas, "Tetris", [64., 430.], 40., Color::RED);
    
    self.draw_text(&mut canvas, "Press Enter Key", [64., 580.], 18., Color::WHITE);
    
    // Draw the top image
    canvas.draw(
        &background,
        DrawParam::new()
          .src(Rect::new(0., 0., 1., 1.))
          .dest([0., 0.])
    );
        
    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, _ctx: &mut Context, input: KeyInput) {
    if let Some(KeyCode::Return) = input.keycode {
        self.done = true;
    }
  }
  
  fn name(&self) -> &str {
    "start"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
    false
  }
}

