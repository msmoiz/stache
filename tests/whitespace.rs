use indoc::indoc;
use stache::{Context, Result, Template};
use std::collections::HashMap;

#[test]
fn preserve_inner() -> Result<()> {
    let text = "Hello world";
    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::new());
    let rendered = template.render(context);
    let expected = "Hello world";
    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn preserve_indent() -> Result<()> {
    let text = "   Hello world";
    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::new());
    let rendered = template.render(context);
    let expected = "   Hello world";
    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn preserve_trailing() -> Result<()> {
    let text = "Hello world   ";
    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::new());
    let rendered = template.render(context);
    let expected = "Hello world   ";
    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn preserve_newlines() -> Result<()> {
    let text = indoc! {"
        Hello world



        Goodbye world
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::new());
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello world



        Goodbye world
    "};

    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn preserve_standalone_variable() -> Result<()> {
    let text = indoc! {"
        Hello world
           {{greeting}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([(
        String::from("greeting"),
        Context::String("Goodbye world".into()),
    )]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello world
           Goodbye world
    "};

    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn strip_standalone_special_tag() -> Result<()> {
    let text = indoc! {"
        Hello world
           {{! this is a comment}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::new());
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello world
    "};

    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn strip_standalone_special_tags_2() -> Result<()> {
    let text = indoc! {"
        {{#condition}}
        Hello world
        {{/condition}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([(
        String::from("condition"),
        Context::Bool(true),
    )]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello world
    "};

    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn strip_standalone_special_tags_3() -> Result<()> {
    let text = indoc! {"
        {{#condition}}
        {{#condition2}}
        Hello world
        {{/condition2}}
        {{/condition}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([
        (String::from("condition"), Context::Bool(true)),
        (String::from("condition2"), Context::Bool(true)),
    ]));
    let rendered = template.render(context);

    let expected = indoc! {"
        Hello world
    "};

    assert_eq!(rendered, expected);
    Ok(())
}

#[test]
fn preserve_multiple_special_tags() -> Result<()> {
    let text = indoc! {"
        {{#condition}}   {{/condition}}
    "};

    let template = Template::compile(text)?;
    let context = Context::Map(HashMap::from([(
        String::from("condition"),
        Context::Bool(true),
    )]));
    let rendered = template.render(context);

    let expected = "   \n";

    assert_eq!(rendered, expected);
    Ok(())
}
