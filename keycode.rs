use super::util::enum_set::*;
use std::cast;
use super::scancode;

static SCANCODE_MASK: uint = 1u << 30u;
macro_rules! scancode_to_keycode (($e:expr) => ($e as uint | SCANCODE_MASK))

pub enum Keycode {
    UNKNOWN = 0,

    RETURN = '\r' as uint,
    ESCAPE = '\x1b' as uint,
    BACKSPACE = '\x08' as uint,
    TAB = '\t' as uint,
    SPACE = ' ' as uint,
    EXCLAIM = '!' as uint,
    QUOTEDBL = '"' as uint,
    HASH = '#' as uint,
    PERCENT = '%' as uint,
    DOLLAR = '$' as uint,
    AMPERSAND = '&' as uint,
    QUOTE = '\'' as uint,
    LEFTPAREN = '(' as uint,
    RIGHTPAREN = ')' as uint,
    ASTERISK = '*' as uint,
    PLUS = '+' as uint,
    COMMA = ',' as uint,
    MINUS = '-' as uint,
    PERIOD = '.' as uint,
    SLASH = '/' as uint,
    _0 = '0' as uint,
    _1 = '1' as uint,
    _2 = '2' as uint,
    _3 = '3' as uint,
    _4 = '4' as uint,
    _5 = '5' as uint,
    _6 = '6' as uint,
    _7 = '7' as uint,
    _8 = '8' as uint,
    _9 = '9' as uint,
    COLON = ':' as uint,
    SEMICOLON = ';' as uint,
    LESS = '<' as uint,
    EQUALS = '=' as uint,
    GREATER = '>' as uint,
    QUESTION = '?' as uint,
    AT = '@' as uint,
    /*
       Skip uppercase letters
     */
    LEFTBRACKET = '[' as uint,
    BACKSLASH = '\\' as uint,
    RIGHTBRACKET = ']' as uint,
    CARET = '^' as uint,
    UNDERSCORE = '_' as uint,
    BACKQUOTE = '`' as uint,
    a = 'a' as uint,
    b = 'b' as uint,
    c = 'c' as uint,
    d = 'd' as uint,
    e = 'e' as uint,
    f = 'f' as uint,
    g = 'g' as uint,
    h = 'h' as uint,
    i = 'i' as uint,
    j = 'j' as uint,
    k = 'k' as uint,
    l = 'l' as uint,
    m = 'm' as uint,
    n = 'n' as uint,
    o = 'o' as uint,
    p = 'p' as uint,
    q = 'q' as uint,
    r = 'r' as uint,
    s = 's' as uint,
    t = 't' as uint,
    u = 'u' as uint,
    v = 'v' as uint,
    w = 'w' as uint,
    x = 'x' as uint,
    y = 'y' as uint,
    z = 'z' as uint,

    CAPSLOCK = scancode_to_keycode!(scancode::CAPSLOCK),

    F1 = scancode_to_keycode!(scancode::F1),
    F2 = scancode_to_keycode!(scancode::F2),
    F3 = scancode_to_keycode!(scancode::F3),
    F4 = scancode_to_keycode!(scancode::F4),
    F5 = scancode_to_keycode!(scancode::F5),
    F6 = scancode_to_keycode!(scancode::F6),
    F7 = scancode_to_keycode!(scancode::F7),
    F8 = scancode_to_keycode!(scancode::F8),
    F9 = scancode_to_keycode!(scancode::F9),
    F10 = scancode_to_keycode!(scancode::F10),
    F11 = scancode_to_keycode!(scancode::F11),
    F12 = scancode_to_keycode!(scancode::F12),

    PRINTSCREEN = scancode_to_keycode!(scancode::PRINTSCREEN),
    SCROLLLOCK = scancode_to_keycode!(scancode::SCROLLLOCK),
    PAUSE = scancode_to_keycode!(scancode::PAUSE),
    INSERT = scancode_to_keycode!(scancode::INSERT),
    HOME = scancode_to_keycode!(scancode::HOME),
    PAGEUP = scancode_to_keycode!(scancode::PAGEUP),
    DELETE = '\x7f' as uint,
    END = scancode_to_keycode!(scancode::END),
    PAGEDOWN = scancode_to_keycode!(scancode::PAGEDOWN),
    RIGHT = scancode_to_keycode!(scancode::RIGHT),
    LEFT = scancode_to_keycode!(scancode::LEFT),
    DOWN = scancode_to_keycode!(scancode::DOWN),
    UP = scancode_to_keycode!(scancode::UP),

