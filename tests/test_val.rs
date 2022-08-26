#[cfg(feature = "value_enum")]
use alt_enum::alt_val_enum;

#[cfg(feature = "value_enum")]
alt_val_enum!(
#[derive(Debug)]
test enm -> usize:
    first nya: 42,
    second-variant: 69,
);

#[cfg(feature = "value_enum")]
#[test]
fn test() {
    assert_eq!(usize::from(TestEnm::FirstNya), 42)
}
