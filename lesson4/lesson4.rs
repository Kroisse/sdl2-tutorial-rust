/* http://twinklebear.github.io/sdl2%20tutorials/2013/08/20/lesson-4-handling-events/
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */

use sdl::window::{Window};
use sdl::render::{Renderer, Texture};
use sdl::ext::image::load_texture;
use sdl::events;
mod sdl;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);

fn load_image<'a>(renderer: &'a Renderer, file: &Path) -> Result<~Texture<'a>, ~str> {
    load_texture(renderer, file)
}

fn render_texture(texture: &Texture, renderer: &Renderer, position: (int, int)) {
    let (x, y) = position;
    let (w, h) = texture.size();
    let rect = (x, y, w, h);
    renderer.copy_(texture, &(), &rect);
}

fn main() {
    sdl::init(sdl::SDL_INIT_EVERYTHING()).unwrap();
    let w = match SCREEN_RESOLUTION {
        (w, h) => Window::new("Lesson 4", 0, 0, w, h).unwrap()
    };
    let ren = w.create_renderer(-1).unwrap();
    let image = load_image(ren, &Path("res/image.png")).unwrap();

    let mut quit = false;
    while (!quit) {
        loop {
            match events::poll() {
                events::Quit(_) => quit = true,
                events::KeyDown(_) => quit = true,
                events::MouseButtonDown(_) => quit = true,
                events::NoEvent => break,
                _ => (),
            }
        }
        ren.clear();
        render_texture(image, ren, (0, 0));
        ren.present();
    }
    sdl::quit();
}

// workaround
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
     std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
