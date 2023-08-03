# Stache

This is a quick and dirty implementation of the Mustache template engine in Rust
thrown together in an afternoon. The process of compilation is mostly fleshed
out, but the rendering is lacking central features at the moment including
support for iteration, functions, and nested context. There are a number of
other odds and ends (like whitespace management around standalone tags) that
went beyond the scope of this exercise. For a more full-featured library, check
out [nickel-org/rust-mustache](https://github.com/nickel-org/rust-mustache).

```rust
use std::collections::HashMap;
use stache::{Context, Result, Template};

let text = "
    Hello {{name}}
    You have just won {{value}} dollars!
";

let template = Template::compile(text).unwrap();
let context = Context::Map(HashMap::from([
    ("name".into(), Context::String("Frodo".into())),
    ("value".into(), Context::Integer(10000)),
]));
let rendered = template.render(context);

let expected = "
    Hello Frodo
    You have just won 10000 dollars!
";

assert_eq!(rendered, expected);
```
