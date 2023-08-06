mod macros;

use std::collections::HashMap;

mstest!(
    intro,
    Context::Map(HashMap::from([
        ("name".into(), Context::String("Mustafa".into())),
        ("value".into(), Context::Integer(10000)),
        (
            "taxed_value".into(),
            Context::Float(10000.0 - (10000.0 * 0.4)),
        ),
        ("in_ca".into(), Context::Bool(true)),
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello {{name}}
    You have just won {{value}} dollars!
    {{#in_ca}}
    Well, {{taxed_value}} dollars, after taxes.
    {{/in_ca}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello Mustafa
    You have just won 10000 dollars!
    Well, 6000 dollars, after taxes.
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    variables,
    Context::Map(HashMap::from([
        ("name".into(), Context::String("Mustafa".into())),
        ("company".into(), Context::String("<b>GitHub</b>".into())),
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * {{name}}
    * {{age}}
    * {{company}}
    * {{{company}}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * Mustafa
    * 
    * &lt;b&gt;GitHub&lt;/b&gt;
    * <b>GitHub</b>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    section_false,
    Context::Map(HashMap::from([("person".into(), Context::Bool(false))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Shown.
    {{#person}}
      Never shown!
    {{/person}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Shown.
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    section_non_empty_list,
    Context::Map(HashMap::from([(
        "repo".into(),
        Context::List(vec![
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("resque".into())
            ),])),
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("hub".into())
            )])),
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("rip".into())
            ),]))
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#repo}}
      <b>{{name}}</b>
    {{/repo}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      <b>resque</b>
      <b>hub</b>
      <b>rip</b>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    non_false_values,
    Context::Map(HashMap::from([(
        String::from("person?"),
        Context::Map(HashMap::from([(
            String::from("name"),
            Context::String("Jon".into())
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#person?}}
      Hi {{name}}!
    {{/person?}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      Hi Jon!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    inverted_sections,
    Context::Map(HashMap::from([(
        String::from("repo"),
        Context::List(vec![])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#repo}}
      <b>{{name}}</b>
    {{/repo}}
    {{^repo}}
      No repos :(
    {{/repo}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      No repos :(
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    comments,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h1>Today{{! ignore me }}.</h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h1>Today.</h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    partials,
    Context::Map(HashMap::from([(
        String::from("names"),
        Context::List(vec![
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("Bob".into())
            )])),
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("Billy".into())
            )]))
        ])
    )])),
    HashMap::from([(
        String::from("user"),
        String::from("<strong>{{name}}</strong>\n")
    )]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Names</h2>
    {{#names}}
      {{> user}}
    {{/names}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Names</h2>
      <strong>Bob</strong>
      <strong>Billy</strong>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    set_delimiter,
    Context::Map(HashMap::from([(
        String::from("greeting"),
        Context::String("hello".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * {{greeting}}
    {{=<% %>=}}
    * <% greeting %>
    <%={{ }}=%>
    * {{ greeting }}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * hello
    * hello
    * hello
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
