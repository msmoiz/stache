# Stache

This is a quick and dirty implementation of the Mustache template engine in Rust
thrown together in a few afternoons. It supports all core features outlined in
the [Mustache spec](https://github.com/mustache/spec), including interpolation,
escaping, comments, sections, nested context, and partials. It also includes
some convenience methods for constructing context objects and compiles using a
(mostly) zero-copy approach. For more full-featured (read: supported) libraries,
check out [Ramhorns](https://github.com/maciejhirsz/ramhorns) or
[rust-mustache](https://github.com/nickel-org/rust-mustache).

```rust
use stache::MapBuilder;
use stache::Template;

let text = "
    Hello {{name}}
    You have just won {{value}} dollars!
    {{#in_ca}}
    Well, {{taxed_value}} dollars, after taxes.
    {{/in_ca}}
";

let template = Template::compile(text).unwrap();

let context = MapBuilder::new()
    .str("name", "Mustafa")
    .int("value", 10000)
    .float("taxed_value", 10000.0 - (10000.0 * 0.4))
    .bool("in_ca", true)
    .build();

let rendered = template.render(context);

let expected = "
    Hello Mustafa
    You have just won 10000 dollars!
    Well, 6000 dollars, after taxes.
";

assert_eq!(rendered, expected);
```