    NUMLOCKCLEAR = scancode_to_keycode!(scancode::NUMLOCKCLEAR),
    KP_DIVIDE = scancode_to_keycode!(scancode::KP_DIVIDE),
    KP_MULTIPLY = scancode_to_keycode!(scancode::KP_MULTIPLY),
    KP_MINUS = scancode_to_keycode!(scancode::KP_MINUS),
    KP_PLUS = scancode_to_keycode!(scancode::KP_PLUS),
    KP_ENTER = scancode_to_keycode!(scancode::KP_ENTER),
    KP_1 = scancode_to_keycode!(scancode::KP_1),
    KP_2 = scancode_to_keycode!(scancode::KP_2),
    KP_3 = scancode_to_keycode!(scancode::KP_3),
    KP_4 = scancode_to_keycode!(scancode::KP_4),
    KP_5 = scancode_to_keycode!(scancode::KP_5),
    KP_6 = scancode_to_keycode!(scancode::KP_6),
    KP_7 = scancode_to_keycode!(scancode::KP_7),
    KP_8 = scancode_to_keycode!(scancode::KP_8),
    KP_9 = scancode_to_keycode!(scancode::KP_9),
    KP_0 = scancode_to_keycode!(scancode::KP_0),
    KP_PERIOD = scancode_to_keycode!(scancode::KP_PERIOD),

    APPLICATION = scancode_to_keycode!(scancode::APPLICATION),
    POWER = scancode_to_keycode!(scancode::POWER),
    KP_EQUALS = scancode_to_keycode!(scancode::KP_EQUALS),
    F13 = scancode_to_keycode!(scancode::F13),
    F14 = scancode_to_keycode!(scancode::F14),
    F15 = scancode_to_keycode!(scancode::F15),
    F16 = scancode_to_keycode!(scancode::F16),
    F17 = scancode_to_keycode!(scancode::F17),
    F18 = scancode_to_keycode!(scancode::F18),
    F19 = scancode_to_keycode!(scancode::F19),
    F20 = scancode_to_keycode!(scancode::F20),
    F21 = scancode_to_keycode!(scancode::F21),
    F22 = scancode_to_keycode!(scancode::F22),
    F23 = scancode_to_keycode!(scancode::F23),
    F24 = scancode_to_keycode!(scancode::F24),
    EXECUTE = scancode_to_keycode!(scancode::EXECUTE),
    HELP = scancode_to_keycode!(scancode::HELP),
    MENU = scancode_to_keycode!(scancode::MENU),
    SELECT = scancode_to_keycode!(scancode::SELECT),
    STOP = scancode_to_keycode!(scancode::STOP),
    AGAIN = scancode_to_keycode!(scancode::AGAIN),
    UNDO = scancode_to_keycode!(scancode::UNDO),
    CUT = scancode_to_keycode!(scancode::CUT),
    COPY = scancode_to_keycode!(scancode::COPY),
    PASTE = scancode_to_keycode!(scancode::PASTE),
    FIND = scancode_to_keycode!(scancode::FIND),
    MUTE = scancode_to_keycode!(scancode::MUTE),
    VOLUMEUP = scancode_to_keycode!(scancode::VOLUMEUP),
    VOLUMEDOWN = scancode_to_keycode!(scancode::VOLUMEDOWN),
    KP_COMMA = scancode_to_keycode!(scancode::KP_COMMA),
    KP_EQUALSAS400 =
        scancode_to_keycode!(scancode::KP_EQUALSAS400),

    ALTERASE = scancode_to_keycode!(scancode::ALTERASE),
    SYSREQ = scancode_to_keycode!(scancode::SYSREQ),
    CANCEL = scancode_to_keycode!(scancode::CANCEL),
    CLEAR = scancode_to_keycode!(scancode::CLEAR),
    PRIOR = scancode_to_keycode!(scancode::PRIOR),
    RETURN2 = scancode_to_keycode!(scancode::RETURN2),
    SEPARATOR = scancode_to_keycode!(scancode::SEPARATOR),
    OUT = scancode_to_keycode!(scancode::OUT),
    OPER = scancode_to_keycode!(scancode::OPER),
    CLEARAGAIN = scancode_to_keycode!(scancode::CLEARAGAIN),
    CRSEL = scancode_to_keycode!(scancode::CRSEL),
    EXSEL = scancode_to_keycode!(scancode::EXSEL),

