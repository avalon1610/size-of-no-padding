# Size Of No Padding
This crate provides the derive proc macro `SizeOfNoPadding`, it will generate `size_of_no_padding()` method for your struct or union which can calcuate the size of struct or union without padding even if it is not marked as `#[repr(packed)]`.

## Example
```rust
use size_of_no_padding::SizeOfNoPadding;

#[derive(SizeOfNoPadding)]
struct Abc {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    assert_eq!(8, std::mem::size_of::<Abc>());
    assert_eq!(6, Abc::size_of_no_padding());
}
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.