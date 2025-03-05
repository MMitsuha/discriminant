#[macro_export]
macro_rules! impl_test_enum {
    ($repr:ident, $($variant:ident = $value:expr),+) => {
        paste::paste! {
            #[derive(Discriminant)]
            #[repr($repr)]
            #[allow(non_camel_case_types)]
            enum [<Test _ $repr>] {
                $($variant = $value),+
            }

            #[test]
            fn [<test _ $repr>]() {
                $(assert_eq!([<Test _ $repr>]::$variant.discriminant(), $value));+
            }
        }
    };
}
