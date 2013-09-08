pub enum Scancode {
    UNKNOWN = 0,

    /**
     *  \name Usage page 0x07
     *
     *  These values are from usage page 0x07 (USB keyboard page).
     */
    /*@{*/

    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    _1 = 30,
    _2 = 31,
    _3 = 32,
    _4 = 33,
    _5 = 34,
    _6 = 35,
    _7 = 36,
    _8 = 37,
    _9 = 38,
    _0 = 39,

    RETURN = 40,
    ESCAPE = 41,
    BACKSPACE = 42,
    TAB = 43,
    SPACE = 44,

    MINUS = 45,
    EQUALS = 46,
    LEFTBRACKET = 47,
    RIGHTBRACKET = 48,
    BACKSLASH = 49, /**< Located at the lower left of the return
                                  *   key on ISO keyboards and at the right end
                                  *   of the QWERTY row on ANSI keyboards.
                                  *   Produces REVERSE SOLIDUS (backslash) and
                                  *   VERTICAL LINE in a US layout, REVERSE
                                  *   SOLIDUS and VERTICAL LINE in a UK Mac
                                  *   layout, NUMBER SIGN and TILDE in a UK
                                  *   Windows layout, DOLLAR SIGN and POUND SIGN
                                  *   in a Swiss German layout, NUMBER SIGN and
                                  *   APOSTROPHE in a German layout, GRAVE
                                  *   ACCENT and POUND SIGN in a French Mac
                                  *   layout, and ASTERISK and MICRO SIGN in a
                                  *   French Windows layout.
                                  */
    NONUSHASH = 50, /**< ISO USB keyboards actually use this code
                                  *   instead of 49 for the same key, but all
                                  *   OSes I've seen treat the two codes
                                  *   identically. So, as an implementor, unless
                                  *   your keyboard generates both of those
                                  *   codes and your OS treats them differently,
                                  *   you should generate BACKSLASH
                                  *   instead of this code. As a user, you
                                  *   should not rely on this code because SDL
                                  *   will never generate it with most (all?)
                                  *   keyboards.
                                  */
    SEMICOLON = 51,
    APOSTROPHE = 52,
    GRAVE = 53, /**< Located in the top left corner (on both ANSI
                              *   and ISO keyboards). Produces GRAVE ACCENT and
                              *   TILDE in a US Windows layout and in US and UK
                              *   Mac layouts on ANSI keyboards, GRAVE ACCENT
                              *   and NOT SIGN in a UK Windows layout, SECTION
                              *   SIGN and PLUS-MINUS SIGN in US and UK Mac
                              *   layouts on ISO keyboards, SECTION SIGN and
                              *   DEGREE SIGN in a Swiss German layout (Mac:
                              *   only on ISO keyboards), CIRCUMFLEX ACCENT and
                              *   DEGREE SIGN in a German layout (Mac: only on
                              *   ISO keyboards), SUPERSCRIPT TWO and TILDE in a
                              *   French Windows layout, COMMERCIAL AT and
                              *   NUMBER SIGN in a French Mac layout on ISO
                              *   keyboards, and LESS-THAN SIGN and GREATER-THAN
                              *   SIGN in a Swiss German, German, or French Mac
                              *   layout on ANSI keyboards.
                              */
    COMMA = 54,
    PERIOD = 55,
    SLASH = 56,

    CAPSLOCK = 57,

    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,

    PRINTSCREEN = 70,
    SCROLLLOCK = 71,
    PAUSE = 72,
    INSERT = 73, /**< insert on PC, help on some Mac keyboards (but
                                   does send code 73, not 117) */
    HOME = 74,
    PAGEUP = 75,
    DELETE = 76,
    END = 77,
    PAGEDOWN = 78,
    RIGHT = 79,
    LEFT = 80,
    DOWN = 81,
    UP = 82,

    NUMLOCKCLEAR = 83, /**< num lock on PC, clear on Mac keyboards
                                     */
    KP_DIVIDE = 84,
    KP_MULTIPLY = 85,
    KP_MINUS = 86,
    KP_PLUS = 87,
    KP_ENTER = 88,
    KP_1 = 89,
    KP_2 = 90,
    KP_3 = 91,
    KP_4 = 92,
    KP_5 = 93,
    KP_6 = 94,
    KP_7 = 95,
    KP_8 = 96,
    KP_9 = 97,
    KP_0 = 98,
    KP_PERIOD = 99,

    NONUSBACKSLASH = 100, /**< This is the additional key that ISO
                                        *   keyboards have over ANSI ones,
                                        *   located between left shift and Y.
                                        *   Produces GRAVE ACCENT and TILDE in a
                                        *   US or UK Mac layout, REVERSE SOLIDUS
                                        *   (backslash) and VERTICAL LINE in a
                                        *   US or UK Windows layout, and
                                        *   LESS-THAN SIGN and GREATER-THAN SIGN
                                        *   in a Swiss German, German, or French
                                        *   layout. */
    APPLICATION = 101, /**< windows contextual menu, compose */
    POWER = 102, /**< The USB document says this is a status flag,
                               *   not a physical key - but some Mac keyboards
                               *   do have a power key. */
    KP_EQUALS = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    EXECUTE = 116,
    HELP = 117,
    MENU = 118,
    SELECT = 119,
    STOP = 120,
    AGAIN = 121,   /**< redo */
    UNDO = 122,
    CUT = 123,
    COPY = 124,
    PASTE = 125,
    FIND = 126,
    MUTE = 127,
    VOLUMEUP = 128,
    VOLUMEDOWN = 129,
/* not sure whether there's a reason to enable these */
/*     LOCKINGCAPSLOCK = 130,  */
/*     LOCKINGNUMLOCK = 131, */
/*     LOCKINGSCROLLLOCK = 132, */
    KP_COMMA = 133,
    KP_EQUALSAS400 = 134,

    INTERNATIONAL1 = 135, /**< used on Asian keyboards, see
                                            footnotes in USB doc */
    INTERNATIONAL2 = 136,
    INTERNATIONAL3 = 137, /**< Yen */
    INTERNATIONAL4 = 138,
    INTERNATIONAL5 = 139,
    INTERNATIONAL6 = 140,
    INTERNATIONAL7 = 141,
    INTERNATIONAL8 = 142,
    INTERNATIONAL9 = 143,
    LANG1 = 144, /**< Hangul/English toggle */
    LANG2 = 145, /**< Hanja conversion */
    LANG3 = 146, /**< Katakana */
    LANG4 = 147, /**< Hiragana */
    LANG5 = 148, /**< Zenkaku/Hankaku */
    LANG6 = 149, /**< reserved */
    LANG7 = 150, /**< reserved */
    LANG8 = 151, /**< reserved */
    LANG9 = 152, /**< reserved */

    ALTERASE = 153, /**< Erase-Eaze */
    SYSREQ = 154,
    CANCEL = 155,
    CLEAR = 156,
    PRIOR = 157,
    RETURN2 = 158,
    SEPARATOR = 159,
    OUT = 160,
    OPER = 161,
    CLEARAGAIN = 162,
    CRSEL = 163,
    EXSEL = 164,

    KP_00 = 176,
    KP_000 = 177,
    THOUSANDSSEPARATOR = 178,
    DECIMALSEPARATOR = 179,
    CURRENCYUNIT = 180,
    CURRENCYSUBUNIT = 181,
    KP_LEFTPAREN = 182,
    KP_RIGHTPAREN = 183,
    KP_LEFTBRACE = 184,
    KP_RIGHTBRACE = 185,
    KP_TAB = 186,
    KP_BACKSPACE = 187,
    KP_A = 188,
    KP_B = 189,
    KP_C = 190,
    KP_D = 191,
    KP_E = 192,
    KP_F = 193,
    KP_XOR = 194,
    KP_POWER = 195,
    KP_PERCENT = 196,
    KP_LESS = 197,
    KP_GREATER = 198,
    KP_AMPERSAND = 199,
    KP_DBLAMPERSAND = 200,
    KP_VERTICALBAR = 201,
    KP_DBLVERTICALBAR = 202,
    KP_COLON = 203,
    KP_HASH = 204,
    KP_SPACE = 205,
    KP_AT = 206,
    KP_EXCLAM = 207,
    KP_MEMSTORE = 208,
    KP_MEMRECALL = 209,
    KP_MEMCLEAR = 210,
    KP_MEMADD = 211,
    KP_MEMSUBTRACT = 212,
    KP_MEMMULTIPLY = 213,
    KP_MEMDIVIDE = 214,
    KP_PLUSMINUS = 215,
    KP_CLEAR = 216,
    KP_CLEARENTRY = 217,
    KP_BINARY = 218,
    KP_OCTAL = 219,
    KP_DECIMAL = 220,
    KP_HEXADECIMAL = 221,

    LCTRL = 224,
    LSHIFT = 225,
    LALT = 226, /**< alt, option */
    LGUI = 227, /**< windows, command (apple), meta */
    RCTRL = 228,
    RSHIFT = 229,
    RALT = 230, /**< alt gr, option */
    RGUI = 231, /**< windows, command (apple), meta */

    MODE = 257,    /**< I'm not sure if this is really not covered
                                 *   by any of the above, but since there's a
                                 *   special KMOD_MODE for it I'm adding it here
                                 */

    /*@}*//*Usage page 0x07*/

    /**
     *  \name Usage page 0x0C
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /*@{*/

    AUDIONEXT = 258,
    AUDIOPREV = 259,
    AUDIOSTOP = 260,
    AUDIOPLAY = 261,
    AUDIOMUTE = 262,
    MEDIASELECT = 263,
    WWW = 264,
    MAIL = 265,
    CALCULATOR = 266,
    COMPUTER = 267,
    AC_SEARCH = 268,
    AC_HOME = 269,
    AC_BACK = 270,
    AC_FORWARD = 271,
    AC_STOP = 272,
    AC_REFRESH = 273,
    AC_BOOKMARKS = 274,

    /*@}*//*Usage page 0x0C*/

    /**
     *  \name Walther keys
     *
     *  These are values that Christian Walther added (for mac keyboard?).
     */
    /*@{*/

    BRIGHTNESSDOWN = 275,
    BRIGHTNESSUP = 276,
    DISPLAYSWITCH = 277, /**< display mirroring/dual display
                                           switch, video mode switch */
    KBDILLUMTOGGLE = 278,
    KBDILLUMDOWN = 279,
    KBDILLUMUP = 280,
    EJECT = 281,
    SLEEP = 282,

    APP1 = 283,
    APP2 = 284,

    /*@}*//*Walther keys*/

    /* Add any other keys here. */

    /**< not a key, just marks the number of scancodes
         for array bounds */
    NUM_SCANCODES = 512,
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::ll;
    use std::libc::c_uint;

    #[test]
    fn test_scancodes() {
        assert_eq!(UNKNOWN as c_uint, ll::SDL_SCANCODE_UNKNOWN);
        assert_eq!(A as c_uint, ll::SDL_SCANCODE_A);
        assert_eq!(B as c_uint, ll::SDL_SCANCODE_B);
        assert_eq!(C as c_uint, ll::SDL_SCANCODE_C);
        assert_eq!(D as c_uint, ll::SDL_SCANCODE_D);
        assert_eq!(E as c_uint, ll::SDL_SCANCODE_E);
        assert_eq!(F as c_uint, ll::SDL_SCANCODE_F);
        assert_eq!(G as c_uint, ll::SDL_SCANCODE_G);
        assert_eq!(H as c_uint, ll::SDL_SCANCODE_H);
        assert_eq!(I as c_uint, ll::SDL_SCANCODE_I);
        assert_eq!(J as c_uint, ll::SDL_SCANCODE_J);
        assert_eq!(K as c_uint, ll::SDL_SCANCODE_K);
        assert_eq!(L as c_uint, ll::SDL_SCANCODE_L);
        assert_eq!(M as c_uint, ll::SDL_SCANCODE_M);
        assert_eq!(N as c_uint, ll::SDL_SCANCODE_N);
        assert_eq!(O as c_uint, ll::SDL_SCANCODE_O);
        assert_eq!(P as c_uint, ll::SDL_SCANCODE_P);
        assert_eq!(Q as c_uint, ll::SDL_SCANCODE_Q);
        assert_eq!(R as c_uint, ll::SDL_SCANCODE_R);
        assert_eq!(S as c_uint, ll::SDL_SCANCODE_S);
        assert_eq!(T as c_uint, ll::SDL_SCANCODE_T);
        assert_eq!(U as c_uint, ll::SDL_SCANCODE_U);
        assert_eq!(V as c_uint, ll::SDL_SCANCODE_V);
        assert_eq!(W as c_uint, ll::SDL_SCANCODE_W);
        assert_eq!(X as c_uint, ll::SDL_SCANCODE_X);
        assert_eq!(Y as c_uint, ll::SDL_SCANCODE_Y);
        assert_eq!(Z as c_uint, ll::SDL_SCANCODE_Z);
        assert_eq!(_1 as c_uint, ll::SDL_SCANCODE_1);
        assert_eq!(_2 as c_uint, ll::SDL_SCANCODE_2);
        assert_eq!(_3 as c_uint, ll::SDL_SCANCODE_3);
        assert_eq!(_4 as c_uint, ll::SDL_SCANCODE_4);
        assert_eq!(_5 as c_uint, ll::SDL_SCANCODE_5);
        assert_eq!(_6 as c_uint, ll::SDL_SCANCODE_6);
        assert_eq!(_7 as c_uint, ll::SDL_SCANCODE_7);
        assert_eq!(_8 as c_uint, ll::SDL_SCANCODE_8);
        assert_eq!(_9 as c_uint, ll::SDL_SCANCODE_9);
        assert_eq!(_0 as c_uint, ll::SDL_SCANCODE_0);
        assert_eq!(RETURN as c_uint, ll::SDL_SCANCODE_RETURN);
        assert_eq!(ESCAPE as c_uint, ll::SDL_SCANCODE_ESCAPE);
        assert_eq!(BACKSPACE as c_uint, ll::SDL_SCANCODE_BACKSPACE);
        assert_eq!(TAB as c_uint, ll::SDL_SCANCODE_TAB);
        assert_eq!(SPACE as c_uint, ll::SDL_SCANCODE_SPACE);
        assert_eq!(MINUS as c_uint, ll::SDL_SCANCODE_MINUS);
        assert_eq!(EQUALS as c_uint, ll::SDL_SCANCODE_EQUALS);
        assert_eq!(LEFTBRACKET as c_uint, ll::SDL_SCANCODE_LEFTBRACKET);
        assert_eq!(RIGHTBRACKET as c_uint, ll::SDL_SCANCODE_RIGHTBRACKET);
        assert_eq!(BACKSLASH as c_uint, ll::SDL_SCANCODE_BACKSLASH);
        assert_eq!(NONUSHASH as c_uint, ll::SDL_SCANCODE_NONUSHASH);
        assert_eq!(SEMICOLON as c_uint, ll::SDL_SCANCODE_SEMICOLON);
        assert_eq!(APOSTROPHE as c_uint, ll::SDL_SCANCODE_APOSTROPHE);
        assert_eq!(GRAVE as c_uint, ll::SDL_SCANCODE_GRAVE);
        assert_eq!(COMMA as c_uint, ll::SDL_SCANCODE_COMMA);
        assert_eq!(PERIOD as c_uint, ll::SDL_SCANCODE_PERIOD);
        assert_eq!(SLASH as c_uint, ll::SDL_SCANCODE_SLASH);
        assert_eq!(CAPSLOCK as c_uint, ll::SDL_SCANCODE_CAPSLOCK);
        assert_eq!(F1 as c_uint, ll::SDL_SCANCODE_F1);
        assert_eq!(F2 as c_uint, ll::SDL_SCANCODE_F2);
        assert_eq!(F3 as c_uint, ll::SDL_SCANCODE_F3);
        assert_eq!(F4 as c_uint, ll::SDL_SCANCODE_F4);
        assert_eq!(F5 as c_uint, ll::SDL_SCANCODE_F5);
        assert_eq!(F6 as c_uint, ll::SDL_SCANCODE_F6);
        assert_eq!(F7 as c_uint, ll::SDL_SCANCODE_F7);
        assert_eq!(F8 as c_uint, ll::SDL_SCANCODE_F8);
        assert_eq!(F9 as c_uint, ll::SDL_SCANCODE_F9);
        assert_eq!(F10 as c_uint, ll::SDL_SCANCODE_F10);
        assert_eq!(F11 as c_uint, ll::SDL_SCANCODE_F11);
        assert_eq!(F12 as c_uint, ll::SDL_SCANCODE_F12);
        assert_eq!(PRINTSCREEN as c_uint, ll::SDL_SCANCODE_PRINTSCREEN);
        assert_eq!(SCROLLLOCK as c_uint, ll::SDL_SCANCODE_SCROLLLOCK);
        assert_eq!(PAUSE as c_uint, ll::SDL_SCANCODE_PAUSE);
        assert_eq!(INSERT as c_uint, ll::SDL_SCANCODE_INSERT);
        assert_eq!(HOME as c_uint, ll::SDL_SCANCODE_HOME);
        assert_eq!(PAGEUP as c_uint, ll::SDL_SCANCODE_PAGEUP);
        assert_eq!(DELETE as c_uint, ll::SDL_SCANCODE_DELETE);
        assert_eq!(END as c_uint, ll::SDL_SCANCODE_END);
        assert_eq!(PAGEDOWN as c_uint, ll::SDL_SCANCODE_PAGEDOWN);
        assert_eq!(RIGHT as c_uint, ll::SDL_SCANCODE_RIGHT);
        assert_eq!(LEFT as c_uint, ll::SDL_SCANCODE_LEFT);
        assert_eq!(DOWN as c_uint, ll::SDL_SCANCODE_DOWN);
        assert_eq!(UP as c_uint, ll::SDL_SCANCODE_UP);
        assert_eq!(NUMLOCKCLEAR as c_uint, ll::SDL_SCANCODE_NUMLOCKCLEAR);
        assert_eq!(KP_DIVIDE as c_uint, ll::SDL_SCANCODE_KP_DIVIDE);
        assert_eq!(KP_MULTIPLY as c_uint, ll::SDL_SCANCODE_KP_MULTIPLY);
        assert_eq!(KP_MINUS as c_uint, ll::SDL_SCANCODE_KP_MINUS);
        assert_eq!(KP_PLUS as c_uint, ll::SDL_SCANCODE_KP_PLUS);
        assert_eq!(KP_ENTER as c_uint, ll::SDL_SCANCODE_KP_ENTER);
        assert_eq!(KP_1 as c_uint, ll::SDL_SCANCODE_KP_1);
        assert_eq!(KP_2 as c_uint, ll::SDL_SCANCODE_KP_2);
        assert_eq!(KP_3 as c_uint, ll::SDL_SCANCODE_KP_3);
        assert_eq!(KP_4 as c_uint, ll::SDL_SCANCODE_KP_4);
        assert_eq!(KP_5 as c_uint, ll::SDL_SCANCODE_KP_5);
        assert_eq!(KP_6 as c_uint, ll::SDL_SCANCODE_KP_6);
        assert_eq!(KP_7 as c_uint, ll::SDL_SCANCODE_KP_7);
        assert_eq!(KP_8 as c_uint, ll::SDL_SCANCODE_KP_8);
        assert_eq!(KP_9 as c_uint, ll::SDL_SCANCODE_KP_9);
        assert_eq!(KP_0 as c_uint, ll::SDL_SCANCODE_KP_0);
        assert_eq!(KP_PERIOD as c_uint, ll::SDL_SCANCODE_KP_PERIOD);
        assert_eq!(NONUSBACKSLASH as c_uint, ll::SDL_SCANCODE_NONUSBACKSLASH);
        assert_eq!(APPLICATION as c_uint, ll::SDL_SCANCODE_APPLICATION);
        assert_eq!(POWER as c_uint, ll::SDL_SCANCODE_POWER);
        assert_eq!(KP_EQUALS as c_uint, ll::SDL_SCANCODE_KP_EQUALS);
        assert_eq!(F13 as c_uint, ll::SDL_SCANCODE_F13);
        assert_eq!(F14 as c_uint, ll::SDL_SCANCODE_F14);
        assert_eq!(F15 as c_uint, ll::SDL_SCANCODE_F15);
        assert_eq!(F16 as c_uint, ll::SDL_SCANCODE_F16);
        assert_eq!(F17 as c_uint, ll::SDL_SCANCODE_F17);
        assert_eq!(F18 as c_uint, ll::SDL_SCANCODE_F18);
        assert_eq!(F19 as c_uint, ll::SDL_SCANCODE_F19);
        assert_eq!(F20 as c_uint, ll::SDL_SCANCODE_F20);
        assert_eq!(F21 as c_uint, ll::SDL_SCANCODE_F21);
        assert_eq!(F22 as c_uint, ll::SDL_SCANCODE_F22);
        assert_eq!(F23 as c_uint, ll::SDL_SCANCODE_F23);
        assert_eq!(F24 as c_uint, ll::SDL_SCANCODE_F24);
        assert_eq!(EXECUTE as c_uint, ll::SDL_SCANCODE_EXECUTE);
        assert_eq!(HELP as c_uint, ll::SDL_SCANCODE_HELP);
        assert_eq!(MENU as c_uint, ll::SDL_SCANCODE_MENU);
        assert_eq!(SELECT as c_uint, ll::SDL_SCANCODE_SELECT);
        assert_eq!(STOP as c_uint, ll::SDL_SCANCODE_STOP);
        assert_eq!(AGAIN as c_uint, ll::SDL_SCANCODE_AGAIN);
        assert_eq!(UNDO as c_uint, ll::SDL_SCANCODE_UNDO);
        assert_eq!(CUT as c_uint, ll::SDL_SCANCODE_CUT);
        assert_eq!(COPY as c_uint, ll::SDL_SCANCODE_COPY);
        assert_eq!(PASTE as c_uint, ll::SDL_SCANCODE_PASTE);
        assert_eq!(FIND as c_uint, ll::SDL_SCANCODE_FIND);
        assert_eq!(MUTE as c_uint, ll::SDL_SCANCODE_MUTE);
        assert_eq!(VOLUMEUP as c_uint, ll::SDL_SCANCODE_VOLUMEUP);
        assert_eq!(VOLUMEDOWN as c_uint, ll::SDL_SCANCODE_VOLUMEDOWN);
        assert_eq!(KP_COMMA as c_uint, ll::SDL_SCANCODE_KP_COMMA);
        assert_eq!(KP_EQUALSAS400 as c_uint, ll::SDL_SCANCODE_KP_EQUALSAS400);
        assert_eq!(INTERNATIONAL1 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL1);
        assert_eq!(INTERNATIONAL2 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL2);
        assert_eq!(INTERNATIONAL3 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL3);
        assert_eq!(INTERNATIONAL4 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL4);
        assert_eq!(INTERNATIONAL5 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL5);
        assert_eq!(INTERNATIONAL6 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL6);
        assert_eq!(INTERNATIONAL7 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL7);
        assert_eq!(INTERNATIONAL8 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL8);
        assert_eq!(INTERNATIONAL9 as c_uint, ll::SDL_SCANCODE_INTERNATIONAL9);
        assert_eq!(LANG1 as c_uint, ll::SDL_SCANCODE_LANG1);
        assert_eq!(LANG2 as c_uint, ll::SDL_SCANCODE_LANG2);
        assert_eq!(LANG3 as c_uint, ll::SDL_SCANCODE_LANG3);
        assert_eq!(LANG4 as c_uint, ll::SDL_SCANCODE_LANG4);
        assert_eq!(LANG5 as c_uint, ll::SDL_SCANCODE_LANG5);
        assert_eq!(LANG6 as c_uint, ll::SDL_SCANCODE_LANG6);
        assert_eq!(LANG7 as c_uint, ll::SDL_SCANCODE_LANG7);
        assert_eq!(LANG8 as c_uint, ll::SDL_SCANCODE_LANG8);
        assert_eq!(LANG9 as c_uint, ll::SDL_SCANCODE_LANG9);
        assert_eq!(ALTERASE as c_uint, ll::SDL_SCANCODE_ALTERASE);
        assert_eq!(SYSREQ as c_uint, ll::SDL_SCANCODE_SYSREQ);
        assert_eq!(CANCEL as c_uint, ll::SDL_SCANCODE_CANCEL);
        assert_eq!(CLEAR as c_uint, ll::SDL_SCANCODE_CLEAR);
        assert_eq!(PRIOR as c_uint, ll::SDL_SCANCODE_PRIOR);
        assert_eq!(RETURN2 as c_uint, ll::SDL_SCANCODE_RETURN2);
        assert_eq!(SEPARATOR as c_uint, ll::SDL_SCANCODE_SEPARATOR);
        assert_eq!(OUT as c_uint, ll::SDL_SCANCODE_OUT);
        assert_eq!(OPER as c_uint, ll::SDL_SCANCODE_OPER);
        assert_eq!(CLEARAGAIN as c_uint, ll::SDL_SCANCODE_CLEARAGAIN);
        assert_eq!(CRSEL as c_uint, ll::SDL_SCANCODE_CRSEL);
        assert_eq!(EXSEL as c_uint, ll::SDL_SCANCODE_EXSEL);
        assert_eq!(KP_00 as c_uint, ll::SDL_SCANCODE_KP_00);
        assert_eq!(KP_000 as c_uint, ll::SDL_SCANCODE_KP_000);
        assert_eq!(THOUSANDSSEPARATOR as c_uint, ll::SDL_SCANCODE_THOUSANDSSEPARATOR);
        assert_eq!(DECIMALSEPARATOR as c_uint, ll::SDL_SCANCODE_DECIMALSEPARATOR);
        assert_eq!(CURRENCYUNIT as c_uint, ll::SDL_SCANCODE_CURRENCYUNIT);
        assert_eq!(CURRENCYSUBUNIT as c_uint, ll::SDL_SCANCODE_CURRENCYSUBUNIT);
        assert_eq!(KP_LEFTPAREN as c_uint, ll::SDL_SCANCODE_KP_LEFTPAREN);
        assert_eq!(KP_RIGHTPAREN as c_uint, ll::SDL_SCANCODE_KP_RIGHTPAREN);
        assert_eq!(KP_LEFTBRACE as c_uint, ll::SDL_SCANCODE_KP_LEFTBRACE);
        assert_eq!(KP_RIGHTBRACE as c_uint, ll::SDL_SCANCODE_KP_RIGHTBRACE);
        assert_eq!(KP_TAB as c_uint, ll::SDL_SCANCODE_KP_TAB);
        assert_eq!(KP_BACKSPACE as c_uint, ll::SDL_SCANCODE_KP_BACKSPACE);
        assert_eq!(KP_A as c_uint, ll::SDL_SCANCODE_KP_A);
        assert_eq!(KP_B as c_uint, ll::SDL_SCANCODE_KP_B);
        assert_eq!(KP_C as c_uint, ll::SDL_SCANCODE_KP_C);
        assert_eq!(KP_D as c_uint, ll::SDL_SCANCODE_KP_D);
        assert_eq!(KP_E as c_uint, ll::SDL_SCANCODE_KP_E);
        assert_eq!(KP_F as c_uint, ll::SDL_SCANCODE_KP_F);
        assert_eq!(KP_XOR as c_uint, ll::SDL_SCANCODE_KP_XOR);
        assert_eq!(KP_POWER as c_uint, ll::SDL_SCANCODE_KP_POWER);
        assert_eq!(KP_PERCENT as c_uint, ll::SDL_SCANCODE_KP_PERCENT);
        assert_eq!(KP_LESS as c_uint, ll::SDL_SCANCODE_KP_LESS);
        assert_eq!(KP_GREATER as c_uint, ll::SDL_SCANCODE_KP_GREATER);
        assert_eq!(KP_AMPERSAND as c_uint, ll::SDL_SCANCODE_KP_AMPERSAND);
        assert_eq!(KP_DBLAMPERSAND as c_uint, ll::SDL_SCANCODE_KP_DBLAMPERSAND);
        assert_eq!(KP_VERTICALBAR as c_uint, ll::SDL_SCANCODE_KP_VERTICALBAR);
        assert_eq!(KP_DBLVERTICALBAR as c_uint, ll::SDL_SCANCODE_KP_DBLVERTICALBAR);
        assert_eq!(KP_COLON as c_uint, ll::SDL_SCANCODE_KP_COLON);
        assert_eq!(KP_HASH as c_uint, ll::SDL_SCANCODE_KP_HASH);
        assert_eq!(KP_SPACE as c_uint, ll::SDL_SCANCODE_KP_SPACE);
        assert_eq!(KP_AT as c_uint, ll::SDL_SCANCODE_KP_AT);
        assert_eq!(KP_EXCLAM as c_uint, ll::SDL_SCANCODE_KP_EXCLAM);
        assert_eq!(KP_MEMSTORE as c_uint, ll::SDL_SCANCODE_KP_MEMSTORE);
        assert_eq!(KP_MEMRECALL as c_uint, ll::SDL_SCANCODE_KP_MEMRECALL);
        assert_eq!(KP_MEMCLEAR as c_uint, ll::SDL_SCANCODE_KP_MEMCLEAR);
        assert_eq!(KP_MEMADD as c_uint, ll::SDL_SCANCODE_KP_MEMADD);
        assert_eq!(KP_MEMSUBTRACT as c_uint, ll::SDL_SCANCODE_KP_MEMSUBTRACT);
        assert_eq!(KP_MEMMULTIPLY as c_uint, ll::SDL_SCANCODE_KP_MEMMULTIPLY);
        assert_eq!(KP_MEMDIVIDE as c_uint, ll::SDL_SCANCODE_KP_MEMDIVIDE);
        assert_eq!(KP_PLUSMINUS as c_uint, ll::SDL_SCANCODE_KP_PLUSMINUS);
        assert_eq!(KP_CLEAR as c_uint, ll::SDL_SCANCODE_KP_CLEAR);
        assert_eq!(KP_CLEARENTRY as c_uint, ll::SDL_SCANCODE_KP_CLEARENTRY);
        assert_eq!(KP_BINARY as c_uint, ll::SDL_SCANCODE_KP_BINARY);
        assert_eq!(KP_OCTAL as c_uint, ll::SDL_SCANCODE_KP_OCTAL);
        assert_eq!(KP_DECIMAL as c_uint, ll::SDL_SCANCODE_KP_DECIMAL);
        assert_eq!(KP_HEXADECIMAL as c_uint, ll::SDL_SCANCODE_KP_HEXADECIMAL);
        assert_eq!(LCTRL as c_uint, ll::SDL_SCANCODE_LCTRL);
        assert_eq!(LSHIFT as c_uint, ll::SDL_SCANCODE_LSHIFT);
        assert_eq!(LALT as c_uint, ll::SDL_SCANCODE_LALT);
        assert_eq!(LGUI as c_uint, ll::SDL_SCANCODE_LGUI);
        assert_eq!(RCTRL as c_uint, ll::SDL_SCANCODE_RCTRL);
        assert_eq!(RSHIFT as c_uint, ll::SDL_SCANCODE_RSHIFT);
        assert_eq!(RALT as c_uint, ll::SDL_SCANCODE_RALT);
        assert_eq!(RGUI as c_uint, ll::SDL_SCANCODE_RGUI);
        assert_eq!(MODE as c_uint, ll::SDL_SCANCODE_MODE);
        assert_eq!(AUDIONEXT as c_uint, ll::SDL_SCANCODE_AUDIONEXT);
        assert_eq!(AUDIOPREV as c_uint, ll::SDL_SCANCODE_AUDIOPREV);
        assert_eq!(AUDIOSTOP as c_uint, ll::SDL_SCANCODE_AUDIOSTOP);
        assert_eq!(AUDIOPLAY as c_uint, ll::SDL_SCANCODE_AUDIOPLAY);
        assert_eq!(AUDIOMUTE as c_uint, ll::SDL_SCANCODE_AUDIOMUTE);
        assert_eq!(MEDIASELECT as c_uint, ll::SDL_SCANCODE_MEDIASELECT);
        assert_eq!(WWW as c_uint, ll::SDL_SCANCODE_WWW);
        assert_eq!(MAIL as c_uint, ll::SDL_SCANCODE_MAIL);
        assert_eq!(CALCULATOR as c_uint, ll::SDL_SCANCODE_CALCULATOR);
        assert_eq!(COMPUTER as c_uint, ll::SDL_SCANCODE_COMPUTER);
        assert_eq!(AC_SEARCH as c_uint, ll::SDL_SCANCODE_AC_SEARCH);
        assert_eq!(AC_HOME as c_uint, ll::SDL_SCANCODE_AC_HOME);
        assert_eq!(AC_BACK as c_uint, ll::SDL_SCANCODE_AC_BACK);
        assert_eq!(AC_FORWARD as c_uint, ll::SDL_SCANCODE_AC_FORWARD);
        assert_eq!(AC_STOP as c_uint, ll::SDL_SCANCODE_AC_STOP);
        assert_eq!(AC_REFRESH as c_uint, ll::SDL_SCANCODE_AC_REFRESH);
        assert_eq!(AC_BOOKMARKS as c_uint, ll::SDL_SCANCODE_AC_BOOKMARKS);
        assert_eq!(BRIGHTNESSDOWN as c_uint, ll::SDL_SCANCODE_BRIGHTNESSDOWN);
        assert_eq!(BRIGHTNESSUP as c_uint, ll::SDL_SCANCODE_BRIGHTNESSUP);
        assert_eq!(DISPLAYSWITCH as c_uint, ll::SDL_SCANCODE_DISPLAYSWITCH);
        assert_eq!(KBDILLUMTOGGLE as c_uint, ll::SDL_SCANCODE_KBDILLUMTOGGLE);
        assert_eq!(KBDILLUMDOWN as c_uint, ll::SDL_SCANCODE_KBDILLUMDOWN);
        assert_eq!(KBDILLUMUP as c_uint, ll::SDL_SCANCODE_KBDILLUMUP);
        assert_eq!(EJECT as c_uint, ll::SDL_SCANCODE_EJECT);
        assert_eq!(SLEEP as c_uint, ll::SDL_SCANCODE_SLEEP);
        assert_eq!(APP1 as c_uint, ll::SDL_SCANCODE_APP1);
        assert_eq!(APP2 as c_uint, ll::SDL_SCANCODE_APP2);
        assert_eq!(NUM_SCANCODES as c_uint, ll::SDL_NUM_SCANCODES);
    }
}