    KP_00 = scancode_to_keycode!(scancode::KP_00),
    KP_000 = scancode_to_keycode!(scancode::KP_000),
    THOUSANDSSEPARATOR =
        scancode_to_keycode!(scancode::THOUSANDSSEPARATOR),
    DECIMALSEPARATOR =
        scancode_to_keycode!(scancode::DECIMALSEPARATOR),
    CURRENCYUNIT = scancode_to_keycode!(scancode::CURRENCYUNIT),
    CURRENCYSUBUNIT =
        scancode_to_keycode!(scancode::CURRENCYSUBUNIT),
    KP_LEFTPAREN = scancode_to_keycode!(scancode::KP_LEFTPAREN),
    KP_RIGHTPAREN = scancode_to_keycode!(scancode::KP_RIGHTPAREN),
    KP_LEFTBRACE = scancode_to_keycode!(scancode::KP_LEFTBRACE),
    KP_RIGHTBRACE = scancode_to_keycode!(scancode::KP_RIGHTBRACE),
    KP_TAB = scancode_to_keycode!(scancode::KP_TAB),
    KP_BACKSPACE = scancode_to_keycode!(scancode::KP_BACKSPACE),
    KP_A = scancode_to_keycode!(scancode::KP_A),
    KP_B = scancode_to_keycode!(scancode::KP_B),
    KP_C = scancode_to_keycode!(scancode::KP_C),
    KP_D = scancode_to_keycode!(scancode::KP_D),
    KP_E = scancode_to_keycode!(scancode::KP_E),
    KP_F = scancode_to_keycode!(scancode::KP_F),
    KP_XOR = scancode_to_keycode!(scancode::KP_XOR),
    KP_POWER = scancode_to_keycode!(scancode::KP_POWER),
    KP_PERCENT = scancode_to_keycode!(scancode::KP_PERCENT),
    KP_LESS = scancode_to_keycode!(scancode::KP_LESS),
    KP_GREATER = scancode_to_keycode!(scancode::KP_GREATER),
    KP_AMPERSAND = scancode_to_keycode!(scancode::KP_AMPERSAND),
    KP_DBLAMPERSAND =
        scancode_to_keycode!(scancode::KP_DBLAMPERSAND),
    KP_VERTICALBAR =
        scancode_to_keycode!(scancode::KP_VERTICALBAR),
    KP_DBLVERTICALBAR =
        scancode_to_keycode!(scancode::KP_DBLVERTICALBAR),
    KP_COLON = scancode_to_keycode!(scancode::KP_COLON),
    KP_HASH = scancode_to_keycode!(scancode::KP_HASH),
    KP_SPACE = scancode_to_keycode!(scancode::KP_SPACE),
    KP_AT = scancode_to_keycode!(scancode::KP_AT),
    KP_EXCLAM = scancode_to_keycode!(scancode::KP_EXCLAM),
    KP_MEMSTORE = scancode_to_keycode!(scancode::KP_MEMSTORE),
    KP_MEMRECALL = scancode_to_keycode!(scancode::KP_MEMRECALL),
    KP_MEMCLEAR = scancode_to_keycode!(scancode::KP_MEMCLEAR),
    KP_MEMADD = scancode_to_keycode!(scancode::KP_MEMADD),
    KP_MEMSUBTRACT =
        scancode_to_keycode!(scancode::KP_MEMSUBTRACT),
    KP_MEMMULTIPLY =
        scancode_to_keycode!(scancode::KP_MEMMULTIPLY),
    KP_MEMDIVIDE = scancode_to_keycode!(scancode::KP_MEMDIVIDE),
    KP_PLUSMINUS = scancode_to_keycode!(scancode::KP_PLUSMINUS),
    KP_CLEAR = scancode_to_keycode!(scancode::KP_CLEAR),
    KP_CLEARENTRY = scancode_to_keycode!(scancode::KP_CLEARENTRY),
    KP_BINARY = scancode_to_keycode!(scancode::KP_BINARY),
    KP_OCTAL = scancode_to_keycode!(scancode::KP_OCTAL),
    KP_DECIMAL = scancode_to_keycode!(scancode::KP_DECIMAL),
    KP_HEXADECIMAL =
        scancode_to_keycode!(scancode::KP_HEXADECIMAL),

    LCTRL = scancode_to_keycode!(scancode::LCTRL),
    LSHIFT = scancode_to_keycode!(scancode::LSHIFT),
    LALT = scancode_to_keycode!(scancode::LALT),
    LGUI = scancode_to_keycode!(scancode::LGUI),
    RCTRL = scancode_to_keycode!(scancode::RCTRL),
    RSHIFT = scancode_to_keycode!(scancode::RSHIFT),
    RALT = scancode_to_keycode!(scancode::RALT),
    RGUI = scancode_to_keycode!(scancode::RGUI),

    MODE = scancode_to_keycode!(scancode::MODE),

