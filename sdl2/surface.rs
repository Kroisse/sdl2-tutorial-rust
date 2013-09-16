use std::ptr;

use super::ll;
use super::get_error;


pub struct Surface {
    p_surface: *mut ll::SDL_Surface,
}

impl Drop for Surface {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_FreeSurface(self.p_surface);
        }
    }
}

impl Surface {
    #[fixed_stack_segment]
    pub fn from_bmp(path: &Path) -> Result<~Surface, ~str> {
        unsafe {
            let mode = "rb".to_c_str();
            let p = do mode.with_ref |mode| {
                do path.with_c_str |file| {
                    ll::SDL_LoadBMP_RW(ll::SDL_RWFromFile(file, mode), 1)
                }
            };
            if ptr::is_null(p) {
                return Err(get_error());
            }
            Ok(~Surface {p_surface: p})
        }
    }
}
