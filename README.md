# random_struct_layout

This crate provides custom attributes to randomize struct members layout.

# Example

```rust
use random_struct_layout::layout_randomize;

#[layout_randomize]
#[derive(Debug)]
struct Data {
    a: i32,
    b: i32,
    c: i32,
    d: i64,
    e: i64,
}

fn main() {
    let d = Data {
        a: 0x10,
        b: 0x20,
        c: 0x30,
        d: 0x30,
        e: 0x40,
    };

    println!("{:x?}", d);

    // type punning is not defined behivair
    let raw_memory_slice =
        unsafe { core::slice::from_raw_parts(&d as *const _ as *const u8, 0x20) };

    println!("{:x?}", raw_memory_slice); // execution result will vary each time.
                                         // cargo clean -p this_crate && cargo run
}
```
