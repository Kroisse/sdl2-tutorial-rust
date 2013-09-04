/* http://twinklebeardev.blogspot.kr/2012/07/lesson-2-dont-put-everything-in-main.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */

use sdl::window::{Window};
use sdl::render::{Renderer, Texture};
use sdl::surface::Surface;
mod sdl;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);

fn load_image<'a>(renderer: &'a Renderer, file: &Path) -> Result<~Texture<'a>, ~str> {
    do Surface::from_bmp(file).chain |image| {
        renderer.create_texture_from_surface(image)
    }
}

fn apply_surface(x: int, y: int, texture: &Texture, renderer: &Renderer) {
    let (w, h) = texture.size();
    let rect = sdl::Rect(x, y, w, h);
    renderer.copy_(texture, Some(&rect));
}

fn blit_background(size: (uint, uint), texture: &Texture, renderer: &Renderer) {
    let (bw, bh) = texture.size();
    let (bw, bh) = (bw as int, bh as int);
    let (w, h) = match size { (w, h) => (w as int, h as int) };

    for j in range(0, h / bh + 1) {
        for i in range(0, w / bw + 1) {
            apply_surface(i * bw, j * bh, texture, renderer);
        }
    }
}

fn main() {
    sdl::init(sdl::SDL_INIT_EVERYTHING()).unwrap();
    let w = match SCREEN_RESOLUTION {
        (w, h) => Window::new("Lesson 2", 0, 0, w, h).unwrap()
    };
    let ren = w.create_renderer(-1).unwrap();
    let background = load_image(ren, &Path("Lesson2res/background.bmp")).unwrap();
    let image = load_image(ren, &Path("Lesson2res/image.bmp")).unwrap();

    ren.clear();
    blit_background(SCREEN_RESOLUTION, background, ren);

    let (iw, ih) = image.size();
    let (x, y) = match SCREEN_RESOLUTION {
        (w, h) => ((w / 2 - iw / 2) as int, (h / 2 - ih / 2) as int)
    };
    apply_surface(x, y, image, ren);
    
    ren.present();
    sdl::delay(2000);
}
