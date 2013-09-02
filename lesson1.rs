/* http://twinklebeardev.blogspot.kr/2012/07/lesson-1-hello-world.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */

use std::libc::{uint32_t, c_int};
use std::str::raw::from_c_str;
use std::c_str::ToCStr;
use std::ptr;

mod ll {
    use std::libc::{uint32_t, c_char, c_int};

    pub static SDL_INIT_TIMER: uint32_t = 0x00000001;
    pub static SDL_INIT_AUDIO: uint32_t = 0x00000010;
    /**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
    pub static SDL_INIT_VIDEO: uint32_t = 0x00000020;
    /**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
    pub static SDL_INIT_JOYSTICK: uint32_t = 0x00000200;
    pub static SDL_INIT_HAPTIC: uint32_t = 0x00001000;
    /**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
    pub static SDL_INIT_GAMECONTROLLER: uint32_t = 0x00002000;
    pub static SDL_INIT_EVENTS: uint32_t = 0x00004000;
    /**< Don't catch fatal signals */
    pub static SDL_INIT_NOPARACHUTE: uint32_t = 0x00100000;
    pub static SDL_INIT_EVERYTHING: uint32_t = (
                SDL_INIT_TIMER | SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_EVENTS |
                SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER
            );

    pub struct SDL_Window;
    pub struct SDL_Renderer;

    pub mod SDL_WindowFlags {
        use std::libc::uint32_t;

        /**< fullscreen window */
        pub static SDL_WINDOW_FULLSCREEN: uint32_t = 0x00000001;
        /**< window usable with OpenGL context */
        pub static SDL_WINDOW_OPENGL: uint32_t = 0x00000002;
        /**< window is visible */
        pub static SDL_WINDOW_SHOWN: uint32_t = 0x00000004;
        /**< window is not visible */
        pub static SDL_WINDOW_HIDDEN: uint32_t = 0x00000008;
        /**< no window decoration */
        pub static SDL_WINDOW_BORDERLESS: uint32_t = 0x00000010;
        /**< window can be resized */
        pub static SDL_WINDOW_RESIZABLE: uint32_t = 0x00000020;
        /**< window is minimized */
        pub static SDL_WINDOW_MINIMIZED: uint32_t = 0x00000040;
        /**< window is maximized */
        pub static SDL_WINDOW_MAXIMIZED: uint32_t = 0x00000080;
        /**< window has grabbed input focus */
        pub static SDL_WINDOW_INPUT_GRABBED: uint32_t = 0x00000100;
        /**< window has input focus */
        pub static SDL_WINDOW_INPUT_FOCUS: uint32_t = 0x00000200;
        /**< window has mouse focus */
        pub static SDL_WINDOW_MOUSE_FOCUS: uint32_t = 0x00000400;

        pub static SDL_WINDOW_FULLSCREEN_DESKTOP: uint32_t = SDL_WINDOW_FULLSCREEN | 0x00001000;
        /**< window not created by SDL */
        pub static SDL_WINDOW_FOREIGN: uint32_t = 0x00000800;
    }

    pub mod SDL_RendererFlags {
        use std::libc::uint32_t;

        /**< The renderer is a software fallback */
        pub static SDL_RENDERER_SOFTWARE: uint32_t = 0x00000001;
        /**< The renderer uses hardware acceleration */
        pub static SDL_RENDERER_ACCELERATED: uint32_t = 0x00000002;
        /**< Present is synchronized with the refresh rate */
        pub static SDL_RENDERER_PRESENTVSYNC: uint32_t = 0x00000004;
        /**< The renderer supports rendering to texture */
        pub static SDL_RENDERER_TARGETTEXTURE: uint32_t = 0x00000008;
    }

    pub struct SDL_RWops;
    pub struct SDL_Surface;
    pub struct SDL_Texture;

    pub struct SDL_Rect { x: c_int, y: c_int, w: c_int, h: c_int }

    #[link_args="-framework SDL2"]
    extern {
        fn SDL_Init(flags: uint32_t) -> c_int;
        fn SDL_GetError() -> *c_char;

        fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int, w: c_int, h: c_int,
                            flags: uint32_t) -> *SDL_Window;
        fn SDL_DestroyWindow(window: *SDL_Window);

        fn SDL_CreateRenderer(window: *SDL_Window, index: c_int, flags: uint32_t) -> *SDL_Renderer;
        fn SDL_DestroyRenderer(renderer: *SDL_Renderer);

