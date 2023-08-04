use std::collections::HashMap;

mod macros;

mstest!(
    falsey,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^boolean}}This should be rendered.{{/boolean}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"This should be rendered.\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    truthy,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^boolean}}This should be rendered.{{/boolean}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    null_is_falsey,
    Context::Map(HashMap::from([(String::from("null"), Context::Null)])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^null}}This should be rendered.{{/null}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"This should be rendered.\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    context,
    Context::Map(HashMap::from([(
        String::from("context"),
        Context::Map(HashMap::from([(
            String::from("name"),
            Context::String("Joe".into())
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^context}}Hi {{name}}.{{/context}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    list,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::Map(HashMap::from([(String::from("n"), Context::Integer(1))])),
            Context::Map(HashMap::from([(String::from("n"), Context::Integer(2))])),
            Context::Map(HashMap::from([(String::from("n"), Context::Integer(3))]))
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^list}}{{n}}{{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    empty_list,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^list}}Yay lists!{{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Yay lists!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    doubled,
    Context::Map(HashMap::from([
        (String::from("bool"), Context::Bool(false)),
        (String::from("two"), Context::String("second".into()))
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{^bool}}\n* first\n{{/bool}}\n* {{two}}\n{{^bool}}\n* third\n{{/bool}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * first\n* second\n* third\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    nested_falsey,
    Context::Map(HashMap::from([(
        String::from("bool"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A B C D E |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    nested_truthy,
    Context::Map(HashMap::from([(String::from("bool"), Context::Bool(true))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A {{^bool}}B {{^bool}}C{{/bool}} D{{/bool}} E |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A  E |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    context_misses,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [{{^missing}}Cannot find key 'missing'!{{/missing}}]
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [Cannot find key 'missing'!]
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_truthy,
    Context::Map(HashMap::from([(
        String::from("a"),
        Context::Map(HashMap::from([(
            String::from("b"),
            Context::Map(HashMap::from([(String::from("c"), Context::Bool(true))]))
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_falsey,
    Context::Map(HashMap::from([(
        String::from("a"),
        Context::Map(HashMap::from([(
            String::from("b"),
            Context::Map(HashMap::from([(String::from("c"), Context::Bool(false))]))
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Not Here\" == \"Not Here\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_broken_chains,
    Context::Map(HashMap::from([(
        String::from("a"),
        Context::Map(HashMap::from([]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{^a.b.c}}Not Here{{/a.b.c}}\" == \"Not Here\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Not Here\" == \"Not Here\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    surrounding_whitespace,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{^boolean}}\t|\t{{/boolean}} | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | \t|\t | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    internal_whitespace,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{^boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |  \n  | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_inline_sections,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{^boolean}}NO{{/boolean}}\n {{^boolean}}WAY{{/boolean}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      NO\n WAY\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_lines,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n{{^boolean}}\n|\n{{/boolean}}\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n|\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_indented_lines,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n  {{^boolean}}\n|\n  {{/boolean}}\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n|\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_line_endings,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n{{^boolean}}\r\n{{/boolean}}\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_previous_line,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{^boolean}}\n^{{/boolean}}\n/
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ^\n/
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_following_line,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ^{{^boolean}}\n/\n  {{/boolean}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ^\n/\n\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    padding,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{^ boolean }}={{/ boolean }}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |=|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
