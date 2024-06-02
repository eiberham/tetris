use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::sync::mpsc;
use tokio::task;
use ggez::{
  audio::{SoundSource, Source},
  graphics::{Canvas, Color, FontData},
  input::keyboard::{KeyCode, KeyInput},
  Context
};


use crate::record::*;
use crate::{
  scene::{Scene, SceneSwitch},
  utils::{Shape, DrawableText, Tetromino}, 
  factory::{Factory, Piece}, 
  block::Block, board::Board
};

pub struct PlayScene {
  block: Option<Block>,
  board: Board,
  score: u16,
  music: Source,
  pause: bool,
  done: bool,
  sender: mpsc::Sender<u16>,
  receiver: mpsc::Receiver<u16>,
  record: u16
}

impl PlayScene {
  pub fn new(ctx: &mut Context) -> Self {
    let mut music = Source::new(ctx, "/music.mp3").unwrap();
    music.set_repeat(true);
    music.set_volume(0.1);
    
    let (sender, receiver) = mpsc::channel(1);
    let shipper = sender.clone();
    task::spawn(async move {
      let record = match Record::fetch().await {
          Ok(record) => record.value,
          Err(e) => panic!("{}", e),
      };
      shipper.send(record).await.unwrap();
    });
    
    Self { 
      block: None,
      board: Board::new(),
      score: 0,
      music,
      pause: false,
      done: false,
      sender,
      receiver,
      record: 0
    }
  }
}

impl DrawableText for PlayScene {}

impl<Ev> Scene<Ev> for PlayScene {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev> {
    while !self.pause {
      if let Ok(record) = self.receiver.try_recv() {
        self.record = record;
      }
      
      if self.music.paused() { self.music.resume(); }
      if self.music.stopped() { 
        self.music.play(ctx).unwrap(); 
      }
  
      if self.block.is_some() {
        let mut block: Block = self.block.unwrap();
        
        if block.has_stacked(self.board) {
          self.music.stop(ctx).unwrap();
          let mut crash = Source::new(ctx, "/crash.wav").unwrap();
          crash.set_volume(0.1);
          crash.play_detached(ctx).unwrap();
          self.done = true;
        }

        // checks if the block has landed
        // checks if there's a collision
        if !block.has_landed() && !block.collides(self.board) {
          block.move_down();
          self.block = Some(block);

          if ctx.keyboard.is_key_pressed(KeyCode::Down) && ctx.keyboard.is_key_repeated() {
            block.drop(self.board);
            self.block = Some(block);
          }
        } else {
          // ocupies position on board
          self.board.fill(block.get_positions(), block.color);
          self.block = None;
        }

        // checks if any row has been filled
        let count = self.board.clear(ctx);
        let score = self.score + count;
        let arc: Arc<Mutex<u16>> = Arc::new(Mutex::new(score));
        let clone: Arc<Mutex<u16>> = Arc::clone(&arc);
        let sender : Arc<Mutex<mpsc::Sender<u16>>> = Arc::new(Mutex::new(self.sender.clone()));
        let shipper: Arc<Mutex<mpsc::Sender<u16>>> = Arc::clone(&sender);
        if score > self.record {
          self.record = score;
          task::spawn(async move {
            let score: MutexGuard<u16> = clone.lock().await;
            let sender: MutexGuard<mpsc::Sender<u16>> = shipper.lock().await;
            let record = match Record::save(*score).await {
              Ok(record) => record,
              Err(e) => panic!("{}", e),
            };
            sender.send(record).await.unwrap_or_else(|e| panic!("{}", e));
          });
        }

        self.score += count;
        
        if self.done {
          return SceneSwitch::Pop::<Ev>
        } else {
          return SceneSwitch::None::<Ev>
        }
      }
    }
    if self.music.playing() { self.music.pause(); }
    SceneSwitch::None::<Ev>
  }
  
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    ctx.gfx.add_font("arcade", FontData::from_path(ctx, "/arcade.ttf")?,);
            
    let score = format!("{:0>4}", self.score);
    let text = format!("score {}", score);
    self.draw_text( &mut canvas, &text,[16., 32.], 16.,Color::WHITE );
    
    let record = format!("{:0>4}", self.record);
    let text = format!("record {}", record);
    self.draw_text( &mut canvas, &text,[192., 32.], 16.,Color::CYAN );

    if self.block.is_none() {
      let shape: Shape = rand::random();
      let block: Block = Piece::create(shape);
      self.block = Some(block);
    }
    
    self.block.unwrap().draw(&mut canvas, ctx)?;

    // draws the board
    self.board.render(&mut canvas, ctx);

    canvas.finish(ctx)?;
    Ok(())
  }
  
  fn input(&mut self, _ctx: &mut Context, input: KeyInput) {
    match input.keycode {
      Some(KeyCode::Left) => {
        let mut block: Block = self.block.unwrap();
        block.move_left();
        self.block = Some(block);
      }
      Some(KeyCode::Right) => {
        let mut block: Block = self.block.unwrap();
        block.move_right();
        self.block = Some(block);
      }
      Some(KeyCode::Up) => {
        let mut block: Block = self.block.unwrap();
        block.rotate();
        self.block = Some(block);
      }
      Some(KeyCode::P) => {
        self.pause = !self.pause;
      }
      Some(KeyCode::Return) => {
          self.done = true;
      }
      _ => (),
    }
  }
  
  fn name(&self) -> &str {
    "play"
  }
  
  /// This returns whether or not to draw the next scene down on the
  /// stack as well; this is useful for layers or GUI stuff that
  /// only partially covers the screen.
  fn draw_previous(&self) -> bool {
      false
  }
}

