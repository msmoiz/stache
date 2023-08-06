#[test]
fn readme() {
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
}
