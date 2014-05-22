use libc::{c_int};

use super::ll;
use super::get_error;


pub struct Window {
    pub p_window: *mut ll::SDL_Window,
}


impl Window {
        pub fn new(title: &str, x: int, y: int, w: uint, h: uint) -> Result<Window, StrBuf> {
        let p = unsafe {
            title.with_c_str(|title| {
                ll::SDL_CreateWindow(title, x as c_int, y as c_int,
                                            w as c_int, h as c_int,
                                            ll::SDL_WINDOW_SHOWN)
            })
        };
        if p.is_null() {
            Err(get_error())
        } else {
            Ok(Window{ p_window: p })
        }
    }

        pub fn size(&self) -> (uint, uint) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            ll::SDL_GetWindowSize(self.p_window, &mut w, &mut h);
        }
        (w as uint, h as uint)
    }
}

impl Drop for Window {
        fn drop(&mut self) {
        unsafe {
            ll::SDL_DestroyWindow(self.p_window);
        }
    }
}