    AUDIONEXT = scancode_to_keycode!(scancode::AUDIONEXT),
    AUDIOPREV = scancode_to_keycode!(scancode::AUDIOPREV),
    AUDIOSTOP = scancode_to_keycode!(scancode::AUDIOSTOP),
    AUDIOPLAY = scancode_to_keycode!(scancode::AUDIOPLAY),
    AUDIOMUTE = scancode_to_keycode!(scancode::AUDIOMUTE),
    MEDIASELECT = scancode_to_keycode!(scancode::MEDIASELECT),
    WWW = scancode_to_keycode!(scancode::WWW),
    MAIL = scancode_to_keycode!(scancode::MAIL),
    CALCULATOR = scancode_to_keycode!(scancode::CALCULATOR),
    COMPUTER = scancode_to_keycode!(scancode::COMPUTER),
    AC_SEARCH = scancode_to_keycode!(scancode::AC_SEARCH),
    AC_HOME = scancode_to_keycode!(scancode::AC_HOME),
    AC_BACK = scancode_to_keycode!(scancode::AC_BACK),
    AC_FORWARD = scancode_to_keycode!(scancode::AC_FORWARD),
    AC_STOP = scancode_to_keycode!(scancode::AC_STOP),
    AC_REFRESH = scancode_to_keycode!(scancode::AC_REFRESH),
    AC_BOOKMARKS = scancode_to_keycode!(scancode::AC_BOOKMARKS),

    BRIGHTNESSDOWN =
        scancode_to_keycode!(scancode::BRIGHTNESSDOWN),
    BRIGHTNESSUP = scancode_to_keycode!(scancode::BRIGHTNESSUP),
    DISPLAYSWITCH = scancode_to_keycode!(scancode::DISPLAYSWITCH),
    KBDILLUMTOGGLE =
        scancode_to_keycode!(scancode::KBDILLUMTOGGLE),
    KBDILLUMDOWN = scancode_to_keycode!(scancode::KBDILLUMDOWN),
    KBDILLUMUP = scancode_to_keycode!(scancode::KBDILLUMUP),
    EJECT = scancode_to_keycode!(scancode::EJECT),
    SLEEP = scancode_to_keycode!(scancode::SLEEP)
}

pub enum KeyModifier {
    ModLShift = 0,
    ModRShift = 1,
    ModLCtrl = 6,
    ModRCtrl = 7,
    ModLAlt = 8,
    ModRAlt = 9,
    ModLGUI = 10,
    ModRGUI = 11,
    ModNum = 12,
    ModCaps = 13,
    ModMode = 14,
}
impl_clike!(KeyModifier)

#[cfg(test)]
mod test {
    use super::*;
    use super::super::util::enum_set;
    use super::super::ll;
    use std::libc::c_uint;

    #[test]
    fn test_clike_keymodifiers() {
        assert_eq!(enum_set::bit(ModLShift), 0x0001);
        assert_eq!(enum_set::bit(ModRShift), 0x0002);
        assert_eq!(enum_set::bit(ModLCtrl), 0x0040);
        assert_eq!(enum_set::bit(ModRCtrl), 0x0080);
        assert_eq!(enum_set::bit(ModLAlt), 0x0100);
        assert_eq!(enum_set::bit(ModRAlt), 0x0200);
        assert_eq!(enum_set::bit(ModLGUI), 0x0400);
        assert_eq!(enum_set::bit(ModRGUI), 0x0800);
        assert_eq!(enum_set::bit(ModNum), 0x1000);
        assert_eq!(enum_set::bit(ModCaps), 0x2000);
        assert_eq!(enum_set::bit(ModMode), 0x4000);
    }

