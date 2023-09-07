mod animation;

use raylib::prelude::*;
use animation::Animation;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920 / 2, 1080 / 2)
        .vsync()
        .build();

    let mut frame_counter = 0;

    let mut anim = Animation::new("assets/Player/Run.png",
                                  10, 3, 4,
                                  &mut rl, &thread);

    while !rl.window_should_close() {
        let mut dl = rl.begin_drawing(&thread);

        frame_counter += 1;
        if frame_counter >= anim.speed {
            anim.advance(false);
            frame_counter = 0;
        }

        anim.draw(&mut dl, &thread, Vector2::new(300.0, 300.0), false);

        dl.clear_background(Color::WHITE);
        dl.draw_fps(10, 10);
    }
}
