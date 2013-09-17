use std::ptr;
use std::libc::{c_int};

use super::ll;
use super::get_error;


pub struct Window {
    p_window: *mut ll::SDL_Window,
}


impl Window {
    #[fixed_stack_segment]
    pub fn new(title: &str, x: int, y: int, w: uint, h: uint) -> Result<~Window, ~str> {
        let p = unsafe {
            do title.with_c_str |title| {
                ll::SDL_CreateWindow(title, x as c_int, y as c_int,
                                            w as c_int, h as c_int,
                                            ll::SDL_WINDOW_SHOWN)
            }
        };
        if ptr::is_null(p) {
            Err(get_error())
        } else {
            Ok(~Window{ p_window: p })
        }
    }

    #[fixed_stack_segment]
    pub fn size(&self) -> (uint, uint) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            ll::SDL_GetWindowSize(self.p_window,
                ptr::to_mut_unsafe_ptr(&mut w),
                ptr::to_mut_unsafe_ptr(&mut h));
        }
        (w as uint, h as uint)
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
