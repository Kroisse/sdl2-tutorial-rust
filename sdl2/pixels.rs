pub type Color = super::ll::Struct_SDL_Color;

pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
    super::ll::Struct_SDL_Color {r: r, g: g, b: b, a: a}
}
