use ggez::{
  graphics::{Canvas, Color, DrawParam, FontData, Image, Rect},
  input::keyboard::{KeyCode, KeyInput},
  Context
};
use crate::scene::{Scene, SceneSwitch};
use crate::utils::DrawableText;

pub struct OverScene {
  done: bool
}

impl OverScene {
  pub fn new(_ctx: &mut Context) -> Self {
      Self { done: false }
    }
}

impl DrawableText for OverScene {}

impl<Ev> Scene<Ev> for OverScene {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    if self.done {
      ctx.request_quit(); 
    }
    SceneSwitch::None::<Ev>
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
    let background = Image::from_path(ctx, "/over.png").unwrap();
    
    self.draw_text(&mut canvas, "You Lose!", [64., 430.], 32., Color::RED);
    self.draw_text(&mut canvas, "Press Enter To Exit", [32., 550.], 18., Color::WHITE);
    
    // Draw the top image
    canvas.draw(
        &background,
        DrawParam::new()
          .src(Rect::new(0., 0., 1., 1.))
          .scale([0.8, 0.8])
          .dest([32., 32.])
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
    "over"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
    false
  }
}

