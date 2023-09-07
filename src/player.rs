use crate::animation::Animation;
use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub animations: Vec<Animation>,
    pub current_anim: usize,
    pub flipped: bool,
}

impl Player {
    pub fn new(position: Vector2) -> Player {
        let player = Player {
            position,
            animations: Vec::new(),
            current_anim: 0,
            flipped: false
        };
        
        return player;
    }

    pub fn add_animation(&mut self, image_path: &str, length: i32, speed: i32,
                         rl: &mut RaylibHandle, thread: &RaylibThread) {
        let anim = Animation::new(image_path, length, 3, speed, rl, &thread);

        self.animations.push(anim);
    }

    pub fn advance_anim(&mut self) {
        self.animations[self.current_anim].advance(self.flipped);
    }

    pub fn draw(&mut self, dl: &mut RaylibDrawHandle, thread: &RaylibThread) {
        self.animations[self.current_anim].draw(dl, &thread, self.position, self.flipped);
    }

}

