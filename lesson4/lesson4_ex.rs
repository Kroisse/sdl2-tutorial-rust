/* http://twinklebear.github.io/sdl2%20tutorials/2013/08/20/lesson-4-handling-events/
 * rustc 0.11.0-pre (1c4a9b9 2014-05-19 13:36:22 -0700)
 * host: x86_64-apple-darwin
 */
extern crate sdl2;

use sdl2::video::{Window};
use sdl2::render::{Renderer, Texture};
use sdl2::ext::image::load_texture;
use sdl2::events;
use sdl2::keycode;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);
static speed: int = 10;

fn render_texture(texture: &Texture, renderer: &Renderer, position: (int, int)) {
    let (x, y) = position;
    let (w, h) = texture.size();
    let rect = (x, y, w, h);
    renderer.copy_(texture, (), rect);
}

fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    let (w, h) = SCREEN_RESOLUTION;
    let win = Window::new("Lesson 4 - extra", 0, 0, w, h).unwrap();
    let ren = win.create_renderer(-1).unwrap();
    let image = load_texture(&ren, &Path::new("res/image.png")).unwrap();

    let mut quit = false;
    let mut x = 0;
    let mut y = 0;
    while !quit {
        loop {
            match events::poll() {
                events::Quit(_) => quit = true,
                events::MouseButtonDown(_) => quit = true,
                events::NoEvent => break,
                events::KeyDown(e) => {
                    match e.sym {
                        keycode::K_ESCAPE => quit = true,
                        keycode::K_LEFT  => x -= speed,
                        keycode::K_RIGHT => x += speed,
                        keycode::K_UP    => y -= speed,
                        keycode::K_DOWN  => y += speed,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        ren.clear();
        render_texture(&image, &ren, (x, y));
        ren.present();
    }
    sdl2::quit();
}
