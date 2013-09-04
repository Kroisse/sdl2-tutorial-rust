/* http://twinklebeardev.blogspot.kr/2012/07/lesson-1-hello-world.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */

mod sdl;


fn main() {
    sdl::init(sdl::SDL_INIT_EVERYTHING()).unwrap();
    let w = sdl::window::Window::new("Title", 0, 0, 960, 640).unwrap();
    let ren = w.create_renderer(-1).unwrap();
    let bmp = sdl::surface::Surface::from_bmp(&Path("Lesson1res/hello.bmp")).unwrap();
    let tex = ren.create_texture_from_surface(bmp).unwrap();
    ren.clear();
    ren.copy_(tex, None);
    ren.present();
    sdl::delay(2000);
    println("Hello?");
}
