/* http://twinklebeardev.blogspot.kr/2012/08/lesson-5-clipping-sprite-sheets.html
 * rustc 0.11.0-pre (1c4a9b9 2014-05-19 13:36:22 -0700)
 * host: x86_64-apple-darwin
 */
extern crate sdl2;

use sdl2::video::{Window};
use sdl2::render::{Renderer, Texture};
use sdl2::ext::image::load_texture;
use sdl2::events;
use sdl2::keycode;
use sdl2::rect::{ToRect, ToSize};

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);

fn render_texture<T: ToRect>(texture: &Texture, renderer: &Renderer, position: (int, int), clip: T) {
    let (x, y) = position;
    let (w, h) = match clip.to_rect() {
        Some(r) => r.to_size().unwrap(),
        None => texture.size(),
    };
    let rect = (x, y, w, h);
    renderer.copy_(texture, clip, rect);
}

fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    let (w, h) = SCREEN_RESOLUTION;
    let win = Window::new("Lesson 5", 0, 0, w, h).unwrap();
    let ren = win.create_renderer(-1).unwrap();
    let image = load_texture(&ren, &Path::new("res/image.png")).unwrap();

    let (iw, ih) = (100, 100);
    let mut clips = Vec::with_capacity(4);
    let mut column = 0;
    for i in range(0, 4) {
        if i != 0 && i % 2 == 0 { column += 1 }
        clips.push((column * iw, i % 2 * ih, iw as uint, ih as uint));
    }
    let clips = clips;

    let mut quit = false;
    let x = (w as int - iw) / 2;
    let y = (h as int - ih) / 2;
    let mut use_clip = 0;
    while !quit {
        loop {
            match events::poll() {
                events::Quit(_) => quit = true,
                events::MouseButtonDown(_) => quit = true,
                events::NoEvent => break,
                events::KeyDown(e) => {
                    match e.sym {
                        keycode::K_ESCAPE => quit = true,
                        keycode::K_1 => use_clip = 0,
                        keycode::K_2 => use_clip = 1,
                        keycode::K_3 => use_clip = 2,
                        keycode::K_4 => use_clip = 3,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        ren.clear();
        render_texture(&image, &ren, (x, y), clips.get(use_clip));
        ren.present();
    }
    sdl2::quit();
}
