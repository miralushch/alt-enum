# alt-enum

Various macros providing more beautiful syntax for enums.

## alt_enum

Macro providing a more beautiful syntax for enums.

### Example

```Rust
use alt_enum::alt_enum;
alt_enum!(
#[derive(Debug)]
test enum:
    first,
    second-variant,
    nyan nyan
);

assert_eq!(format!("{:?}", TestEnum::SecondVariant), "SecondVariant");
```

## alt_val_enum

Macro providing a more beautiful syntax for enums with associated values.

(using value-enum crate, needs value_enum feature enabled)

### Example

```Rust
use alt_enum::alt_val_enum;
 
alt_val_enum!(
#[derive(Debug)]
some nya -> &'static str:
    first: "42",
    second-variant: "meow",
    nyan nyan: "nyaa~"
);

assert_eq!(<&str>::from(SomeNya::NyanNyan), "nyaa~");
```
