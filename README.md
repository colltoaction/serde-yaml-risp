# serde-yaml-risp [![Crate Version](https://img.shields.io/crates/v/serde-yaml-risp.svg)](https://crates.io/crates/serde-yaml-risp)

A YAML-based syntax crate for [risp] the rusty Lisp.

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
