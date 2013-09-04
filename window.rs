use std::ptr;
use std::str::raw::from_c_str;
use std::libc::{c_int};

use super::ll;


pub struct Window {
    p_window: *mut ll::SDL_Window,
}


impl Window {
    #[fixed_stack_segment]
    pub fn new(title: &str, x: int, y: int, w: uint, h: uint) -> Result<~Window, ~str> {
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
}

impl Drop for Window {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyWindow(self.p_window);
        }
    }
}
