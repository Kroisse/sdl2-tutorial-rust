use super::ll;
use super::get_error;


pub struct Surface {
    pub p_surface: *mut ll::SDL_Surface,
}

impl Drop for Surface {
        fn drop(&mut self) {
        unsafe {
            ll::SDL_FreeSurface(self.p_surface);
        }
    }
}

impl Surface {
        pub fn from_bmp(path: &Path) -> Result<Surface, StrBuf> {
        unsafe {
            let mode = "rb".to_c_str();
            let p = mode.with_ref(|mode| {
                path.with_c_str(|file| {
                    ll::SDL_LoadBMP_RW(ll::SDL_RWFromFile(file, mode), 1)
                })
            });
            if p.is_null() {
                return Err(get_error());
            }
            Ok(Surface {p_surface: p})
        }
    }
}
