/* http://twinklebeardev.blogspot.kr/2012/08/lesson-5-clipping-sprite-sheets.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */
extern mod sdl2;

use std::vec;

use sdl2::video::{Window};
use sdl2::render::{Renderer, Texture};
use sdl2::ext::image::load_texture;
use sdl2::events;
use sdl2::keycode;
use sdl2::rect::*;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);
static speed: int = 10;

fn load_image<'a>(renderer: &'a Renderer, file: &Path) -> Result<~Texture<'a>, ~str> {
    load_texture(renderer, file)
}

fn render_texture<T:ToRect>(texture: &Texture, renderer: &Renderer, position: (int, int), clip: &T) {
    let (x, y) = position;
    let (w, h) = match clip.to_rect() {
        Some(r) => r.to_size().unwrap(),
        None => texture.size(),
    };
    let rect = (x, y, w, h);
    renderer.copy_(texture, clip, &rect);
}

fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    let w = match SCREEN_RESOLUTION {
        (w, h) => Window::new("Lesson 5", 0, 0, w, h).unwrap()
    };
    let ren = w.create_renderer(-1).unwrap();
    let image = load_image(ren, &Path("res/image.png")).unwrap();

    let (iw, ih) = (100, 100);
    let clips = do vec::build(Some(4)) |push| {
        let mut column = 0;
        for i in range(0, 4) {
            if i != 0 && i % 2 == 0 { column += 1 }
            push((column * iw, i % 2 * ih, iw as uint, ih as uint));
        }
    };

    let mut quit = false;
    let x = (SCREEN_RESOLUTION.n0() as int - iw) / 2;
    let y = (SCREEN_RESOLUTION.n1() as int - ih) / 2;
    let mut use_clip = 0;
    while (!quit) {
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
        render_texture(image, ren, (x, y), &clips[use_clip]);
        ren.present();
    }
    sdl2::quit();
}

// workaround
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
     std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
