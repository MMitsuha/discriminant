# Discriminant

Convert a enum (with or without field) with `#[repr(...)]` to corresponding type. Compatible with `#![no_std]`.

Tested on rust-2021 and rust-2024

## Examples

```rust
use discriminant::Discriminant;
#[derive(Discriminant)]
#[repr(i16)]
enum Test {
    One = 1,
    Two = 2,
    Four = 4,
}

fn test() {
    assert_eq!(Test::One.discriminant(), 1);
    assert_eq!(Test::Two.discriminant(), 2);
    assert_eq!(Test::Four.discriminant(), 4);
}
```

## Credits

I had been seen such an approach on `StackOverflow` long time ago, but I can not find the original post, to make it convenient for other developers dealing with lower level data struct, I made this crate. Thank for the author on `StackOverflow`.
