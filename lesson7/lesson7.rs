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

struct GameContext {
    win: ~video::Window,
}

impl GameContext {
    fn new(title: &str) -> Result<GameContext, ~str> {
        let initialized = do sdl2::init(sdl2::SDL_INIT_EVERYTHING()).and_then |_| {
            ttf::init()
        };
        if initialized.is_err() {
            return Err(initialized.unwrap_err());
        }
        let (w, h) = SCREEN_RESOLUTION;
        match video::Window::new(title, 0, 0, w, h) {
            Ok(w) => Ok(GameContext{win: w}),
            Err(e) => Err(e),
        }
    }

    fn window<'a>(&'a self) -> &'a video::Window {
        borrow(self.win)
    }
}

fn borrow<'a, T>(v: &'a T) -> &'a T { v }

impl Drop for GameContext {
    fn drop(&self) {
        ttf::quit();
        sdl2::quit();
    }
}

trait RendererUtil {
    fn draw<D: ToRect, E: ToRect>(&self, texture: &Texture, dest: &D, clip: &E,
                                  angle: float, pivot: (int, int), flip: RendererFlip);
    fn load_image<'a>(&'a self, file: &Path) -> Result<~Texture<'a>, ~str>;
    fn render_text<'a>(&'a self, message: &str, font_path: &Path,
                       color: Color, font_size: int) -> Result<~Texture<'a>, ~str>;
}

impl<'self> RendererUtil for Renderer<'self> {
    fn draw<D: ToRect, E: ToRect>(&self, texture: &Texture, dest: &D, clip: &E,
                                  angle: float, pivot: (int, int), flip: RendererFlip) {
        let dest = dest.to_rect().unwrap();
        let pivot = match pivot {
            (x, y) => (x + dest.w as int / 2, y + dest.h as int / 2)
        };
        self.copy_ex(texture, clip, &dest, angle, &pivot, flip);
    }
    
    fn load_image<'a>(&'a self, file: &Path) -> Result<~Texture<'a>, ~str> {
        load_texture(self, file)
    }
    
    fn render_text<'a>(&'a self, message: &str, font_path: &Path,
                       color: Color, font_size: int) -> Result<~Texture<'a>, ~str> {
        do ttf::Font::new(font_path, font_size).and_then |font| {
            do font.render_blended(message, color).and_then |surf| {
                self.create_texture_from_surface(surf)
            }
        }
    }
}


fn main() {
    let ctx = GameContext::new("Lesson 7").unwrap();
    let win = ctx.window();
    let ren = win.create_renderer(-1).unwrap();
    let img = ren.load_image(&Path("res/image.png")).unwrap();
    let font_path = Path("res/SourceSansPro-Regular.ttf");
    let text = "TTF Fonts too!";
    let color = RGBA(255, 255, 255, 255);
    let msg = ren.render_text(text, &font_path, color, 25).unwrap();

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
        ren.clear();
        ren.draw(img, &pos, &(), angle, (0, 0), FlipNone);
        ren.draw(msg, &pos, &(), angle, (0, 0), FlipVertical);
        ren.present();
    }
}

// workaround
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
     std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
