# Stache

This is a quick and dirty implementation of the Mustache template engine in Rust
thrown together in an afternoon. The process of compilation is mostly fleshed
out, but the rendering is lacking central features at the moment including
support for iteration, functions, and nested context. There are a number of
other odds and ends (like whitespace management within tags) that went beyond
the scope of this exercise. For a more full-featured library, check out
[Ramhorns](https://github.com/maciejhirsz/ramhorns).

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
