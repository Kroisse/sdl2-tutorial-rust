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