    #[test]
    fn test_keycodes() {
        assert_eq!(UNKNOWN as c_uint, ll::SDLK_UNKNOWN);
        assert_eq!(RETURN as c_uint, ll::SDLK_RETURN);
        assert_eq!(ESCAPE as c_uint, ll::SDLK_ESCAPE);
        assert_eq!(BACKSPACE as c_uint, ll::SDLK_BACKSPACE);
        assert_eq!(TAB as c_uint, ll::SDLK_TAB);
        assert_eq!(SPACE as c_uint, ll::SDLK_SPACE);
        assert_eq!(EXCLAIM as c_uint, ll::SDLK_EXCLAIM);
        assert_eq!(QUOTEDBL as c_uint, ll::SDLK_QUOTEDBL);
        assert_eq!(HASH as c_uint, ll::SDLK_HASH);
        assert_eq!(PERCENT as c_uint, ll::SDLK_PERCENT);
        assert_eq!(DOLLAR as c_uint, ll::SDLK_DOLLAR);
        assert_eq!(AMPERSAND as c_uint, ll::SDLK_AMPERSAND);
        assert_eq!(QUOTE as c_uint, ll::SDLK_QUOTE);
        assert_eq!(LEFTPAREN as c_uint, ll::SDLK_LEFTPAREN);
        assert_eq!(RIGHTPAREN as c_uint, ll::SDLK_RIGHTPAREN);
        assert_eq!(ASTERISK as c_uint, ll::SDLK_ASTERISK);
        assert_eq!(PLUS as c_uint, ll::SDLK_PLUS);
        assert_eq!(COMMA as c_uint, ll::SDLK_COMMA);
        assert_eq!(MINUS as c_uint, ll::SDLK_MINUS);
        assert_eq!(PERIOD as c_uint, ll::SDLK_PERIOD);
        assert_eq!(SLASH as c_uint, ll::SDLK_SLASH);
        assert_eq!(_0 as c_uint, ll::SDLK_0);
        assert_eq!(_1 as c_uint, ll::SDLK_1);
        assert_eq!(_2 as c_uint, ll::SDLK_2);
        assert_eq!(_3 as c_uint, ll::SDLK_3);
        assert_eq!(_4 as c_uint, ll::SDLK_4);
        assert_eq!(_5 as c_uint, ll::SDLK_5);
        assert_eq!(_6 as c_uint, ll::SDLK_6);
        assert_eq!(_7 as c_uint, ll::SDLK_7);
        assert_eq!(_8 as c_uint, ll::SDLK_8);
        assert_eq!(_9 as c_uint, ll::SDLK_9);
        assert_eq!(COLON as c_uint, ll::SDLK_COLON);
        assert_eq!(SEMICOLON as c_uint, ll::SDLK_SEMICOLON);
        assert_eq!(LESS as c_uint, ll::SDLK_LESS);
        assert_eq!(EQUALS as c_uint, ll::SDLK_EQUALS);
        assert_eq!(GREATER as c_uint, ll::SDLK_GREATER);
        assert_eq!(QUESTION as c_uint, ll::SDLK_QUESTION);
        assert_eq!(AT as c_uint, ll::SDLK_AT);
        assert_eq!(LEFTBRACKET as c_uint, ll::SDLK_LEFTBRACKET);
        assert_eq!(BACKSLASH as c_uint, ll::SDLK_BACKSLASH);
        assert_eq!(RIGHTBRACKET as c_uint, ll::SDLK_RIGHTBRACKET);
        assert_eq!(CARET as c_uint, ll::SDLK_CARET);
        assert_eq!(UNDERSCORE as c_uint, ll::SDLK_UNDERSCORE);
        assert_eq!(BACKQUOTE as c_uint, ll::SDLK_BACKQUOTE);
        assert_eq!(a as c_uint, ll::SDLK_a);
        assert_eq!(b as c_uint, ll::SDLK_b);
        assert_eq!(c as c_uint, ll::SDLK_c);
        assert_eq!(d as c_uint, ll::SDLK_d);
        assert_eq!(e as c_uint, ll::SDLK_e);
        assert_eq!(f as c_uint, ll::SDLK_f);
        assert_eq!(g as c_uint, ll::SDLK_g);
        assert_eq!(h as c_uint, ll::SDLK_h);
        assert_eq!(i as c_uint, ll::SDLK_i);
        assert_eq!(j as c_uint, ll::SDLK_j);
        assert_eq!(k as c_uint, ll::SDLK_k);
        assert_eq!(l as c_uint, ll::SDLK_l);
        assert_eq!(m as c_uint, ll::SDLK_m);
        assert_eq!(n as c_uint, ll::SDLK_n);
        assert_eq!(o as c_uint, ll::SDLK_o);
        assert_eq!(p as c_uint, ll::SDLK_p);
        assert_eq!(q as c_uint, ll::SDLK_q);
        assert_eq!(r as c_uint, ll::SDLK_r);
        assert_eq!(s as c_uint, ll::SDLK_s);
        assert_eq!(t as c_uint, ll::SDLK_t);
        assert_eq!(u as c_uint, ll::SDLK_u);
        assert_eq!(v as c_uint, ll::SDLK_v);
        assert_eq!(w as c_uint, ll::SDLK_w);
        assert_eq!(x as c_uint, ll::SDLK_x);
        assert_eq!(y as c_uint, ll::SDLK_y);
        assert_eq!(z as c_uint, ll::SDLK_z);
        assert_eq!(CAPSLOCK as c_uint, ll::SDLK_CAPSLOCK);
        assert_eq!(F1 as c_uint, ll::SDLK_F1);
        assert_eq!(F2 as c_uint, ll::SDLK_F2);
        assert_eq!(F3 as c_uint, ll::SDLK_F3);
        assert_eq!(F4 as c_uint, ll::SDLK_F4);
        assert_eq!(F5 as c_uint, ll::SDLK_F5);
        assert_eq!(F6 as c_uint, ll::SDLK_F6);
        assert_eq!(F7 as c_uint, ll::SDLK_F7);
        assert_eq!(F8 as c_uint, ll::SDLK_F8);
        assert_eq!(F9 as c_uint, ll::SDLK_F9);
        assert_eq!(F10 as c_uint, ll::SDLK_F10);
        assert_eq!(F11 as c_uint, ll::SDLK_F11);
        assert_eq!(F12 as c_uint, ll::SDLK_F12);
        assert_eq!(PRINTSCREEN as c_uint, ll::SDLK_PRINTSCREEN);
        assert_eq!(SCROLLLOCK as c_uint, ll::SDLK_SCROLLLOCK);
        assert_eq!(PAUSE as c_uint, ll::SDLK_PAUSE);
        assert_eq!(INSERT as c_uint, ll::SDLK_INSERT);
        assert_eq!(HOME as c_uint, ll::SDLK_HOME);
        assert_eq!(PAGEUP as c_uint, ll::SDLK_PAGEUP);
        assert_eq!(DELETE as c_uint, ll::SDLK_DELETE);
        assert_eq!(END as c_uint, ll::SDLK_END);
        assert_eq!(PAGEDOWN as c_uint, ll::SDLK_PAGEDOWN);
        assert_eq!(RIGHT as c_uint, ll::SDLK_RIGHT);
        assert_eq!(LEFT as c_uint, ll::SDLK_LEFT);
        assert_eq!(DOWN as c_uint, ll::SDLK_DOWN);
        assert_eq!(UP as c_uint, ll::SDLK_UP);
        assert_eq!(NUMLOCKCLEAR as c_uint, ll::SDLK_NUMLOCKCLEAR);
        assert_eq!(KP_DIVIDE as c_uint, ll::SDLK_KP_DIVIDE);
        assert_eq!(KP_MULTIPLY as c_uint, ll::SDLK_KP_MULTIPLY);
        assert_eq!(KP_MINUS as c_uint, ll::SDLK_KP_MINUS);
        assert_eq!(KP_PLUS as c_uint, ll::SDLK_KP_PLUS);
        assert_eq!(KP_ENTER as c_uint, ll::SDLK_KP_ENTER);
        assert_eq!(KP_1 as c_uint, ll::SDLK_KP_1);
        assert_eq!(KP_2 as c_uint, ll::SDLK_KP_2);
        assert_eq!(KP_3 as c_uint, ll::SDLK_KP_3);
        assert_eq!(KP_4 as c_uint, ll::SDLK_KP_4);
        assert_eq!(KP_5 as c_uint, ll::SDLK_KP_5);
        assert_eq!(KP_6 as c_uint, ll::SDLK_KP_6);
        assert_eq!(KP_7 as c_uint, ll::SDLK_KP_7);
        assert_eq!(KP_8 as c_uint, ll::SDLK_KP_8);
        assert_eq!(KP_9 as c_uint, ll::SDLK_KP_9);
        assert_eq!(KP_0 as c_uint, ll::SDLK_KP_0);
        assert_eq!(KP_PERIOD as c_uint, ll::SDLK_KP_PERIOD);
        assert_eq!(APPLICATION as c_uint, ll::SDLK_APPLICATION);
        assert_eq!(POWER as c_uint, ll::SDLK_POWER);
        assert_eq!(KP_EQUALS as c_uint, ll::SDLK_KP_EQUALS);
        assert_eq!(F13 as c_uint, ll::SDLK_F13);
        assert_eq!(F14 as c_uint, ll::SDLK_F14);
        assert_eq!(F15 as c_uint, ll::SDLK_F15);
        assert_eq!(F16 as c_uint, ll::SDLK_F16);
        assert_eq!(F17 as c_uint, ll::SDLK_F17);
        assert_eq!(F18 as c_uint, ll::SDLK_F18);
        assert_eq!(F19 as c_uint, ll::SDLK_F19);
        assert_eq!(F20 as c_uint, ll::SDLK_F20);
        assert_eq!(F21 as c_uint, ll::SDLK_F21);
        assert_eq!(F22 as c_uint, ll::SDLK_F22);
        assert_eq!(F23 as c_uint, ll::SDLK_F23);
        assert_eq!(F24 as c_uint, ll::SDLK_F24);
        assert_eq!(EXECUTE as c_uint, ll::SDLK_EXECUTE);
        assert_eq!(HELP as c_uint, ll::SDLK_HELP);
        assert_eq!(MENU as c_uint, ll::SDLK_MENU);
        assert_eq!(SELECT as c_uint, ll::SDLK_SELECT);
        assert_eq!(STOP as c_uint, ll::SDLK_STOP);
        assert_eq!(AGAIN as c_uint, ll::SDLK_AGAIN);
        assert_eq!(UNDO as c_uint, ll::SDLK_UNDO);
        assert_eq!(CUT as c_uint, ll::SDLK_CUT);
        assert_eq!(COPY as c_uint, ll::SDLK_COPY);
        assert_eq!(PASTE as c_uint, ll::SDLK_PASTE);
        assert_eq!(FIND as c_uint, ll::SDLK_FIND);
        assert_eq!(MUTE as c_uint, ll::SDLK_MUTE);
        assert_eq!(VOLUMEUP as c_uint, ll::SDLK_VOLUMEUP);
        assert_eq!(VOLUMEDOWN as c_uint, ll::SDLK_VOLUMEDOWN);
        assert_eq!(KP_COMMA as c_uint, ll::SDLK_KP_COMMA);
        assert_eq!(KP_EQUALSAS400 as c_uint, ll::SDLK_KP_EQUALSAS400);
        assert_eq!(ALTERASE as c_uint, ll::SDLK_ALTERASE);
        assert_eq!(SYSREQ as c_uint, ll::SDLK_SYSREQ);
        assert_eq!(CANCEL as c_uint, ll::SDLK_CANCEL);
        assert_eq!(CLEAR as c_uint, ll::SDLK_CLEAR);
        assert_eq!(PRIOR as c_uint, ll::SDLK_PRIOR);
        assert_eq!(RETURN2 as c_uint, ll::SDLK_RETURN2);
        assert_eq!(SEPARATOR as c_uint, ll::SDLK_SEPARATOR);
        assert_eq!(OUT as c_uint, ll::SDLK_OUT);
        assert_eq!(OPER as c_uint, ll::SDLK_OPER);
        assert_eq!(CLEARAGAIN as c_uint, ll::SDLK_CLEARAGAIN);
        assert_eq!(CRSEL as c_uint, ll::SDLK_CRSEL);
        assert_eq!(EXSEL as c_uint, ll::SDLK_EXSEL);
        assert_eq!(KP_00 as c_uint, ll::SDLK_KP_00);
        assert_eq!(KP_000 as c_uint, ll::SDLK_KP_000);
        assert_eq!(THOUSANDSSEPARATOR as c_uint, ll::SDLK_THOUSANDSSEPARATOR);
        assert_eq!(DECIMALSEPARATOR as c_uint, ll::SDLK_DECIMALSEPARATOR);
        assert_eq!(CURRENCYUNIT as c_uint, ll::SDLK_CURRENCYUNIT);
        assert_eq!(CURRENCYSUBUNIT as c_uint, ll::SDLK_CURRENCYSUBUNIT);
        assert_eq!(KP_LEFTPAREN as c_uint, ll::SDLK_KP_LEFTPAREN);
        assert_eq!(KP_RIGHTPAREN as c_uint, ll::SDLK_KP_RIGHTPAREN);
        assert_eq!(KP_LEFTBRACE as c_uint, ll::SDLK_KP_LEFTBRACE);
        assert_eq!(KP_RIGHTBRACE as c_uint, ll::SDLK_KP_RIGHTBRACE);
        assert_eq!(KP_TAB as c_uint, ll::SDLK_KP_TAB);
        assert_eq!(KP_BACKSPACE as c_uint, ll::SDLK_KP_BACKSPACE);
        assert_eq!(KP_A as c_uint, ll::SDLK_KP_A);
        assert_eq!(KP_B as c_uint, ll::SDLK_KP_B);
        assert_eq!(KP_C as c_uint, ll::SDLK_KP_C);
        assert_eq!(KP_D as c_uint, ll::SDLK_KP_D);
        assert_eq!(KP_E as c_uint, ll::SDLK_KP_E);
        assert_eq!(KP_F as c_uint, ll::SDLK_KP_F);
        assert_eq!(KP_XOR as c_uint, ll::SDLK_KP_XOR);
        assert_eq!(KP_POWER as c_uint, ll::SDLK_KP_POWER);
        assert_eq!(KP_PERCENT as c_uint, ll::SDLK_KP_PERCENT);
        assert_eq!(KP_LESS as c_uint, ll::SDLK_KP_LESS);
        assert_eq!(KP_GREATER as c_uint, ll::SDLK_KP_GREATER);
        assert_eq!(KP_AMPERSAND as c_uint, ll::SDLK_KP_AMPERSAND);
        assert_eq!(KP_DBLAMPERSAND as c_uint, ll::SDLK_KP_DBLAMPERSAND);
        assert_eq!(KP_VERTICALBAR as c_uint, ll::SDLK_KP_VERTICALBAR);
        assert_eq!(KP_DBLVERTICALBAR as c_uint, ll::SDLK_KP_DBLVERTICALBAR);
        assert_eq!(KP_COLON as c_uint, ll::SDLK_KP_COLON);
        assert_eq!(KP_HASH as c_uint, ll::SDLK_KP_HASH);
        assert_eq!(KP_SPACE as c_uint, ll::SDLK_KP_SPACE);
        assert_eq!(KP_AT as c_uint, ll::SDLK_KP_AT);
        assert_eq!(KP_EXCLAM as c_uint, ll::SDLK_KP_EXCLAM);
        assert_eq!(KP_MEMSTORE as c_uint, ll::SDLK_KP_MEMSTORE);
        assert_eq!(KP_MEMRECALL as c_uint, ll::SDLK_KP_MEMRECALL);
        assert_eq!(KP_MEMCLEAR as c_uint, ll::SDLK_KP_MEMCLEAR);
        assert_eq!(KP_MEMADD as c_uint, ll::SDLK_KP_MEMADD);
        assert_eq!(KP_MEMSUBTRACT as c_uint, ll::SDLK_KP_MEMSUBTRACT);
        assert_eq!(KP_MEMMULTIPLY as c_uint, ll::SDLK_KP_MEMMULTIPLY);
        assert_eq!(KP_MEMDIVIDE as c_uint, ll::SDLK_KP_MEMDIVIDE);
        assert_eq!(KP_PLUSMINUS as c_uint, ll::SDLK_KP_PLUSMINUS);
        assert_eq!(KP_CLEAR as c_uint, ll::SDLK_KP_CLEAR);
        assert_eq!(KP_CLEARENTRY as c_uint, ll::SDLK_KP_CLEARENTRY);
        assert_eq!(KP_BINARY as c_uint, ll::SDLK_KP_BINARY);
        assert_eq!(KP_OCTAL as c_uint, ll::SDLK_KP_OCTAL);
        assert_eq!(KP_DECIMAL as c_uint, ll::SDLK_KP_DECIMAL);
        assert_eq!(KP_HEXADECIMAL as c_uint, ll::SDLK_KP_HEXADECIMAL);
        assert_eq!(LCTRL as c_uint, ll::SDLK_LCTRL);
        assert_eq!(LSHIFT as c_uint, ll::SDLK_LSHIFT);
        assert_eq!(LALT as c_uint, ll::SDLK_LALT);
        assert_eq!(LGUI as c_uint, ll::SDLK_LGUI);
        assert_eq!(RCTRL as c_uint, ll::SDLK_RCTRL);
        assert_eq!(RSHIFT as c_uint, ll::SDLK_RSHIFT);
        assert_eq!(RALT as c_uint, ll::SDLK_RALT);
        assert_eq!(RGUI as c_uint, ll::SDLK_RGUI);
        assert_eq!(MODE as c_uint, ll::SDLK_MODE);
        assert_eq!(AUDIONEXT as c_uint, ll::SDLK_AUDIONEXT);
        assert_eq!(AUDIOPREV as c_uint, ll::SDLK_AUDIOPREV);
        assert_eq!(AUDIOSTOP as c_uint, ll::SDLK_AUDIOSTOP);
        assert_eq!(AUDIOPLAY as c_uint, ll::SDLK_AUDIOPLAY);
        assert_eq!(AUDIOMUTE as c_uint, ll::SDLK_AUDIOMUTE);
        assert_eq!(MEDIASELECT as c_uint, ll::SDLK_MEDIASELECT);
        assert_eq!(WWW as c_uint, ll::SDLK_WWW);
        assert_eq!(MAIL as c_uint, ll::SDLK_MAIL);
        assert_eq!(CALCULATOR as c_uint, ll::SDLK_CALCULATOR);
        assert_eq!(COMPUTER as c_uint, ll::SDLK_COMPUTER);
        assert_eq!(AC_SEARCH as c_uint, ll::SDLK_AC_SEARCH);
        assert_eq!(AC_HOME as c_uint, ll::SDLK_AC_HOME);
        assert_eq!(AC_BACK as c_uint, ll::SDLK_AC_BACK);
        assert_eq!(AC_FORWARD as c_uint, ll::SDLK_AC_FORWARD);
        assert_eq!(AC_STOP as c_uint, ll::SDLK_AC_STOP);
        assert_eq!(AC_REFRESH as c_uint, ll::SDLK_AC_REFRESH);
        assert_eq!(AC_BOOKMARKS as c_uint, ll::SDLK_AC_BOOKMARKS);
        assert_eq!(BRIGHTNESSDOWN as c_uint, ll::SDLK_BRIGHTNESSDOWN);
        assert_eq!(BRIGHTNESSUP as c_uint, ll::SDLK_BRIGHTNESSUP);
        assert_eq!(DISPLAYSWITCH as c_uint, ll::SDLK_DISPLAYSWITCH);
        assert_eq!(KBDILLUMTOGGLE as c_uint, ll::SDLK_KBDILLUMTOGGLE);
        assert_eq!(KBDILLUMDOWN as c_uint, ll::SDLK_KBDILLUMDOWN);
        assert_eq!(KBDILLUMUP as c_uint, ll::SDLK_KBDILLUMUP);
        assert_eq!(EJECT as c_uint, ll::SDLK_EJECT);
        assert_eq!(SLEEP as c_uint, ll::SDLK_SLEEP);
    }
}
