mod animation;
mod player;

use raylib::prelude::*;
use animation::Animation;
use player::Player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920 / 2, 1080 / 2)
        .vsync()
        .build();

    let mut frame_counter = 0;

    let mut player = Player::new(Vector2::new(300.0, 300.0));
    player.add_animation("assets/Player/Idle.png", 10, 7, &mut rl, &thread);
    player.add_animation("assets/Player/Run.png", 10, 7, &mut rl, &thread);

    player.current_anim = 1;

    while !rl.window_should_close() {
        let mut dl = rl.begin_drawing(&thread);

        frame_counter += 1;
        if frame_counter >= player.animations[player.current_anim].speed {
            frame_counter = 0;
            player.advance_anim();
        }

        player.draw(&mut dl, &thread);

        dl.clear_background(Color::new(60, 160, 250, 255));
        dl.draw_fps(10, 10);
    }
}
