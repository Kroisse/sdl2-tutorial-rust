#![macro_escape]

pub mod enum_set {
    #![macro_escape]

    extern crate collections;
    pub use collections::enum_set::{EnumSet, Items, CLike};
    use std::mem;

    pub trait EnumSetUtil<E> {
        fn to_uint(&self) -> uint;
        fn from_uint(v: uint) -> Self;
    }

    impl<E: CLike> EnumSetUtil<E> for EnumSet<E> {
        fn to_uint(&self) -> uint {
            unsafe { mem::transmute_copy(self) }
        }
        fn from_uint(v: uint) -> EnumSet<E> {
            unsafe { mem::transmute_copy(&v) }
        }
    }

    macro_rules! enum_set (($($elem:ident)|*) => ((||{
        let mut e = EnumSet::empty();
        $( e.add($elem); )*
        e
    })()))

    macro_rules! impl_clike (($T:ty) => (impl collections::enum_set::CLike for $T {
        fn to_uint(&self) -> uint { *self as uint }
        fn from_uint(value: uint) -> $T { FromPrimitive::from_uint(value).unwrap() }
    }))

    #[cfg(test)]
    pub fn bit<E: CLike>(e: E) -> uint {
        1 << e.to_uint()
    }

    #[cfg(test)]
    mod test {
        use super::{EnumSet, EnumSetUtil, CLike};

        #[deriving(Eq, FromPrimitive)]
        enum Fruit { Apple, Banana, Orange }
        impl CLike for Fruit {
            fn to_uint(&self) -> uint { *self as uint }
            fn from_uint(e: uint) -> Fruit { FromPrimitive::from_uint(e).unwrap() }
        }

        #[test]
        fn test_enum_set_macro() {
            let e: EnumSet<Fruit> = enum_set!(Banana|Orange);
            assert!(e.iter().collect::<Vec<_>>().as_slice() == [Banana, Orange]);
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
