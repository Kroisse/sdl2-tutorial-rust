/* http://twinklebeardev.blogspot.kr/2012/07/lesson-2-dont-put-everything-in-main.html
 * rustc 0.11.0-pre (1c4a9b9 2014-05-19 13:36:22 -0700)
 * host: x86_64-apple-darwin
 */
extern crate sdl2;

use sdl2::video::{Window};
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;

static SCREEN_RESOLUTION: (uint, uint) = (960, 640);

fn load_image<'a>(renderer: &'a Renderer, file: &Path) -> Result<Texture<'a>, StrBuf> {
    Surface::from_bmp(file).and_then(|image|
        renderer.create_texture_from_surface(&image))
}

fn apply_surface(x: int, y: int, texture: &Texture, renderer: &Renderer) {
    let (w, h) = texture.size();
    let rect = (x, y, w, h);
    renderer.copy_(texture, (), rect);
}

fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    let (w, h) = SCREEN_RESOLUTION;
    let win = Window::new("Lesson 2", 0, 0, w, h).unwrap();
    let ren = win.create_renderer(-1).unwrap();
    let background = load_image(&ren, &Path::new("res/background.bmp")).unwrap();
    let image = load_image(&ren, &Path::new("res/image.bmp")).unwrap();

    ren.clear();
    let (bw, bh) = background.size();
    let (bw, bh) = (bw as int, bh as int);
    apply_surface(0, 0, &background, &ren);
    apply_surface(bw, 0, &background, &ren);
    apply_surface(0, bh, &background, &ren);
    apply_surface(bw, bh, &background, &ren);

    let (iw, ih) = image.size();
    let (x, y) = ((w / 2 - iw / 2) as int, (h / 2 - ih / 2) as int);
    apply_surface(x, y, &image, &ren);

    ren.present();
    sdl2::delay(2000);
}
