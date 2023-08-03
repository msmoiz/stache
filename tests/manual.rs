use indoc::indoc;
use std::collections::HashMap;

use stache::{Context, Result, Template};

#[test]
fn intro() -> Result<()> {
    let text = indoc! {"
        Hello {{name}}
        You have just won {{value}} dollars!
        {{#in_ca}}
        Well, {{taxed_value}} dollars, after taxes.
        {{/in_ca}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([
        ("name".into(), Context::String("Mustafa".into())),
        ("value".into(), Context::Integer(10000)),
        (
            "taxed_value".into(),
            Context::Float(10000.0 - (10000.0 * 0.4)),
        ),
        ("in_ca".into(), Context::Bool(true)),
    ]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello Mustafa
        You have just won 10000 dollars!
        Well, 6000 dollars, after taxes.
    "};

    assert_eq!(rendered, expected);

    Ok(())
}

#[test]
fn variables() -> Result<()> {
    let text = indoc! {"
        * {{name}}
        * {{age}}
        * {{company}}
        * {{{company}}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([
        ("name".into(), Context::String("Mustafa".into())),
        ("company".into(), Context::String("<b>GitHub</b>".into())),
    ]));
    let rendered = template.render(context);

    let expected = indoc! {"
        * Mustafa
        * 
        * &lt;b&gt;GitHub&lt;/b&gt;
        * <b>GitHub</b>
    "};

    assert_eq!(rendered, expected);

    Ok(())
}

#[test]
fn section_false() -> Result<()> {
    let text = indoc! {"
        Shown.
        {{#person}}
        Never shown!
        {{/person}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([("person".into(), Context::Bool(false))]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Shown.
    "};

    assert_eq!(rendered, expected);

    Ok(())
}

#[test]
fn section_empty_list() -> Result<()> {
    let text = indoc! {"
        Shown.
        {{#person}}
        Never shown!
        {{/person}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([("person".into(), Context::List(vec![]))]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Shown.
    "};

    assert_eq!(rendered, expected);

    Ok(())
}
