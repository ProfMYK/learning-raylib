use raylib::prelude::*;
use raylib::texture::Image;

pub struct Animation {
    pub texture: Texture2D,
    pub texture_flipped: Texture2D,
    pub frame_rect: Rectangle,
    pub current_frame: i32,
    pub speed: i32,
    pub length: i32
}

impl Animation {
    pub fn new(image_path: &str,
               length: i32,
               resize: i32,
               speed: i32,
               rl: &mut RaylibHandle,
               thread: &RaylibThread) -> Animation {
        let mut image = Image::load_image(image_path)
            .expect("Expected proper image load with pat");
        image.resize_nn(image.width * resize, image.height * resize);
        let texture = rl.load_texture_from_image(&thread, &image)
            .expect("expected proper texture");
        image.flip_horizontal();
        let texture_flip = rl.load_texture_from_image(&thread, &image)
            .expect("expected proper texture flip");

        let rec = Rectangle::new(0.0, 
                                 0.0,
                                 (image.width / length) as f32,
                                 image.height as f32);

        let anim = Animation {
            texture,
            texture_flipped: texture_flip,
            frame_rect: rec,
            current_frame: 0,
            speed,
            length
        };

        return anim;
    }

    pub fn advance(&mut self, flipped: bool) {
        if !flipped {
            self.current_frame += 1;
            if self.current_frame >= self.length {
                self.current_frame = 0;
            }
        } else {
            self.current_frame -= 1;
            if self.current_frame < 0 {
                self.current_frame = self.length - 1;
            }
        }

        self.update_rect();
    }

    pub fn update_rect(&mut self) {
        self.frame_rect.x = (self.current_frame * self.texture.width / self.length) as f32;
    }

    pub fn draw(&mut self,
                dl: &mut RaylibDrawHandle,
                thread: &RaylibThread,
                pos: Vector2,
                flipped: bool) {
        if !flipped {
            dl.draw_texture_rec(&self.texture,
                                self.frame_rect, 
                                pos, 
                                Color::WHITE);
        } else {
            dl.draw_texture_rec(&self.texture_flipped,
                                self.frame_rect, 
                                pos, 
                                Color::WHITE);
        }
    }
}
