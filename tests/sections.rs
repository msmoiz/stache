use std::collections::HashMap;

mod macros;

mstest!(
    truthy,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#boolean}}This should be rendered.{{/boolean}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"This should be rendered.\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    falsey,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(false)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#boolean}}This should be rendered.{{/boolean}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    null_is_falsey,
    Context::Map(HashMap::from([(String::from("boolean"), Context::Null)])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#boolean}}This should be rendered.{{/boolean}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
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
    \"{{#context}}Hi {{name}}.{{/context}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Hi Joe.\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    parent_contexts,
    Context::Map(HashMap::from([
        (String::from("a"), Context::String("foo".into())),
        (String::from("b"), Context::String("wrong".into())),
        (
            String::from("sec"),
            Context::Map(HashMap::from([(
                String::from("b"),
                Context::String("bar".into())
            )]))
        ),
        (
            String::from("c"),
            Context::Map(HashMap::from([(
                String::from("d"),
                Context::String("baz".into())
            )]))
        )
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#sec}}{{a}}, {{b}}, {{c.d}}{{/sec}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"foo, bar, baz\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    variable_test,
    Context::Map(HashMap::from([(
        String::from("foo"),
        Context::String("bar".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#foo}}{{.}} is {{foo}}{{/foo}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"bar is bar\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    list_contexts,
    Context::Map(HashMap::from([(String::from("tops"), Context::List(vec![
        Context::Map(HashMap::from([
            (
                String::from("tname"), Context::Map(HashMap::from([
                    (String::from("upper"), Context::String("A".into())),
                    (String::from("lower"), Context::String("a".into()))
                ]))
            ),
            (
                String::from("middles"), Context::List(vec![
                    Context::Map(HashMap::from([
                        (String::from("mname"), Context::String("1".into())),
                        (String::from("bottoms"), Context::List(vec![
                            Context::Map(HashMap::from([
                                (String::from("bname"), Context::String("x".into()))
                            ])),
                            Context::Map(HashMap::from([
                                (String::from("bname"), Context::String("y".into()))
                            ]))
                        ]))
                    ]))
                ])
            )
        ]))
    ]))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#tops}}{{#middles}}{{tname.lower}}{{mname}}.{{#bottoms}}{{tname.upper}}{{mname}}{{bname}}.{{/bottoms}}{{/middles}}{{/tops}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    a1.A1x.A1y.
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    deeply_nested_contexts,
    Context::Map(HashMap::from([
        (
            String::from("a"),
            Context::Map(HashMap::from([(
                String::from("one"),
                Context::Integer(1)
            )]))
        ),
        (
            String::from("b"),
            Context::Map(HashMap::from([(
                String::from("two"),
                Context::Integer(2)
            )]))
        ),
        (
            String::from("c"),
            Context::Map(HashMap::from([
                (String::from("three"), Context::Integer(3)),
                (
                    String::from("d"),
                    Context::Map(HashMap::from([
                        (String::from("four"), Context::Integer(4)),
                        (String::from("five"), Context::Integer(5))]
                    ))
                ),
            ]))
        ),
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#a}}\n{{one}}\n{{#b}}\n{{one}}{{two}}{{one}}\n{{#c}}\n{{one}}{{two}}{{three}}{{two}}{{one}}\n{{#d}}\n{{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}\n{{#five}}\n{{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}\n{{one}}{{two}}{{three}}{{four}}{{.}}6{{.}}{{four}}{{three}}{{two}}{{one}}\n{{one}}{{two}}{{three}}{{four}}{{five}}{{four}}{{three}}{{two}}{{one}}\n{{/five}}\n{{one}}{{two}}{{three}}{{four}}{{three}}{{two}}{{one}}\n{{/d}}\n{{one}}{{two}}{{three}}{{two}}{{one}}\n{{/c}}\n{{one}}{{two}}{{one}}\n{{/b}}\n{{one}}\n{{/a}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    1\n121\n12321\n1234321\n123454321\n12345654321\n123454321\n1234321\n12321\n121\n1\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    list,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::Map(HashMap::from([(String::from("item"), Context::Integer(1))])),
            Context::Map(HashMap::from([(String::from("item"), Context::Integer(2))])),
            Context::Map(HashMap::from([(String::from("item"), Context::Integer(3))]))
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#list}}{{item}}{{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"123\"
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
    \"{{#list}}Yay lists!{{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    doubled,
    Context::Map(HashMap::from([
        (String::from("bool"), Context::Bool(true)),
        (String::from("two"), Context::String("second".into()))
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{#bool}}\n* first\n{{/bool}}\n* {{two}}\n{{#bool}}\n* third\n{{/bool}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * first\n* second\n* third\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    nested_truthy,
    Context::Map(HashMap::from([(String::from("bool"), Context::Bool(true))])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | A B C D E |
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
    | A {{#bool}}B {{#bool}}C{{/bool}} D{{/bool}} E |
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
    [{{#missing}}Found key 'missing'!{{/missing}}]
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    []
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterator_string,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::String("a".into()),
            Context::String("b".into()),
            Context::String("c".into()),
            Context::String("d".into()),
            Context::String("e".into())
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#list}}({{.}}){{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"(a)(b)(c)(d)(e)\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterator_integer,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::Integer(1),
            Context::Integer(2),
            Context::Integer(3),
            Context::Integer(4),
            Context::Integer(5)
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#list}}({{.}}){{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"(1)(2)(3)(4)(5)\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterator_decimal,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::Float(1.1),
            Context::Float(2.2),
            Context::Float(3.3),
            Context::Float(4.4),
            Context::Float(5.5)
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#list}}({{.}}){{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"(1.1)(2.2)(3.3)(4.4)(5.5)\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    implicit_iterator_array,
    Context::Map(HashMap::from([(
        String::from("list"),
        Context::List(vec![
            Context::List(vec![
                Context::Integer(1),
                Context::Integer(2),
                Context::Integer(3),
            ]),
            Context::List(vec![
                Context::String("a".into()),
                Context::String("b".into()),
                Context::String("c".into()),
            ])
        ])
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"{{#list}}({{#.}}{{.}}{{/.}}){{/list}}\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"(123)(abc)\"
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
    \"{{#a.b.c}}Here{{/a.b.c}}\" == \"Here\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"Here\" == \"Here\"
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
    \"{{#a.b.c}}Here{{/a.b.c}}\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\" == \"\"
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
    \"{{#a.b.c}}Here{{/a.b.c}}\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \"\" == \"\"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    surrounding_whitespace,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{#boolean}}\t|\t{{/boolean}} | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | \t|\t | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    internal_whitespace,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{#boolean}} {{! Important Whitespace }}\n {{/boolean}} | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |  \n  | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_inline_sections,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{#boolean}}YES{{/boolean}}\n {{#boolean}}GOOD{{/boolean}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      YES\n GOOD\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_lines,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n{{#boolean}}\n|\n{{/boolean}}\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n|\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_standalone_lines,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n  {{#boolean}}\n|\n  {{/boolean}}\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | This Is\n|\n| A Line\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_line_endings,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n{{#boolean}}\r\n{{/boolean}}\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_previous_line,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      {{#boolean}}\n#{{/boolean}}\n/
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #\n/
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_following_line,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #{{#boolean}}\n/\n  {{/boolean}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #\n/\n\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    padding,
    Context::Map(HashMap::from([(
        String::from("boolean"),
        Context::Bool(true)
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{# boolean }}={{/ boolean }}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |=|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
