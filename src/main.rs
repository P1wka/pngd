use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::{BufReader, Write, stdin};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    print!("Type pixel count: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let input_u64: u64 = match input.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Unvalid number!");
            return;
        }
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("pngd", 800, 600)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let decoder = png::Decoder::new(BufReader::new(File::open("assets/png/file.png").unwrap()));
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..info.buffer_size()];
    let width = info.width as usize;
    let height = info.height as usize;

    let bpp = info.color_type.samples() as usize;

    let total_pixels = (width * height) as u64;

    if input_u64 > total_pixels {
        println!("Warningı: PNG file is including {} pixels!.", total_pixels);
    }

    let draw_count = input_u64.min(total_pixels);

    let mut coords: Vec<(usize, usize)> =
        (0..width).flat_map(|x| (0..height).map(move |y| (x, y))).collect();

    coords.shuffle(&mut thread_rng());

    for (x, y) in coords.into_iter().take(draw_count as usize) {
        let i = (y * width + x) * bpp;

        let r = bytes[i];
        let g = bytes[i + 1];
        let b = bytes[i + 2];

        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1)).unwrap();
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        canvas.present();
    }
}
