# 52-weeks-rust
#### With Noah Gift
Trying out rust and making it part of my personal practice.

## Trying to Learn Rust

Intent: every week, explore rust further and commit code.

Quick Example:

```rust
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceeans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

* [learn by example guide](https://doc.rust-lang.org/stable/rust-by-example/)
