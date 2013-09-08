use super::util::enum_set::*;
use std::cast;

pub enum Keycode {
    K_UNKNOWN = 0,
    K_RETURN = 13,
    K_ESCAPE = 27,
    K_BACKSPACE = 8,
    K_TAB = 9,
    K_SPACE = 32,
    K_EXCLAIM = 33,
    K_QUOTEDBL = 34,
    K_HASH = 35,
    K_PERCENT = 37,
    K_DOLLAR = 36,
    K_AMPERSAND = 38,
    K_QUOTE = 39,
    K_LEFTPAREN = 40,
    K_RIGHTPAREN = 41,
    K_ASTERISK = 42,
    K_PLUS = 43,
    K_COMMA = 44,
    K_MINUS = 45,
    K_PERIOD = 46,
    K_SLASH = 47,
    K_0 = 48,
    K_1 = 49,
    K_2 = 50,
    K_3 = 51,
    K_4 = 52,
    K_5 = 53,
    K_6 = 54,
    K_7 = 55,
    K_8 = 56,
    K_9 = 57,
    K_COLON = 58,
    K_SEMICOLON = 59,
    K_LESS = 60,
    K_EQUALS = 61,
    K_GREATER = 62,
    K_QUESTION = 63,
    K_AT = 64,
    K_LEFTBRACKET = 91,
    K_BACKSLASH = 92,
    K_RIGHTBRACKET = 93,
    K_CARET = 94,
    K_UNDERSCORE = 95,
    K_BACKQUOTE = 96,
    K_a = 97,
    K_b = 98,
    K_c = 99,
    K_d = 100,
    K_e = 101,
    K_f = 102,
    K_g = 103,
    K_h = 104,
    K_i = 105,
    K_j = 106,
    K_k = 107,
    K_l = 108,
    K_m = 109,
    K_n = 110,
    K_o = 111,
    K_p = 112,
    K_q = 113,
    K_r = 114,
    K_s = 115,
    K_t = 116,
    K_u = 117,
    K_v = 118,
    K_w = 119,
    K_x = 120,
    K_y = 121,
    K_z = 122,
    K_CAPSLOCK = 1073741881,
    K_F1 = 1073741882,
    K_F2 = 1073741883,
    K_F3 = 1073741884,
    K_F4 = 1073741885,
    K_F5 = 1073741886,
    K_F6 = 1073741887,
    K_F7 = 1073741888,
    K_F8 = 1073741889,
    K_F9 = 1073741890,
    K_F10 = 1073741891,
    K_F11 = 1073741892,
    K_F12 = 1073741893,
    K_PRINTSCREEN = 1073741894,
    K_SCROLLLOCK = 1073741895,
    K_PAUSE = 1073741896,
    K_INSERT = 1073741897,
    K_HOME = 1073741898,
    K_PAGEUP = 1073741899,
    K_DELETE = 127,
    K_END = 1073741901,
    K_PAGEDOWN = 1073741902,
    K_RIGHT = 1073741903,
    K_LEFT = 1073741904,
    K_DOWN = 1073741905,
    K_UP = 1073741906,
    K_NUMLOCKCLEAR = 1073741907,
    K_KP_DIVIDE = 1073741908,
    K_KP_MULTIPLY = 1073741909,
    K_KP_MINUS = 1073741910,
    K_KP_PLUS = 1073741911,
    K_KP_ENTER = 1073741912,
    K_KP_1 = 1073741913,
    K_KP_2 = 1073741914,
    K_KP_3 = 1073741915,
    K_KP_4 = 1073741916,
    K_KP_5 = 1073741917,
    K_KP_6 = 1073741918,
    K_KP_7 = 1073741919,
    K_KP_8 = 1073741920,
    K_KP_9 = 1073741921,
    K_KP_0 = 1073741922,
    K_KP_PERIOD = 1073741923,
    K_APPLICATION = 1073741925,
    K_POWER = 1073741926,
    K_KP_EQUALS = 1073741927,
    K_F13 = 1073741928,
    K_F14 = 1073741929,
    K_F15 = 1073741930,
    K_F16 = 1073741931,
    K_F17 = 1073741932,
    K_F18 = 1073741933,
    K_F19 = 1073741934,
    K_F20 = 1073741935,
    K_F21 = 1073741936,
    K_F22 = 1073741937,
    K_F23 = 1073741938,
    K_F24 = 1073741939,
    K_EXECUTE = 1073741940,
    K_HELP = 1073741941,
    K_MENU = 1073741942,
    K_SELECT = 1073741943,
    K_STOP = 1073741944,
    K_AGAIN = 1073741945,
    K_UNDO = 1073741946,
    K_CUT = 1073741947,
    K_COPY = 1073741948,
    K_PASTE = 1073741949,
    K_FIND = 1073741950,
    K_MUTE = 1073741951,
    K_VOLUMEUP = 1073741952,
    K_VOLUMEDOWN = 1073741953,
    K_KP_COMMA = 1073741957,
    K_KP_EQUALSAS400 = 1073741958,
    K_ALTERASE = 1073741977,
    K_SYSREQ = 1073741978,
    K_CANCEL = 1073741979,
    K_CLEAR = 1073741980,
    K_PRIOR = 1073741981,
    K_RETURN2 = 1073741982,
    K_SEPARATOR = 1073741983,
    K_OUT = 1073741984,
    K_OPER = 1073741985,
    K_CLEARAGAIN = 1073741986,
    K_CRSEL = 1073741987,
    K_EXSEL = 1073741988,
    K_KP_00 = 1073742000,
    K_KP_000 = 1073742001,
    K_THOUSANDSSEPARATOR = 1073742002,
    K_DECIMALSEPARATOR = 1073742003,
    K_CURRENCYUNIT = 1073742004,
    K_CURRENCYSUBUNIT = 1073742005,
    K_KP_LEFTPAREN = 1073742006,
    K_KP_RIGHTPAREN = 1073742007,
    K_KP_LEFTBRACE = 1073742008,
    K_KP_RIGHTBRACE = 1073742009,
    K_KP_TAB = 1073742010,
    K_KP_BACKSPACE = 1073742011,
    K_KP_A = 1073742012,
    K_KP_B = 1073742013,
    K_KP_C = 1073742014,
    K_KP_D = 1073742015,
    K_KP_E = 1073742016,
    K_KP_F = 1073742017,
    K_KP_XOR = 1073742018,
    K_KP_POWER = 1073742019,
    K_KP_PERCENT = 1073742020,
    K_KP_LESS = 1073742021,
    K_KP_GREATER = 1073742022,
    K_KP_AMPERSAND = 1073742023,
    K_KP_DBLAMPERSAND = 1073742024,
    K_KP_VERTICALBAR = 1073742025,
    K_KP_DBLVERTICALBAR = 1073742026,
    K_KP_COLON = 1073742027,
    K_KP_HASH = 1073742028,
    K_KP_SPACE = 1073742029,
    K_KP_AT = 1073742030,
    K_KP_EXCLAM = 1073742031,
    K_KP_MEMSTORE = 1073742032,
    K_KP_MEMRECALL = 1073742033,
    K_KP_MEMCLEAR = 1073742034,
    K_KP_MEMADD = 1073742035,
    K_KP_MEMSUBTRACT = 1073742036,
    K_KP_MEMMULTIPLY = 1073742037,
    K_KP_MEMDIVIDE = 1073742038,
    K_KP_PLUSMINUS = 1073742039,
    K_KP_CLEAR = 1073742040,
    K_KP_CLEARENTRY = 1073742041,
    K_KP_BINARY = 1073742042,
    K_KP_OCTAL = 1073742043,
    K_KP_DECIMAL = 1073742044,
    K_KP_HEXADECIMAL = 1073742045,
    K_LCTRL = 1073742048,
    K_LSHIFT = 1073742049,
    K_LALT = 1073742050,
    K_LGUI = 1073742051,
    K_RCTRL = 1073742052,
    K_RSHIFT = 1073742053,
    K_RALT = 1073742054,
    K_RGUI = 1073742055,
    K_MODE = 1073742081,
    K_AUDIONEXT = 1073742082,
    K_AUDIOPREV = 1073742083,
    K_AUDIOSTOP = 1073742084,
    K_AUDIOPLAY = 1073742085,
    K_AUDIOMUTE = 1073742086,
    K_MEDIASELECT = 1073742087,
    K_WWW = 1073742088,
    K_MAIL = 1073742089,
    K_CALCULATOR = 1073742090,
    K_COMPUTER = 1073742091,
    K_AC_SEARCH = 1073742092,
    K_AC_HOME = 1073742093,
    K_AC_BACK = 1073742094,
    K_AC_FORWARD = 1073742095,
    K_AC_STOP = 1073742096,
    K_AC_REFRESH = 1073742097,
    K_AC_BOOKMARKS = 1073742098,
    K_BRIGHTNESSDOWN = 1073742099,
    K_BRIGHTNESSUP = 1073742100,
    K_DISPLAYSWITCH = 1073742101,
    K_KBDILLUMTOGGLE = 1073742102,
    K_KBDILLUMDOWN = 1073742103,
    K_KBDILLUMUP = 1073742104,
    K_EJECT = 1073742105,
    K_SLEEP = 1073742106,
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
        assert_eq!(K_UNKNOWN as c_uint, ll::SDLK_UNKNOWN);
        assert_eq!(K_RETURN as c_uint, ll::SDLK_RETURN);
        assert_eq!(K_ESCAPE as c_uint, ll::SDLK_ESCAPE);
        assert_eq!(K_BACKSPACE as c_uint, ll::SDLK_BACKSPACE);
        assert_eq!(K_TAB as c_uint, ll::SDLK_TAB);
        assert_eq!(K_SPACE as c_uint, ll::SDLK_SPACE);
        assert_eq!(K_EXCLAIM as c_uint, ll::SDLK_EXCLAIM);
        assert_eq!(K_QUOTEDBL as c_uint, ll::SDLK_QUOTEDBL);
        assert_eq!(K_HASH as c_uint, ll::SDLK_HASH);
        assert_eq!(K_PERCENT as c_uint, ll::SDLK_PERCENT);
        assert_eq!(K_DOLLAR as c_uint, ll::SDLK_DOLLAR);
        assert_eq!(K_AMPERSAND as c_uint, ll::SDLK_AMPERSAND);
        assert_eq!(K_QUOTE as c_uint, ll::SDLK_QUOTE);
        assert_eq!(K_LEFTPAREN as c_uint, ll::SDLK_LEFTPAREN);
        assert_eq!(K_RIGHTPAREN as c_uint, ll::SDLK_RIGHTPAREN);
        assert_eq!(K_ASTERISK as c_uint, ll::SDLK_ASTERISK);
        assert_eq!(K_PLUS as c_uint, ll::SDLK_PLUS);
        assert_eq!(K_COMMA as c_uint, ll::SDLK_COMMA);
        assert_eq!(K_MINUS as c_uint, ll::SDLK_MINUS);
        assert_eq!(K_PERIOD as c_uint, ll::SDLK_PERIOD);
        assert_eq!(K_SLASH as c_uint, ll::SDLK_SLASH);
        assert_eq!(K_0 as c_uint, ll::SDLK_0);
        assert_eq!(K_1 as c_uint, ll::SDLK_1);
        assert_eq!(K_2 as c_uint, ll::SDLK_2);
        assert_eq!(K_3 as c_uint, ll::SDLK_3);
        assert_eq!(K_4 as c_uint, ll::SDLK_4);
        assert_eq!(K_5 as c_uint, ll::SDLK_5);
        assert_eq!(K_6 as c_uint, ll::SDLK_6);
        assert_eq!(K_7 as c_uint, ll::SDLK_7);
        assert_eq!(K_8 as c_uint, ll::SDLK_8);
        assert_eq!(K_9 as c_uint, ll::SDLK_9);
        assert_eq!(K_COLON as c_uint, ll::SDLK_COLON);
        assert_eq!(K_SEMICOLON as c_uint, ll::SDLK_SEMICOLON);
        assert_eq!(K_LESS as c_uint, ll::SDLK_LESS);
        assert_eq!(K_EQUALS as c_uint, ll::SDLK_EQUALS);
        assert_eq!(K_GREATER as c_uint, ll::SDLK_GREATER);
        assert_eq!(K_QUESTION as c_uint, ll::SDLK_QUESTION);
        assert_eq!(K_AT as c_uint, ll::SDLK_AT);
        assert_eq!(K_LEFTBRACKET as c_uint, ll::SDLK_LEFTBRACKET);
        assert_eq!(K_BACKSLASH as c_uint, ll::SDLK_BACKSLASH);
        assert_eq!(K_RIGHTBRACKET as c_uint, ll::SDLK_RIGHTBRACKET);
        assert_eq!(K_CARET as c_uint, ll::SDLK_CARET);
        assert_eq!(K_UNDERSCORE as c_uint, ll::SDLK_UNDERSCORE);
        assert_eq!(K_BACKQUOTE as c_uint, ll::SDLK_BACKQUOTE);
        assert_eq!(K_a as c_uint, ll::SDLK_a);
        assert_eq!(K_b as c_uint, ll::SDLK_b);
        assert_eq!(K_c as c_uint, ll::SDLK_c);
        assert_eq!(K_d as c_uint, ll::SDLK_d);
        assert_eq!(K_e as c_uint, ll::SDLK_e);
        assert_eq!(K_f as c_uint, ll::SDLK_f);
        assert_eq!(K_g as c_uint, ll::SDLK_g);
        assert_eq!(K_h as c_uint, ll::SDLK_h);
        assert_eq!(K_i as c_uint, ll::SDLK_i);
        assert_eq!(K_j as c_uint, ll::SDLK_j);
        assert_eq!(K_k as c_uint, ll::SDLK_k);
        assert_eq!(K_l as c_uint, ll::SDLK_l);
        assert_eq!(K_m as c_uint, ll::SDLK_m);
        assert_eq!(K_n as c_uint, ll::SDLK_n);
        assert_eq!(K_o as c_uint, ll::SDLK_o);
        assert_eq!(K_p as c_uint, ll::SDLK_p);
        assert_eq!(K_q as c_uint, ll::SDLK_q);
        assert_eq!(K_r as c_uint, ll::SDLK_r);
        assert_eq!(K_s as c_uint, ll::SDLK_s);
        assert_eq!(K_t as c_uint, ll::SDLK_t);
        assert_eq!(K_u as c_uint, ll::SDLK_u);
        assert_eq!(K_v as c_uint, ll::SDLK_v);
        assert_eq!(K_w as c_uint, ll::SDLK_w);
        assert_eq!(K_x as c_uint, ll::SDLK_x);
        assert_eq!(K_y as c_uint, ll::SDLK_y);
        assert_eq!(K_z as c_uint, ll::SDLK_z);
        assert_eq!(K_CAPSLOCK as c_uint, ll::SDLK_CAPSLOCK);
        assert_eq!(K_F1 as c_uint, ll::SDLK_F1);
        assert_eq!(K_F2 as c_uint, ll::SDLK_F2);
        assert_eq!(K_F3 as c_uint, ll::SDLK_F3);
        assert_eq!(K_F4 as c_uint, ll::SDLK_F4);
        assert_eq!(K_F5 as c_uint, ll::SDLK_F5);
        assert_eq!(K_F6 as c_uint, ll::SDLK_F6);
        assert_eq!(K_F7 as c_uint, ll::SDLK_F7);
        assert_eq!(K_F8 as c_uint, ll::SDLK_F8);
        assert_eq!(K_F9 as c_uint, ll::SDLK_F9);
        assert_eq!(K_F10 as c_uint, ll::SDLK_F10);
        assert_eq!(K_F11 as c_uint, ll::SDLK_F11);
        assert_eq!(K_F12 as c_uint, ll::SDLK_F12);
        assert_eq!(K_PRINTSCREEN as c_uint, ll::SDLK_PRINTSCREEN);
        assert_eq!(K_SCROLLLOCK as c_uint, ll::SDLK_SCROLLLOCK);
        assert_eq!(K_PAUSE as c_uint, ll::SDLK_PAUSE);
        assert_eq!(K_INSERT as c_uint, ll::SDLK_INSERT);
        assert_eq!(K_HOME as c_uint, ll::SDLK_HOME);
        assert_eq!(K_PAGEUP as c_uint, ll::SDLK_PAGEUP);
        assert_eq!(K_DELETE as c_uint, ll::SDLK_DELETE);
        assert_eq!(K_END as c_uint, ll::SDLK_END);
        assert_eq!(K_PAGEDOWN as c_uint, ll::SDLK_PAGEDOWN);
        assert_eq!(K_RIGHT as c_uint, ll::SDLK_RIGHT);
        assert_eq!(K_LEFT as c_uint, ll::SDLK_LEFT);
        assert_eq!(K_DOWN as c_uint, ll::SDLK_DOWN);
        assert_eq!(K_UP as c_uint, ll::SDLK_UP);
        assert_eq!(K_NUMLOCKCLEAR as c_uint, ll::SDLK_NUMLOCKCLEAR);
        assert_eq!(K_KP_DIVIDE as c_uint, ll::SDLK_KP_DIVIDE);
        assert_eq!(K_KP_MULTIPLY as c_uint, ll::SDLK_KP_MULTIPLY);
        assert_eq!(K_KP_MINUS as c_uint, ll::SDLK_KP_MINUS);
        assert_eq!(K_KP_PLUS as c_uint, ll::SDLK_KP_PLUS);
        assert_eq!(K_KP_ENTER as c_uint, ll::SDLK_KP_ENTER);
        assert_eq!(K_KP_1 as c_uint, ll::SDLK_KP_1);
        assert_eq!(K_KP_2 as c_uint, ll::SDLK_KP_2);
        assert_eq!(K_KP_3 as c_uint, ll::SDLK_KP_3);
        assert_eq!(K_KP_4 as c_uint, ll::SDLK_KP_4);
        assert_eq!(K_KP_5 as c_uint, ll::SDLK_KP_5);
        assert_eq!(K_KP_6 as c_uint, ll::SDLK_KP_6);
        assert_eq!(K_KP_7 as c_uint, ll::SDLK_KP_7);
        assert_eq!(K_KP_8 as c_uint, ll::SDLK_KP_8);
        assert_eq!(K_KP_9 as c_uint, ll::SDLK_KP_9);
        assert_eq!(K_KP_0 as c_uint, ll::SDLK_KP_0);
        assert_eq!(K_KP_PERIOD as c_uint, ll::SDLK_KP_PERIOD);
        assert_eq!(K_APPLICATION as c_uint, ll::SDLK_APPLICATION);
        assert_eq!(K_POWER as c_uint, ll::SDLK_POWER);
        assert_eq!(K_KP_EQUALS as c_uint, ll::SDLK_KP_EQUALS);
        assert_eq!(K_F13 as c_uint, ll::SDLK_F13);
        assert_eq!(K_F14 as c_uint, ll::SDLK_F14);
        assert_eq!(K_F15 as c_uint, ll::SDLK_F15);
        assert_eq!(K_F16 as c_uint, ll::SDLK_F16);
        assert_eq!(K_F17 as c_uint, ll::SDLK_F17);
        assert_eq!(K_F18 as c_uint, ll::SDLK_F18);
        assert_eq!(K_F19 as c_uint, ll::SDLK_F19);
        assert_eq!(K_F20 as c_uint, ll::SDLK_F20);
        assert_eq!(K_F21 as c_uint, ll::SDLK_F21);
        assert_eq!(K_F22 as c_uint, ll::SDLK_F22);
        assert_eq!(K_F23 as c_uint, ll::SDLK_F23);
        assert_eq!(K_F24 as c_uint, ll::SDLK_F24);
        assert_eq!(K_EXECUTE as c_uint, ll::SDLK_EXECUTE);
        assert_eq!(K_HELP as c_uint, ll::SDLK_HELP);
        assert_eq!(K_MENU as c_uint, ll::SDLK_MENU);
        assert_eq!(K_SELECT as c_uint, ll::SDLK_SELECT);
        assert_eq!(K_STOP as c_uint, ll::SDLK_STOP);
        assert_eq!(K_AGAIN as c_uint, ll::SDLK_AGAIN);
        assert_eq!(K_UNDO as c_uint, ll::SDLK_UNDO);
        assert_eq!(K_CUT as c_uint, ll::SDLK_CUT);
        assert_eq!(K_COPY as c_uint, ll::SDLK_COPY);
        assert_eq!(K_PASTE as c_uint, ll::SDLK_PASTE);
        assert_eq!(K_FIND as c_uint, ll::SDLK_FIND);
        assert_eq!(K_MUTE as c_uint, ll::SDLK_MUTE);
        assert_eq!(K_VOLUMEUP as c_uint, ll::SDLK_VOLUMEUP);
        assert_eq!(K_VOLUMEDOWN as c_uint, ll::SDLK_VOLUMEDOWN);
        assert_eq!(K_KP_COMMA as c_uint, ll::SDLK_KP_COMMA);
        assert_eq!(K_KP_EQUALSAS400 as c_uint, ll::SDLK_KP_EQUALSAS400);
        assert_eq!(K_ALTERASE as c_uint, ll::SDLK_ALTERASE);
        assert_eq!(K_SYSREQ as c_uint, ll::SDLK_SYSREQ);
        assert_eq!(K_CANCEL as c_uint, ll::SDLK_CANCEL);
        assert_eq!(K_CLEAR as c_uint, ll::SDLK_CLEAR);
        assert_eq!(K_PRIOR as c_uint, ll::SDLK_PRIOR);
        assert_eq!(K_RETURN2 as c_uint, ll::SDLK_RETURN2);
        assert_eq!(K_SEPARATOR as c_uint, ll::SDLK_SEPARATOR);
        assert_eq!(K_OUT as c_uint, ll::SDLK_OUT);
        assert_eq!(K_OPER as c_uint, ll::SDLK_OPER);
        assert_eq!(K_CLEARAGAIN as c_uint, ll::SDLK_CLEARAGAIN);
        assert_eq!(K_CRSEL as c_uint, ll::SDLK_CRSEL);
        assert_eq!(K_EXSEL as c_uint, ll::SDLK_EXSEL);
        assert_eq!(K_KP_00 as c_uint, ll::SDLK_KP_00);
        assert_eq!(K_KP_000 as c_uint, ll::SDLK_KP_000);
        assert_eq!(K_THOUSANDSSEPARATOR as c_uint, ll::SDLK_THOUSANDSSEPARATOR);
        assert_eq!(K_DECIMALSEPARATOR as c_uint, ll::SDLK_DECIMALSEPARATOR);
        assert_eq!(K_CURRENCYUNIT as c_uint, ll::SDLK_CURRENCYUNIT);
        assert_eq!(K_CURRENCYSUBUNIT as c_uint, ll::SDLK_CURRENCYSUBUNIT);
        assert_eq!(K_KP_LEFTPAREN as c_uint, ll::SDLK_KP_LEFTPAREN);
        assert_eq!(K_KP_RIGHTPAREN as c_uint, ll::SDLK_KP_RIGHTPAREN);
        assert_eq!(K_KP_LEFTBRACE as c_uint, ll::SDLK_KP_LEFTBRACE);
        assert_eq!(K_KP_RIGHTBRACE as c_uint, ll::SDLK_KP_RIGHTBRACE);
        assert_eq!(K_KP_TAB as c_uint, ll::SDLK_KP_TAB);
        assert_eq!(K_KP_BACKSPACE as c_uint, ll::SDLK_KP_BACKSPACE);
        assert_eq!(K_KP_A as c_uint, ll::SDLK_KP_A);
        assert_eq!(K_KP_B as c_uint, ll::SDLK_KP_B);
        assert_eq!(K_KP_C as c_uint, ll::SDLK_KP_C);
        assert_eq!(K_KP_D as c_uint, ll::SDLK_KP_D);
        assert_eq!(K_KP_E as c_uint, ll::SDLK_KP_E);
        assert_eq!(K_KP_F as c_uint, ll::SDLK_KP_F);
        assert_eq!(K_KP_XOR as c_uint, ll::SDLK_KP_XOR);
        assert_eq!(K_KP_POWER as c_uint, ll::SDLK_KP_POWER);
        assert_eq!(K_KP_PERCENT as c_uint, ll::SDLK_KP_PERCENT);
        assert_eq!(K_KP_LESS as c_uint, ll::SDLK_KP_LESS);
        assert_eq!(K_KP_GREATER as c_uint, ll::SDLK_KP_GREATER);
        assert_eq!(K_KP_AMPERSAND as c_uint, ll::SDLK_KP_AMPERSAND);
        assert_eq!(K_KP_DBLAMPERSAND as c_uint, ll::SDLK_KP_DBLAMPERSAND);
        assert_eq!(K_KP_VERTICALBAR as c_uint, ll::SDLK_KP_VERTICALBAR);
        assert_eq!(K_KP_DBLVERTICALBAR as c_uint, ll::SDLK_KP_DBLVERTICALBAR);
        assert_eq!(K_KP_COLON as c_uint, ll::SDLK_KP_COLON);
        assert_eq!(K_KP_HASH as c_uint, ll::SDLK_KP_HASH);
        assert_eq!(K_KP_SPACE as c_uint, ll::SDLK_KP_SPACE);
        assert_eq!(K_KP_AT as c_uint, ll::SDLK_KP_AT);
        assert_eq!(K_KP_EXCLAM as c_uint, ll::SDLK_KP_EXCLAM);
        assert_eq!(K_KP_MEMSTORE as c_uint, ll::SDLK_KP_MEMSTORE);
        assert_eq!(K_KP_MEMRECALL as c_uint, ll::SDLK_KP_MEMRECALL);
        assert_eq!(K_KP_MEMCLEAR as c_uint, ll::SDLK_KP_MEMCLEAR);
        assert_eq!(K_KP_MEMADD as c_uint, ll::SDLK_KP_MEMADD);
        assert_eq!(K_KP_MEMSUBTRACT as c_uint, ll::SDLK_KP_MEMSUBTRACT);
        assert_eq!(K_KP_MEMMULTIPLY as c_uint, ll::SDLK_KP_MEMMULTIPLY);
        assert_eq!(K_KP_MEMDIVIDE as c_uint, ll::SDLK_KP_MEMDIVIDE);
        assert_eq!(K_KP_PLUSMINUS as c_uint, ll::SDLK_KP_PLUSMINUS);
        assert_eq!(K_KP_CLEAR as c_uint, ll::SDLK_KP_CLEAR);
        assert_eq!(K_KP_CLEARENTRY as c_uint, ll::SDLK_KP_CLEARENTRY);
        assert_eq!(K_KP_BINARY as c_uint, ll::SDLK_KP_BINARY);
        assert_eq!(K_KP_OCTAL as c_uint, ll::SDLK_KP_OCTAL);
        assert_eq!(K_KP_DECIMAL as c_uint, ll::SDLK_KP_DECIMAL);
        assert_eq!(K_KP_HEXADECIMAL as c_uint, ll::SDLK_KP_HEXADECIMAL);
        assert_eq!(K_LCTRL as c_uint, ll::SDLK_LCTRL);
        assert_eq!(K_LSHIFT as c_uint, ll::SDLK_LSHIFT);
        assert_eq!(K_LALT as c_uint, ll::SDLK_LALT);
        assert_eq!(K_LGUI as c_uint, ll::SDLK_LGUI);
        assert_eq!(K_RCTRL as c_uint, ll::SDLK_RCTRL);
        assert_eq!(K_RSHIFT as c_uint, ll::SDLK_RSHIFT);
        assert_eq!(K_RALT as c_uint, ll::SDLK_RALT);
        assert_eq!(K_RGUI as c_uint, ll::SDLK_RGUI);
        assert_eq!(K_MODE as c_uint, ll::SDLK_MODE);
        assert_eq!(K_AUDIONEXT as c_uint, ll::SDLK_AUDIONEXT);
        assert_eq!(K_AUDIOPREV as c_uint, ll::SDLK_AUDIOPREV);
        assert_eq!(K_AUDIOSTOP as c_uint, ll::SDLK_AUDIOSTOP);
        assert_eq!(K_AUDIOPLAY as c_uint, ll::SDLK_AUDIOPLAY);
        assert_eq!(K_AUDIOMUTE as c_uint, ll::SDLK_AUDIOMUTE);
        assert_eq!(K_MEDIASELECT as c_uint, ll::SDLK_MEDIASELECT);
        assert_eq!(K_WWW as c_uint, ll::SDLK_WWW);
        assert_eq!(K_MAIL as c_uint, ll::SDLK_MAIL);
        assert_eq!(K_CALCULATOR as c_uint, ll::SDLK_CALCULATOR);
        assert_eq!(K_COMPUTER as c_uint, ll::SDLK_COMPUTER);
        assert_eq!(K_AC_SEARCH as c_uint, ll::SDLK_AC_SEARCH);
        assert_eq!(K_AC_HOME as c_uint, ll::SDLK_AC_HOME);
        assert_eq!(K_AC_BACK as c_uint, ll::SDLK_AC_BACK);
        assert_eq!(K_AC_FORWARD as c_uint, ll::SDLK_AC_FORWARD);
        assert_eq!(K_AC_STOP as c_uint, ll::SDLK_AC_STOP);
        assert_eq!(K_AC_REFRESH as c_uint, ll::SDLK_AC_REFRESH);
        assert_eq!(K_AC_BOOKMARKS as c_uint, ll::SDLK_AC_BOOKMARKS);
        assert_eq!(K_BRIGHTNESSDOWN as c_uint, ll::SDLK_BRIGHTNESSDOWN);
        assert_eq!(K_BRIGHTNESSUP as c_uint, ll::SDLK_BRIGHTNESSUP);
        assert_eq!(K_DISPLAYSWITCH as c_uint, ll::SDLK_DISPLAYSWITCH);
        assert_eq!(K_KBDILLUMTOGGLE as c_uint, ll::SDLK_KBDILLUMTOGGLE);
        assert_eq!(K_KBDILLUMDOWN as c_uint, ll::SDLK_KBDILLUMDOWN);
        assert_eq!(K_KBDILLUMUP as c_uint, ll::SDLK_KBDILLUMUP);
        assert_eq!(K_EJECT as c_uint, ll::SDLK_EJECT);
        assert_eq!(K_SLEEP as c_uint, ll::SDLK_SLEEP);
    }
}
