# serde-yaml-risp [![Crate Version](https://img.shields.io/crates/v/serde-yaml-risp.svg)](https://crates.io/crates/serde-yaml-risp)

A YAML-based syntax crate for [risp] the rusty Lisp.

## Usage in Rust

### Add dependency

```toml
[dependencies]
risp = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.8"
serde-yaml-risp = "0.1"
```

### Using the library

```rs
extern crate risp;

use risp::core::create_core_environment;

use risp::eval::eval;
use serde_yaml::Value;

fn main() {
    let input = "[ def, my_int, 2 ]";
    let value = serde_yaml::from_str::<Value>(input).unwrap();
    let file_risp = serde_yaml_risp::convert(&value);
    let mut env = create_core_environment();

    let r0 = eval(file_risp, &mut env);
    println!("{:?}", r0);
}
```

## Running example

```sh
cargo run --example
```

## Conceptual model

```
[ YAML documents ]
       | serde
       ↓
[ serde_yaml::Value ]
       | serde-yaml-risp
       ↓
[ risp::types::RispType ]
       | risp
       ↓
[ LISP programs ]
```



[risp]: https://crates.io/crates/risp
