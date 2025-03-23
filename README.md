# size-of-no-padding

## Size Of No Padding
This crate provides two derive proc-macro.

### `SizeOfNoPadding`
> Only support struct can be marked as `Copy`

Create a shadow struct which as same as original struct except it's marked `#[repr(packed)]`. call `size_of_no_padding()` method on struct will invoke `std::mem::size_of()` function on shadow one in compile time.

#### Example
```rust
use size_of_no_padding::SizeOfNoPadding;
use size_of_no_padding::SizeOf;

#[derive(SizeOfNoPadding)]
struct Abc {
    a: u8,
    b: u32,
    c: u8,
}

assert_eq!(8, std::mem::size_of::<Abc>());
assert_eq!(6, Abc::size_of_no_padding());

```

#### `SizeOfNoPaddingAny`
> Support struct whose non-Copy fields impl [`SizeOfAny`] trait

Calculate the size for every field, add up all the size. call `size_of_no_padding(&self)` will give the size of whole struct without padding in runtime.

#### Example
```rust
use size_of_no_padding::SizeOfNoPaddingAny;
use size_of_no_padding::SizeOfAny;

#[derive(SizeOfNoPaddingAny)]
struct Abc2 {
    a: u8,
    b: u32,
    c: u8,
    d: Vec<u16>,
    e: Abc3,
}

#[derive(SizeOfNoPaddingAny)]
struct Abc3([u32; 4], u16);

let abc = Abc2 {
    a: 1,
    b: 2,
    c: 3,
    d: vec![4, 5, 6],
    e: Abc3([7, 8, 9, 10], 11),
};

assert_eq!(std::mem::size_of::<Abc2>(), 56);
assert_eq!(abc.size_of_no_padding_any(), 30);
```

### License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
