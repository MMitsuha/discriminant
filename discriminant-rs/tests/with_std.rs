#[macro_use]
mod common;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use discriminant_rs::Discriminant;

    impl_test_enum!(i8, One = 1, Two = 2, Four = 4);
    impl_test_enum!(i16, One = 1, Two = 2, Four = 4);
    impl_test_enum!(i32, One = 1, Two = 2, Four = 4);
    impl_test_enum!(i64, One = 1, Two = 2, Four = 4);
    impl_test_enum!(u8, One = 1, Two = 2, Four = 4);
    impl_test_enum!(u16, One = 1, Two = 2, Four = 4);
    impl_test_enum!(u32, One = 1, Two = 2, Four = 4);
    impl_test_enum!(u64, One = 1, Two = 2, Four = 4);

    #[derive(Discriminant)]
    #[repr(i16)]
    #[allow(dead_code)]
    enum TestWithField {
        One(String) = 1,
        Two(&'static str) = 2,
    }

    #[test]
    fn test_field() {
        assert_eq!(
            TestWithField::One(String::from_str("hello world").unwrap()).discriminant(),
            1
        );
        assert_eq!(TestWithField::Two("hello world").discriminant(), 2);
    }
}
