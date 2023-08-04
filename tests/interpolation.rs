use std::collections::HashMap;

mod macros;

mstest!(
    no_interpolation,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello from {Mustache}!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello from {Mustache}!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    basic_interpolation,
    Context::Map(HashMap::from([(
        String::from("subject"),
        Context::String("world".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello, {{subject}}!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello, world!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    html_escaping,
    Context::Map(HashMap::from([(
        String::from("forbidden"),
        Context::String("& \" < >".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: {{forbidden}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: &amp; &quot; &lt; &gt;\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache,
    Context::Map(HashMap::from([(
        String::from("forbidden"),
        Context::String("& \" < >".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should not be HTML escaped: {{{forbidden}}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should not be HTML escaped: & \" < >\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand,
    Context::Map(HashMap::from([(
        String::from("forbidden"),
        Context::String("& \" < >".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should not be HTML escaped: {{&forbidden}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should not be HTML escaped: & \" < >\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    basic_integer_interpolation,
    Context::Map(HashMap::from([(String::from("mph"), Context::Integer(85))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{mph}} miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"85 miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_integer_interpolation,
    Context::Map(HashMap::from([(String::from("mph"), Context::Integer(85))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{{mph}}} miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"85 miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_integer_interpolation,
    Context::Map(HashMap::from([(String::from("mph"), Context::Integer(85))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{&mph}} miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"85 miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    basic_decimal_interpolation,
    Context::Map(HashMap::from([(
        String::from("power"),
        Context::Float(1.21)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{power}} jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"1.21 jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_decimal_interpolation,
    Context::Map(HashMap::from([(
        String::from("power"),
        Context::Float(1.21)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{{power}}} jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"1.21 jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_decimal_interpolation,
    Context::Map(HashMap::from([(
        String::from("power"),
        Context::Float(1.21)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{&power}} jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"1.21 jiggawatts!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    basic_null_interpolation,
    Context::Map(HashMap::from([(String::from("cannot"), Context::Null)])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{cannot}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_null_interpolation,
    Context::Map(HashMap::from([(String::from("cannot"), Context::Null)])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{{cannot}}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_null_interpolation,
    Context::Map(HashMap::from([(String::from("cannot"), Context::Null)])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{&cannot}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    basic_context_miss_interpolation,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{cannot}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_context_miss_interpolation,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{{cannot}}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_context_miss_interpolation,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I ({{&cannot}}) be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    I () be seen!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_basic_interpolation,
    Context::Map(HashMap::from([(
        String::from("person"),
        Context::Map(HashMap::from([(
            String::from("name"),
            Context::String("Joe".into())
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{person.name}}\" == \"{{#person}}{{name}}{{/person}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Joe\" == \"Joe\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_triple_mustache_interpolation,
    Context::Map(HashMap::from([(
        String::from("person"),
        Context::Map(HashMap::from([(
            String::from("name"),
            Context::String("Joe".into())
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{{person.name}}}\" == \"{{#person}}{{name}}{{/person}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Joe\" == \"Joe\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_ampersand_interpolation,
    Context::Map(HashMap::from([(
        String::from("person"),
        Context::Map(HashMap::from([(
            String::from("name"),
            Context::String("Joe".into())
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{&person.name}}\" == \"{{#person}}{{name}}{{/person}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Joe\" == \"Joe\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_arbitrary_depth,
    Context::Map(HashMap::from([(
        String::from("a"),
        Context::Map(HashMap::from([(
            String::from("b"),
            Context::Map(HashMap::from([(
                String::from("c"),
                Context::Map(HashMap::from([(
                    String::from("d"),
                    Context::Map(HashMap::from([(
                        String::from("e"),
                        Context::Map(HashMap::from([(
                            String::from("name"),
                            Context::String("Phil".into())
                        )]))
                    )]))
                )]))
            )]))
        )]))
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{a.b.c.d.e.name}}\" == \"Phil\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Phil\" == \"Phil\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_broken_chains,
    Context::Map(HashMap::from([(
        String::from("a"),
        Context::Map(HashMap::new())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{a.b.c}}\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_broken_chain_resolution,
    Context::Map(HashMap::from([
        (
            String::from("a"),
            Context::Map(HashMap::from([(
                String::from("b"),
                Context::Map(HashMap::new())
            )]))
        ),
        (
            String::from("c"),
            Context::Map(HashMap::from([(
                String::from("name"),
                Context::String("Jim".into())
            )]))
        )
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{a.b.c.name}}\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_initial_resolution,
    Context::Map(HashMap::from([
        (
            String::from("a"),
            Context::Map(HashMap::from([(
                String::from("b"),
                Context::Map(HashMap::from([(
                    String::from("c"),
                    Context::Map(HashMap::from([(
                        String::from("d"),
                        Context::Map(HashMap::from([(
                            String::from("e"),
                            Context::Map(HashMap::from([(
                                String::from("name"),
                                Context::String("Phil".into())
                            )]))
                        )]))
                    )]))
                )]))
            )]))
        ),
        (
            String::from("b"),
            Context::Map(HashMap::from([(
                String::from("c"),
                Context::Map(HashMap::from([(
                    String::from("d"),
                    Context::Map(HashMap::from([(
                        String::from("e"),
                        Context::Map(HashMap::from([(
                            String::from("name"),
                            Context::String("Phil".into())
                        )]))
                    )]))
                )]))
            )]))
        )
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#a}}{{b.c.d.e.name}}{{/a}}\" == \"Phil\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Phil\" == \"Phil\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    dotted_names_context_precedence,
    Context::Map(HashMap::from([
        (
            String::from("a"),
            Context::Map(HashMap::from([(
                String::from("b"),
                Context::Map(HashMap::new())
            )]))
        ),
        (
            String::from("b"),
            Context::Map(HashMap::from([(
                String::from("c"),
                Context::String("ERROR".into())
            )]))
        )
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#a}}{{b.c}}{{/a}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterators_basic_interpolation,
    Context::String("world".into()),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello, {{.}}!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Hello, world!\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterators_html_escaping,
    Context::String("& \" < >".into()),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: {{.}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: &amp; &quot; &lt; &gt;\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterators_triple_mustache,
    Context::String("& \" < >".into()),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: {{{.}}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: & \" < >\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterators_ampersand,
    Context::String("& \" < >".into()),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: {{&.}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    These characters should be HTML escaped: & \" < >\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterators_basic_integer_interpolation,
    Context::Integer(85),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{.}} miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"85 miles an hour!\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    interpolation_surrounding_whitespace,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{string}} |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | --- |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_surrounding_whitespace,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{{string}}} |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | --- |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_surrounding_whitespace,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{&string}} |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | --- |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    interpolation_standalone,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{string}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      ---\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_standalone,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{{string}}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      ---\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_standalone,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{&string}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      ---\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    interpolation_with_padding,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{ string }}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |---|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    triple_mustache_with_padding,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{{ string }}}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |---|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    ampersand_with_padding,
    Context::Map(HashMap::from([(
        String::from("string"),
        Context::String("---".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{& string }}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |---|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