        fn SDL_RenderClear(renderer: *SDL_Renderer) -> c_int;
        fn SDL_RenderCopy(renderer: *SDL_Renderer, texture: *SDL_Texture,
                          srcrect: *SDL_Rect, dstrect: *SDL_Rect) -> c_int;
        fn SDL_RenderPresent(renderer: *SDL_Renderer);

        fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *SDL_RWops;

        fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) -> *SDL_Surface;
        fn SDL_FreeSurface(surface: *SDL_Surface);

        fn SDL_CreateTextureFromSurface(renderer: *SDL_Renderer, surface: *SDL_Surface) -> *SDL_Texture;
        fn SDL_DestroyTexture(texture: *SDL_Texture);

        fn SDL_Delay(ms: uint32_t);
    }
}

struct Window {
    p_window: *ll::SDL_Window,
    renderers: ~[Renderer],
}

struct Renderer {
    p_renderer: *ll::SDL_Renderer,
}

struct Surface {
    p_surface: *ll::SDL_Surface,
}

struct Texture<'self> {
    parent: &'self Renderer,
    p_texture: *ll::SDL_Texture,
}

#[fixed_stack_segment]
fn init(flags: uint32_t) -> Result<(), ~str> {
    unsafe {
        if ll::SDL_Init(flags) < 0 {
            let msg = ll::SDL_GetError();
            return Err(from_c_str(msg));
        }
    }
    Ok(())
}

#[fixed_stack_segment]
fn delay(ms: uint) {
    unsafe {
        ll::SDL_Delay(ms as uint32_t);
    }
}

impl Window {
    #[fixed_stack_segment]
    fn new(title: &str, x: int, y: int, w: int, h: int) -> Result<~Window, ~str> {
        unsafe {
            let p = title.with_c_str(|title| ll::SDL_CreateWindow(title,
                                                                  x as c_int, y as c_int,
                                                                  w as c_int, h as c_int,
                                                                  ll::SDL_WindowFlags::SDL_WINDOW_SHOWN));
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            Ok(~Window{ p_window: p, renderers: ~[] })
        }
    }

    #[fixed_stack_segment]
    fn create_renderer<'a>(&'a mut self, index: int) -> Result<&'a mut Renderer, ~str> {
        unsafe {
            let flags = ll::SDL_RendererFlags::SDL_RENDERER_ACCELERATED | ll::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC;
            let p = ll::SDL_CreateRenderer(self.p_window, index as c_int, flags);
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            self.renderers.unshift(Renderer{ p_renderer: p });
            Ok(&mut self.renderers[0])
        }
    }
}

impl Renderer {
    #[fixed_stack_segment]
    fn create_texture_from_surface<'a>(&'a self, surface: &Surface) -> Result<~Texture<'a>, ~str> {
        unsafe {
            let p = ll::SDL_CreateTextureFromSurface(self.p_renderer, surface.p_surface);
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            Ok(~Texture{ parent: self, p_texture: p })
        }
    }

    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe {
            ll::SDL_RenderClear(self.p_renderer);
        }
    }

    #[fixed_stack_segment]
    fn copy_(&self, texture: &Texture) {
        unsafe {
            ll::SDL_RenderCopy(self.p_renderer, texture.p_texture, ptr::null(), ptr::null());
        }
    }

    #[fixed_stack_segment]
    fn present(&self) {
        unsafe {
            ll::SDL_RenderPresent(self.p_renderer);
        }
    }
}

impl Drop for Window {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyWindow(self.p_window);
        }
    }
}

impl Drop for Renderer {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyRenderer(self.p_renderer);
        }
    }
}

impl Drop for Surface {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_FreeSurface(self.p_surface);
        }
    }
}

#[unsafe_destructor]
impl<'self> Drop for Texture<'self> {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyTexture(self.p_texture);
        }
    }
}

impl Surface {
    #[fixed_stack_segment]
    fn from_bmp(path: &Path) -> Result<~Surface, ~str> {
        unsafe {
            let mode = "rb".to_c_str();
            let p = do mode.with_ref |mode| {
                do path.with_c_str |file| {
                    ll::SDL_LoadBMP_RW(ll::SDL_RWFromFile(file, mode), 1)
                }
            };
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            Ok(~Surface {p_surface: p})
        }
    }
}

fn main() {
    init(ll::SDL_INIT_EVERYTHING).unwrap();
    let mut w = Window::new("Title", 0, 0, 960, 640).unwrap();
    let mut ren = w.create_renderer(-1).unwrap();
    let bmp = Surface::from_bmp(&Path("hello.bmp")).unwrap();
    let tex = ren.create_texture_from_surface(bmp).unwrap();
    ren.clear();
    ren.copy_(tex);
    ren.present();
    delay(2000);
    println("Hello?");
}
