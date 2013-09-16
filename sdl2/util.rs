#[macro_escape];

#[macro_escape]
pub mod enum_set {
    extern mod extra;
    pub use self::extra::enum_set::*;
    use std::cast;

    pub trait EnumSetUtil<E> {
        fn to_uint(&self) -> uint;
        fn from_uint(v: uint) -> Self;
    }

    impl<E: CLike> EnumSetUtil<E> for EnumSet<E> {
        fn to_uint(&self) -> uint {
            unsafe { cast::transmute_copy(self) }
        }
        fn from_uint(v: uint) -> EnumSet<E> {
            unsafe { cast::transmute_copy(&v) }
        }
    }

    macro_rules! enum_set (($($elem:ident)|*) => ((||{
        let mut e = EnumSet::empty();
        $( e.add($elem); )*
        e
    })()))

    macro_rules! impl_clike (($T:ty) => (impl extra::enum_set::CLike for $T {
        fn to_uint(&self) -> uint { *self as uint }
        fn from_uint(value: uint) -> $T { unsafe { cast::transmute_copy(&value) } }
    }))

    #[cfg(test)]
    mod test {
        use super::*;

        #[deriving(Eq)]
        enum Fruit { Apple, Banana, Orange }
        impl CLike for Fruit {
            fn to_uint(&self) -> uint { *self as uint }
            fn from_uint(e: uint) -> Fruit {
                match e {
                    0 => Apple,
                    1 => Banana,
                    2 => Orange,
                    _ => fail!("Unknown index: %u", e),
                }
            }
        }

        #[test]
        fn test_enum_set_macro() {
            let e: EnumSet<Fruit> = enum_set!(Banana|Orange);
            assert!(e.iter().collect::<~[Fruit]>() == ~[Banana, Orange]);
        }

        #[test]
        fn test_to_uint() {
            let mut e = EnumSet::empty();
            e.add(Apple);
            e.add(Orange);
            assert!(e.to_uint() == 5u);
        }

        #[test]
        fn test_from_uint() {
            let e: EnumSet<Fruit> = EnumSetUtil::from_uint(3u);
            assert!(e.contains_elem(Apple));
            assert!(e.contains_elem(Banana));
            assert!(!e.contains_elem(Orange));
        }
    }
}
