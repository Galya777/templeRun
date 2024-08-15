use ggez::conf::{Conf, WindowMode};
use ggez::{Context, GameResult};
use ggez::event::{self};
use ggez::graphics::{self, Color};


mod game_state;
pub mod player;
pub mod assets;
mod jumping_obstacle;
mod background;
mod slidingObstacke;

fn main() -> GameResult {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1200.0,
            height: 1000.0,
            ..Default::default()
        });
    
let (mut ctx, events_loop) = ggez::ContextBuilder::new("Temple Run", "Galya Dodova")
        .default_conf(conf.clone())
        .build().
        unwrap();


        //self.player.draw(&mut canvas, &self.assets);
        
    
    let game = game_state::GameState::new(&mut ctx, &conf).unwrap();

    ggez::event::run(ctx, events_loop, game);
}

