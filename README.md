# random_struct_layout

This crate provides custom attributes to randomize struct layout, like gcc's Randomizing structure layout(https://lwn.net/Articles/722293/).

# Example

```rust
use offset;
use random_struct_layout::layout_randomize;

#[macro_use]
extern crate random_struct_layout;

#[layout_randomize(Debug)]
struct Data {
    a: i32,
    b: i32,
    c: i32,
    d: i64,
    e: i64,
}

fn main() {
    let data = Data {
        a: 0x10,
        b: 0x20,
        c: 0x30,
        d: 0x30,
        e: 0x40,
    };

    println!("{}", offset::offset_of!(Data::a));
    println!("{}", offset::offset_of!(Data::b));
    println!("{}", offset::offset_of!(Data::c));
    println!("{}", offset::offset_of!(Data::d));
    println!("{}", offset::offset_of!(Data::e));

    /* example output
    20
    16
    0
    8
    24
    */

    println!("{:x?}", data); // Debug print order is same as normal one.
                             // Data { a: 10, b: 20, c: 30, d: 30, e: 40 }
}

#[layout_randomize]
struct HasDst {
    value1: i32,
    value2: i32,

    #[dst]
    value3: [u8], // support dst type.
}
```
