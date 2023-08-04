mod macros;
use std::collections::HashMap;

mstest_with_partials!(
    basic_behavior,
    Context::Map(HashMap::from([])),
    HashMap::from([(String::from("text"), String::from("from partial"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{>text}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"from partial\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    failed_lookup,
    Context::Map(HashMap::from([])),
    HashMap::new(),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{>text}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    context,
    Context::Map(HashMap::from([(
        String::from("text"),
        Context::String("content".into())
    )])),
    HashMap::from([(String::from("partial"), String::from("*{{text}}*"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{>partial}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"*content*\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    recursion,
    Context::Map(HashMap::from([
        (String::from("content"), Context::String("X".into())),
        (
            String::from("nodes"),
            Context::List(vec![Context::Map(HashMap::from([
                (String::from("content"), Context::String("Y".into())),
                (String::from("nodes"), Context::List(vec![]))
            ]))])
        )
    ])),
    HashMap::from([(
        String::from("node"),
        String::from("{{content}}<{{#nodes}}{{>node}}{{/nodes}}>")
    )]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{>node}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    X<Y<>>\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    surrounding_whitespace,
    Context::Map(HashMap::from([])),
    HashMap::from([(String::from("partial"), String::from("\t|\t"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{>partial}} |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | \t|\t |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    inline_indentation,
    Context::Map(HashMap::from([(
        String::from("data"),
        Context::String("|".into())
    )])),
    HashMap::from([(String::from("partial"), String::from(">\n>"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{data}}  {{> partial}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      |  >\n>\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    standalone_line_endings,
    Context::Map(HashMap::from([])),
    HashMap::from([(String::from("partial"), String::from(">"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n{{>partial}}\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n>|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    standalone_no_previous_line,
    Context::Map(HashMap::from([])),
    HashMap::from([(String::from("partial"), String::from(">\n>"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{>partial}}\n>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      >\n  >>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    standalone_no_following_line,
    Context::Map(HashMap::from([])),
    HashMap::from([(String::from("partial"), String::from(">\n>"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >\n  {{>partial}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >\n  >\n  >\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    standalone_indentation,
    Context::Map(HashMap::from([(
        String::from("content"),
        Context::String("<\n->".into())
    )])),
    HashMap::from([(
        String::from("partial"),
        String::from("|\n{{{content}}}\n|\n")
    )]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \\\n {{>partial}}\n/\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \\\n |\n <\n->\n |\n/\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest_with_partials!(
    padding_whitespace,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    HashMap::from([(String::from("partial"), String::from("[]"))]),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{> partial }}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |[]|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
