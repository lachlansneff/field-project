
# field-project

Generic projection for all types! This crate adds projection support, through the `Project` trait and `proj!` macro
to all types in Rust. No derive required!

```rust
use std::pin::Pin;
use field_project::proj;

struct Foo {
    a: i32,
    b: &'static str,
}

fn main() {
    let foo = Box::pin(Foo { a: 42, b: "hello, world" });

    let a: Pin<_> = proj!(foo.a);
    let b = proj!(foo.b);

    println!("a: {:?}, b: {:?}", a, b);
}
```
