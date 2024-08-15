use ggez::audio::{self, SoundSource};
use ggez::graphics::{self, Drawable};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Assets {
    pub player_normal_image:   graphics::Image,
  
 }

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_normal_image   = graphics::Image::from_path(ctx, "/Pictures/playerNormal.png")?;
   

        /*let mut shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        shot_sound.set_volume(0.5);

        let mut boom_sound = audio::Source::new(ctx, "/boom.ogg")?;
        boom_sound.set_volume(0.3);*/

        Ok(Assets {
            player_normal_image, 
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, center: Point2<f32>, canvas: &mut graphics::Canvas);
    fn width(&self, ctx: &mut Context) -> f32;
    fn height(&self, ctx: &mut Context) -> f32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str) -> GameResult<TextSprite> {
        let mut text = graphics::Text::new(label);

        text.set_font("MainFont");
        text.set_scale(graphics::PxScale::from(32.0));

        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, top_left: Point2<f32>, canvas: &mut graphics::Canvas) {
        canvas.draw(&self.text, graphics::DrawParam::default().dest(top_left))
    }

    fn width(&self, ctx: &mut Context) -> f32 { self.text.dimensions(ctx).unwrap().w }
    fn height(&self, ctx: &mut Context) -> f32 { self.text.dimensions(ctx).unwrap().h }
}