/* http://twinklebeardev.blogspot.kr/2012/08/lesson-6-true-type-fonts-with-sdlttf.html
 * rustc 0.11.0-pre (1c4a9b9 2014-05-19 13:36:22 -0700)
 * host: x86_64-apple-darwin
 */
extern crate sdl2;

use sdl2::video::{Window};
use sdl2::render::{Renderer, Texture};
use sdl2::ext::ttf;
use sdl2::events;
use sdl2::keycode;
use sdl2::rect::{ToRect, ToSize};
use sdl2::pixels::{Color, RGBA};

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);

fn render_texture<T: ToRect>(texture: &Texture, renderer: &Renderer, position: (int, int), clip: &T) {
    let (x, y) = position;
    let (w, h) = match clip.to_rect() {
        Some(r) => r.to_size().unwrap(),
        None => texture.size(),
    };
    let rect = (x, y, w, h);
    renderer.copy_(texture, clip, rect);
}

fn render_text<'a>(renderer: &'a Renderer, message: &str, font_path: &Path,
                   color: Color, font_size: int) -> Result<Texture<'a>, StrBuf> {
    ttf::Font::new(font_path, font_size).and_then(|font|
        font.render_blended(message, color).and_then(|surf|
            renderer.create_texture_from_surface(&surf)))
}

fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    ttf::init().unwrap();
    let (w, h) = SCREEN_RESOLUTION;
    let win = Window::new("Lesson 6", 0, 0, w, h).unwrap();
    let ren = win.create_renderer(-1).unwrap();
    let color = RGBA(255, 255, 255, 255);
    let image = render_text(&ren, "TTF fonts are cool!", &Path::new("res/SourceSansPro-Regular.ttf"), color, 80).unwrap();

    let mut quit = false;
    let (iw, ih) = image.size();
    let x = ((w - iw) / 2) as int;
    let y = ((h - ih) / 2) as int;
    while !quit {
        loop {
            match events::poll() {
                events::Quit(_) => quit = true,
                events::MouseButtonDown(_) => quit = true,
                events::NoEvent => break,
                events::KeyDown(e) => {
                    match e.sym {
                        keycode::K_ESCAPE => quit = true,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        ren.clear();
        render_texture(&image, &ren, (x, y), &());
        ren.present();
    }
    ttf::quit();
    sdl2::quit();
}
