use ggez::graphics;
use ggez::{Context, GameResult};


#[derive(Debug)]
pub struct Obstacle {
    //no idea if this is good or not 
   pub x: f32,
   pub y: f32,
   pub width: f32,
   pub height: f32,
}

impl Obstacle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Obstacle {
        Obstacle { x, y, width, height }
    }

    pub fn update(&mut self) {
        self.x -= 5.0; // Move the obstacle to the left
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        //should be pictures as well
        let rectangle = graphics::Rect::new(self.x, self.y, self.width, self.height);
        let color = graphics::Color::from_rgb(255, 0, 0);
        let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rectangle, color)?;
        //find a way to draw images
        //graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
        Ok(())
    }
}
