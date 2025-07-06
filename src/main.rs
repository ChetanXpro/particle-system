use crate::rand::gen_range;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

struct Particle {
    position_y: f32,
    position_x: f32,
    velocity_y: f32,
    velocity_x: f32,
    color: Color,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let radius = 2.0;

    let mut particles: Vec<Particle> = Vec::new();
    let mut frame_count = 0;
    let mut last_fps_update: Instant = Instant::now();

    for _ in 0..10000 {
        particles.push(Particle {
            position_y: gen_range(1, screen_height() as i64) as f32,
            position_x: gen_range(0, screen_width() as i64) as f32,
            velocity_y: gen_range(1.0, 5.0),
            velocity_x: gen_range(1.0, 5.0),
            color: WHITE,
        });
    }
    let mut image = Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK);
    let black_pixels = vec![BLACK; (screen_width() * screen_height()) as usize];
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        image.update(&black_pixels);

        for particle in &mut particles {
            particle.position_y = particle.position_y + particle.velocity_y;
            particle.position_x = particle.position_x + particle.velocity_x;

            // Left Edge
            if (particle.position_x - radius) < 0.0 {
                particle.velocity_x *= -1.0
            }

            // Right Edge
            if (particle.position_x + radius) > screen_width() {
                particle.velocity_x *= -1.0
            }

            // Top Edge
            if (particle.position_y - radius) < 0.0 {
                particle.velocity_y *= -1.0
            }

            // Bottom Edge
            if (particle.position_y + radius) > screen_height() {
                particle.velocity_y *= -1.0
            }

            let x = particle.position_x as u32;
            let y = particle.position_y as u32;

            if x < screen_width() as u32 && y < screen_height() as u32 {
                image.set_pixel(x, y, particle.color);
            }
        }

        frame_count += 1;
        if last_fps_update.elapsed() >= Duration::from_secs(1) {
            let elapsed = last_fps_update.elapsed();
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;

            last_fps_update = Instant::now();

            println!("FPS: {:.0}", fps)
        }

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await
    }
}
