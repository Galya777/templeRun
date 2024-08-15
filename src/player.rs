use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::mint::{Vector2, Point2};

use crate::assets::{Assets, Sprite};

#[derive(Debug)]
pub enum PlayerState{
    Normal, 
    Jumping, 
    LayingDown, 
}
#[derive(Debug)]
pub struct Player {
    pub pos: Point2<f32>,
    pub state: PlayerState, //Състояние на играча
    pub time_until_next_object: f32,
    velocity: Vector2<f32>,
        is_jumping:bool,
    is_laying_down:bool,
    has_gun:bool, //Дали героят има оръжие
    has_magnet:bool, //дали героят има магнит
    has_backpack:bool, //дали героят има раница, с която лети
    has_shoes:bool,//дали героят има обувки за скачане 
    hit:bool,//дали е ударен

    // Други свойства на играча
}

 impl Player {
     pub const SHOT_TIMEOUT: f32 = 1.0;
    pub fn new(pos: Point2<f32>) -> Self {
       
        // Инициализация на играча
        Player {
            pos,
            state: PlayerState::Normal,
            time_until_next_object: Self::SHOT_TIMEOUT,
            velocity: Vector2 { x: 0.0, y: -500.0 },
            is_jumping:false,
            is_laying_down:false,
            has_gun:false, 
            has_magnet:false, 
            has_backpack:false, 
            has_shoes:false,
            hit:false,
        }
    }

    pub fn jump(&mut self) {
        if !self.is_jumping {
            // Код за скок
            self.is_jumping = true;
            self.velocity.x = -10.0; 
            self.velocity.y = -10.0;  // Примерна скокова скорост
        }
    }

    pub fn update(&mut self, seconds: f32) {
        if self.is_jumping {
            // Код за обновление на позицията по време на скок
            self.pos.x += self.velocity.x * seconds;
            self.pos.y += self.velocity.y * seconds;
            // Симулиране на гравитацията
            self.velocity.x += 0.5;
            self.velocity.y += 0.5;
            
            // Проверка дали играчът се е върнал на земята
            if self.pos.y >= 0.0 {
                self.is_jumping = false;
                self.pos.y = 0.0;
            }
        }
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, assets: &Assets){
        match self.state {
            PlayerState::Normal => {
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.95, y: 0.95 }).
                    offset(Point2 { x: 0.5, y: 1.0 });
                canvas.draw(&assets.player_normal_image, draw_params);
            },
            PlayerState::Jumping =>{
                todo!()
            }, 
            PlayerState::LayingDown =>{
                todo!()
            }
        }
    }
}
