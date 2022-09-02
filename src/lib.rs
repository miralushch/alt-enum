//! Various macros providing more beautiful syntax for enums.

pub use paste;

#[cfg(feature = "value_enum")]
pub use value_enum;

/// Macro providing a more beautiful syntax for enums.
/// 
/// # Examples
/// 
/// ```
/// use alt_enum::alt_enum;
/// 
/// alt_enum!(
/// #[derive(Debug)]
/// test enum:
///     first,
///     second-variant,
///     nyan nyan
/// );
/// 
/// assert_eq!(format!("{:?}", TestEnum::SecondVariant), "SecondVariant");
/// ```
#[macro_export]
macro_rules! alt_enum {
    (
        $(#[$attr:meta])*
        $vis:vis $($name:ident $(-)?)+:
            $($($variant:ident $(-)?)+),*
            $(,)?
    ) => {
        $crate::paste::paste! {
            $(#[$attr])*
            $vis enum [<$($name:camel) +>] {
                $([<$($variant:camel) +>]),*
            }
        }
    };
}

/// Macro providing a more beautiful syntax for enums with associated values.
/// 
/// (using value-enum crate, needs value_enum feature enabled)
/// 
/// # Examples
/// 
/// ```
/// use alt_enum::alt_val_enum;
/// 
/// alt_val_enum!(
/// #[derive(Debug)]
/// some nya -> &'static str:
///     first: "42",
///     second-variant: "meow",
///     nyan nyan: "nyaa~"
/// );
/// 
/// assert_eq!(<&str>::from(SomeNya::NyanNyan), "nyaa~");
/// ```
#[macro_export]
#[cfg(feature = "value_enum")]
macro_rules! alt_val_enum {
    (
        $(#[$attr:meta])*
        $vis:vis $($name:ident $(-)?)+ -> $type:ty:
            $($($variant:ident $(-)?)+: $value:literal),*
            $(,)?
    ) => {
        $crate::paste::paste! { $crate::value_enum::value_enum! {
            $(#[$attr])*
            $vis enum [<$($name:camel)+>]: $type {
                $([<$($variant:camel)+>] = $value),*
            }
        }}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    alt_enum!(
    #[derive(Debug)]
    test enm:
        test nya,
        second-variant,
    );

    #[test]
    fn test() {
        assert_eq!(format!("{:?}", TestEnm::TestNya), "TestNya");
        assert_eq!(format!("{:?}", TestEnm::SecondVariant), "SecondVariant");
    }
}

#[cfg(test)]
#[cfg(feature = "value_enum")]
mod val_tests {
    use super::*;

    alt_val_enum!(
    #[derive(Debug)]
    test enm -> &'static str:
        test nya: "nya",
        second-variant: "second",
    );

    #[test]
    fn test_val() {
        assert_eq!(<&str>::from(TestEnm::TestNya), "nya");
        assert_eq!(<&str>::from(TestEnm::SecondVariant), "second");
    }
}
