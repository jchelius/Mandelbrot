extern crate sdl2;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Point;

fn mb_draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, transform : (f32, f32, f32, f32)){
    let (width, height) = canvas.output_size().unwrap();
    let max_iters = 1000;
    let mut x0 = transform.0;
    let mut y0: f32 = transform.1;
        for c in 0..width{
            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;
            let mut i = 0;
            while i < max_iters && x2 + y2 < 2.*2.{
                y = 2.*x*y + y0;
                x = x2 - y2 + x0;
                x2 = x*x;
                y2 = y*y;
                i += 1;
            }
            let diff: i32 = i-6;
            // let intensity = ((diff.abs() as f32) / (max_iters as f32) * 255.) as u8;
            let intensity = if diff <= 6 {
                ((diff.abs() as f32) / 6. * 255.) as u8
            }
            else{
                255
            };
            if i != max_iters{
                canvas.set_draw_color(Color::RGB(intensity, 64, 255-intensity));
            }
            else{
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.draw_point(Point::new(c as i32, r as i32)).unwrap();
            x0 += transform.2 / (width as f32);
        }
        y0 += transform.3 / (height as f32);
        x0 = transform.0;
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    let window = video_subsystem.window("mandelbrot set", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    let mut needs_update = false;

    // transform: (x position of lower left point, y positiion of lower left point, width, height)
    let mut transform_hist = Vec::new();
    let transform_default = (-2., -1.12, 2.47, 2.24);
    let mut transform = transform_default;
    println!("transform: {:?}", transform);
    const SCALE_FACTOR: f32 = 1.3;
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    mb_draw(&mut canvas, transform);
    canvas.present();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown{keycode: Some(Keycode::Num0), ..} | 
                Event::KeyDown{keycode: Some(Keycode::R), ..} => {
                    transform_hist.clear();
                    transform = transform_default;
                    needs_update = true;
                }
                Event::MouseButtonUp { mouse_btn, x, y, ..}=>{
                    if mouse_btn == MouseButton::Right && transform != transform_default{
                        transform = if transform_hist.len() != 0 {transform_hist.pop().unwrap()} else {transform_default};
                        needs_update = true;
                    }
                    else if mouse_btn == MouseButton::Left{
                        if transform != transform_default{
                            transform_hist.push(transform);
                        }
                        if transform_hist.len() > 100{
                            continue;
                        }
                        transform.0 += (x as f32) / (WIDTH as f32) * transform.2;
                        transform.1 += (y as f32) / (HEIGHT as f32) * transform.3;
                        transform.2 /= SCALE_FACTOR;
                        transform.3 /= SCALE_FACTOR;
                        transform.0 -= transform.2 / 2.;
                        transform.1 -= transform.3 / 2.;
                        needs_update = true;
                    }
                },
                _ => {}
            }
        }

        if needs_update{
            mb_draw(&mut canvas, transform);
            needs_update = false;
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
