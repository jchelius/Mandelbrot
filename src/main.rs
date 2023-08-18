extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Point;

fn mb_draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    let (width, height) = canvas.output_size().unwrap();
    let max_iters = 1000;
    // let width = canvas.output_size().unwrap();
    // width_f = width as f64;
    // height_f = height as f64;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut x0=-2.;
    let mut y0 = -1.12;
    let mut tot_intensity=0.;
    let mut vec = Vec::new();
    for r in 0..height{
        for c in 0..width{
            // println!("({x0}, {y0})");
            // scale x0 to be within -2.00 and 0.47
            // let x0 = -2.00 + 2.47/(width as f32) * (c as f32);
            // let y0 = -1.12 + 2.24/(height as f32) * (r as f32);
            let mut x = 0.0;
            let mut y = 0.0;
            let mut i = 0;
            while i < max_iters && x*x + y*y < 2.*2.{
                let xtemp = x*x - y*y + x0;
                y = 2.*x*y + y0;
                x = xtemp;
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
            vec.push(i);
            if i != max_iters{
                // print!("{intensity}\n")
                tot_intensity += intensity as f32 / (height * width) as f32;
                canvas.set_draw_color(Color::RGB(intensity, 64, 255-intensity));
            }
            else{
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.draw_point(Point::new(c as i32, r as i32)).unwrap();
            
            x0 += 2.47/(width as f32);
        }
        y0 += 2.24/(height as f32);
        x0 = -2.;
        // break;
    }
    // let avg_intensity = tot_intensity as f32 / (height * width) as f32;
    println!("{tot_intensity}");
    vec.sort();
    // println!("{:?} {:?} {:?}", vec.len(), vec[0], vec[vec.len() - 1]);
    let mid = vec.len() / 2; 
    let median = if vec.len() % 2 == 0{
        (vec[mid] + vec[mid+1])/2
    }
    else{
        vec[mid]
    };
    println!("low: {:?} high: {:?} median : {:?}", vec.first().unwrap(), vec.last().unwrap(), median);
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 800;
    let height = 800;
    // let palettes = [()];
    let window = video_subsystem.window("mandelbrot set", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    mb_draw(&mut canvas);
    canvas.present();
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseWheel {y: yval, ..}=>{
                    // let mouse_event = Event::MouseWheel { 0, 0, 0, 0, 0};
                    println!("{yval}");
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}