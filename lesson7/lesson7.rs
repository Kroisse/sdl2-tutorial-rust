/* http://twinklebeardev.blogspot.kr/2012/09/lesson-7-taking-advantage-of-classes.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */
extern mod sdl2;

use sdl2::video;
use sdl2::render::{Renderer, Texture, RendererFlip, FlipNone, FlipVertical};
use sdl2::ext::image::load_texture;
use sdl2::ext::ttf;
use sdl2::events;
use sdl2::keycode;
use sdl2::rect::*;
use sdl2::pixels::*;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);
static speed: int = 10;

struct Window<'self> {
    window: ~video::Window,
    renderer: ~Renderer<'self>,
}

impl<'self> Window<'self> {
    fn new(title: &str) -> Result<~Window, ~str> {
        let initialized = do sdl2::init(sdl2::SDL_INIT_EVERYTHING()).and_then |_| {
            ttf::init()
        };
        if initialized.is_err() {
            return Err(initialized.unwrap_err());
        }
        let (w, h) = SCREEN_RESOLUTION;
        let win = match video::Window::new(title, 0, 0, w, h) {
            Ok(w) => w,
            Err(e) => return Err(e),
        };
        let ren = match win.create_renderer::<'self>(-1) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        Ok(~Window {window: win, renderer: ren})
    }
    
    fn draw<D: ToRect, E: ToRect>(&self, texture: &Texture, dest: &D, clip: &E,
                                  angle: float, pivot: (int, int), flip: RendererFlip) {
        let dest = dest.to_rect().unwrap();
        let pivot = match pivot {
            (x, y) => (x + dest.w as int / 2, y + dest.h as int / 2)
        };
        self.renderer.copy_ex(texture, clip, &dest, angle, &pivot, flip);
    }
    
    fn load_image<'a>(&'a self, file: &Path) -> Result<~Texture<'a>, ~str> {
        load_texture(self.renderer, file)
    }
    
    fn render_text<'a>(&'a self, message: &str, font_path: &Path,
                       color: Color, font_size: int) -> Result<~Texture<'a>, ~str> {
        do ttf::Font::new(font_path, font_size).and_then |font| {
            do font.render_blended(message, color).and_then |surf| {
                self.renderer.create_texture_from_surface(surf)
            }
        }
    }

    fn clear(&self) { self.renderer.clear() }

    fn present(&self) { self.renderer.present() }

    fn size(&self) -> (uint, uint) { self.window.size() }
}

#[unsafe_destructor]
impl<'self> Drop for Window<'self> {
    fn drop(&self) {
        ttf::quit();
        sdl2::quit();
    }
}

fn main() {
    let win = Window::new("Lesson 7").unwrap();
    let img = win.load_image(&Path("res/image.png")).unwrap();
    let font_path = Path("res/SourceSansPro-Regular.ttf");
    let text = "TTF Fonts too!";
    let color = RGBA(255, 255, 255, 255);
    let msg = win.render_text(text, &font_path, color, 25).unwrap();

    let pos = match win.size() {
        (w, h) => (w as int / 2 - 150 / 2, h as int / 2 - 150 / 2, 150u, 150u)
    };
    let mut angle = 0.;

    let mut quit = false;
    while (!quit) {
        loop {
            match events::poll() {
                events::Quit(_) => quit = true,
                events::MouseButtonDown(_) => quit = true,
                events::NoEvent => break,
                events::KeyDown(e) => {
                    match e.sym {
                        keycode::K_ESCAPE => quit = true,
                        keycode::K_d => angle += 2.,
                        keycode::K_a => angle -= 2.,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        win.clear();
        win.draw(img, &pos, &(), angle, (0, 0), FlipNone);
        win.draw(img, &pos, &(), angle, (0, 0), FlipVertical);
        win.present();
    }
}

// workaround
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
     std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
