/* http://twinklebeardev.blogspot.kr/2012/07/lesson-1-hello-world.html
 * rustc 0.8-pre
 * host: x86_64-apple-darwin
 */

use std::libc::{uint32_t, c_int};
use std::str::raw::from_c_str;
use std::c_str::ToCStr;
use std::ptr;

mod ll;

#[link_args="-framework SDL2"] extern {}

struct Window {
    p_window: *mut ll::SDL_Window,
}

struct Renderer<'self> {
    parent: &'self Window,
    p_renderer: *mut ll::SDL_Renderer,
}

struct Surface {
    p_surface: *mut ll::SDL_Surface,
}

struct Texture<'self> {
    parent: &'self Renderer<'self>,
    p_texture: *mut ll::SDL_Texture,
}


struct InitFlag(u32);
impl BitOr<InitFlag, InitFlag> for InitFlag {
    fn bitor(&self, rhs: &InitFlag) -> InitFlag {
        match (self, rhs) {
            (&InitFlag(a), &InitFlag(b)) => InitFlag(a | b)
        }
    }
}

static SDL_INIT_TIMER: InitFlag = InitFlag(0x00000001);
static SDL_INIT_AUDIO: InitFlag = InitFlag(0x00000010);
/**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
static SDL_INIT_VIDEO: InitFlag = InitFlag(0x00000020);
/**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
static SDL_INIT_JOYSTICK: InitFlag = InitFlag(0x00000200);
static SDL_INIT_HAPTIC: InitFlag = InitFlag(0x00001000);
/**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
static SDL_INIT_GAMECONTROLLER: InitFlag = InitFlag(0x00002000);
static SDL_INIT_EVENTS: InitFlag = InitFlag(0x00004000);
/**< Don't catch fatal signals */
static SDL_INIT_NOPARACHUTE: InitFlag = InitFlag(0x00100000);


#[fixed_stack_segment]
fn init(flags: InitFlag) -> Result<(), ~str> {
    let raw_flag = match flags {
        InitFlag(f) => f as uint32_t
    };
    unsafe {
        if ll::SDL_Init(raw_flag) < 0 {
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
                                                                  ll::SDL_WINDOW_SHOWN));
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            Ok(~Window{ p_window: p })
        }
    }

    #[fixed_stack_segment]
    fn create_renderer<'a>(&'a self, index: int) -> Result<~Renderer<'a>, ~str> {
        unsafe {
            let flags = ll::SDL_RENDERER_ACCELERATED | ll::SDL_RENDERER_PRESENTVSYNC;
            let p = ll::SDL_CreateRenderer(self.p_window, index as c_int, flags);
            if ptr::is_null(p) {
                return Err(from_c_str(ll::SDL_GetError()));
            }
            Ok(~Renderer{ parent: self, p_renderer: p })
        }
    }
}

impl<'self> Renderer<'self> {
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

#[unsafe_destructor]
impl<'self> Drop for Renderer<'self> {
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
    let SDL_INIT_EVERYTHING: InitFlag = SDL_INIT_TIMER | SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_EVENTS |
                                        SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER;
    init(SDL_INIT_EVERYTHING).unwrap();
    let w = Window::new("Title", 0, 0, 960, 640).unwrap();
    let ren = w.create_renderer(-1).unwrap();
    let bmp = Surface::from_bmp(&Path("hello.bmp")).unwrap();
    let tex = ren.create_texture_from_surface(bmp).unwrap();
    ren.clear();
    ren.copy_(tex);
    ren.present();
    delay(2000);
    println("Hello?");
}
