/* http://twinklebeardev.blogspot.kr/2012/07/lesson-1-hello-world.html
 * rustc 0.11.0-pre (1c4a9b9 2014-05-19 13:36:22 -0700)
 * host: x86_64-apple-darwin
 */

extern crate sdl2;


fn main() {
    sdl2::init(sdl2::SDL_INIT_EVERYTHING()).unwrap();
    let w = sdl2::video::Window::new("Title", 0, 0, 960, 640).unwrap();
    let ren = w.create_renderer(-1).unwrap();
    let bmp = sdl2::surface::Surface::from_bmp(&Path::new("res/hello.bmp")).unwrap();
    let tex = ren.create_texture_from_surface(&bmp).unwrap();
    ren.clear();
    ren.copy_(&tex, (), ());
    ren.present();
    sdl2::delay(2000);
    println!("Hello?");
}
