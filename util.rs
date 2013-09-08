mod enum_set {
    extern mod extra;
    pub use self::extra::enum_set::*;
    pub use super::from_bytes::FromBytes;
    use std::to_bytes::ToBytes;
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

    impl<E: CLike> FromBytes for EnumSet<E> {
        fn from_bytes(bytes: &[u8], lsb0: bool) -> EnumSet<E> {
            fn proc<E: CLike>(e: &mut EnumSet<E>, byte: u8, k: uint) {
                for i in range(0u, 8u) {
                    if (1 << i) & byte != 0 {
                        let v = i + k * 8u;
                        e.add(CLike::from_uint(v));
                    }
                }
            }
            let mut e = EnumSet::<E>::empty();
            let mut k = 0;
            if lsb0 {
                for &i in bytes.rev_iter() {
                    proc(&mut e, i, k);
                    k += 1;
                }
            } else {
                for &i in bytes.iter() {
                    proc(&mut e, i, k);
                    k += 1;
                }
            }
            e
        }
    }

    macro_rules! enum_set (($($elem:ident)|*) => ((||{
        let mut e = EnumSet::empty();
        $( e.add($elem); )*
        e
    })()))

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
        fn test_from_bytes() {
            let e: EnumSet<Fruit> = FromBytes::from_bytes([5u8], true);
            assert!(e.contains_elem(Apple));
            assert!(!e.contains_elem(Banana));
            assert!(e.contains_elem(Orange));
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

mod from_bytes {
    use std::num::{Zero, NumCast};

    pub trait FromBytes {
        fn from_bytes(bytes: &[u8], lsb0: bool) -> Self;
    }
/*
    impl<N: Zero + Add<N, N> + Shl<N, N> + NumCast> FromBytes for N {
        fn from_bytes(bytes: &[u8], lsb0: bool) -> N {
            let mut result: N = Zero::zero();
            let byte_size: N = NumCast::from(8);
            if lsb0 {
                for &i in bytes.rev_iter() {
                    result = (result << byte_size) + NumCast::from(i);
                }
            } else {
                for &i in bytes.iter() {
                    result = (result << byte_size) + NumCast::from(i);
                }
            }
            result
        }
    }

    #[cfg(test)]
    mod test {
        macro_rules! num_test (($T: ident, $name: ident) => (mod $name {
            use std;
            use std::to_bytes::*;
            use super::super::FromBytes;

            #[test]
            fn full_test() {
                let ranges = [
                    (std::$T::min_value, std::$T::min_value / 2 - 1),
                    (std::$T::min_value / 2, -1),
                    (0, std::$T::max_value / 2 - 1),
                    (std::$T::max_value / 2, std::$T::max_value),
                ];
                for &(min, max) in ranges.iter() {
                    do spawn {
                        for i in range(min, max) {
                            let x: $T = FromBytes::from_bytes(i.to_bytes(true), true);
                            assert!(x == i);
                        }
                    }
                }
            }
        }))

        num_test!(i8, test_i8)
        num_test!(i16, test_i16)
        num_test!(u8, test_u8)
        num_test!(u16, test_u16)
    }*/
}