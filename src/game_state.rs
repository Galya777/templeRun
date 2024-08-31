use ggez::{Context, GameResult};
use ggez::conf::{Conf, WindowMode};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use ggez::graphics;
use ggez;
use std::path;

use ggez::input::keyboard::{self, KeyCode};
use assets::{Assets, TextSprite};
use crate::assets;
use crate::player::Player;
use crate::jumping_obstacle::Obstacle;

#[derive(Debug)]
pub struct GameState {
    // Свойства за играта
    player: Player, 
    obstacles: Vec<Obstacle>,
    game_over:bool,  
    assets:Assets, 
    screen_width: f32,
    screen_height: f32,
}

impl GameState {
    pub fn new(ctx: &mut Context, conf: &Conf) -> GameResult<GameState> {
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let player_pos = Point2 {
                x: screen_width / 2.0,
                y: screen_height,
            };
        // Инициализация на играта
         let s=GameState {
            //свойства на играта
            player: Player::new(player_pos),
            game_over:false, 
            assets: assets, 
            //should be random values
            obstacles: vec![Obstacle::new(400.0, 100.0, 50.0, 50.0)], // Example obstacle
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
        };
        Ok(s)

    }

    fn check_collisions(&self) {
        for obstacle in &self.obstacles {
            if self.player.pos.x < obstacle.x + obstacle.width &&
               self.player.pos.x + 50.0 > obstacle.x &&
               self.player.pos.y < obstacle.y + obstacle.height &&
               self.player.pos.y + 50.0 > obstacle.y {
                println!("Collision detected!");
                // Handle collision (e.g., end game or reduce player health)
            }
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        // Обновяване на логиката на играта тук
        if self.game_over {
            return Ok(());
        }
        const DESIRED_FPS: u32 = 60;
        
        let seconds = _ctx.time.delta().as_secs_f32();  
        self.player.update(seconds);
        
        if keyboard::is_key_pressed(_ctx, KeyCode::Space) {
            self.player.jump();
        }
      

        for obstacle in &mut self.obstacles {
            obstacle.update();
        }

        self.check_collisions();
        
        
        Ok(())
    
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw your game objects here
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        let mut canvas = graphics::Canvas::from_frame(ctx, dark_blue);
        

        // Draw your infinite background here
        self.player.draw(&mut canvas, &self.assets);

        for obstacle in &self.obstacles {
            obstacle.draw(ctx)?;
        }

        canvas.finish(ctx)?;
        
    Ok(())
    }

    // Добавете обработка на събития тук, ако е необходимо
}
