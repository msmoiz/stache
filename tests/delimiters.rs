mod macros;
use std::collections::HashMap;

mstest!(
    pair_behavior,
    Context::Map(HashMap::from([(
        (String::from("text")),
        Context::String("Hey!".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{=<% %>=}}(<%text%>)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    (Hey!)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    special_characters,
    Context::Map(HashMap::from([(
        (String::from("text")),
        Context::String("It worked!".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ({{=[ ]=}}[text])
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    (It worked!)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    sections,
    Context::Map(HashMap::from([(String::from("section"), Context::Bool(true)), (
        (String::from("data")),
        Context::String("I got interpolated.".into())
    )])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [\n{{#section}}\n  {{data}}\n  |data|\n{{/section}}\n\n{{= | | =}}\n|#section|\n  {{data}}\n  |data|\n|/section|\n]\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [\n  I got interpolated.\n  |data|\n\n  {{data}}\n  I got interpolated.\n]\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    inverted_sections,
    Context::Map(HashMap::from([
        (String::from("section"), Context::Bool(false)),
        (String::from("data"), Context::String("I got interpolated.".into()))
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [\n{{^section}}\n  {{data}}\n  |data|\n{{/section}}\n\n{{= | | =}}\n|^section|\n  {{data}}\n  |data|\n|/section|\n]\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    [\n  I got interpolated.\n  |data|\n\n  {{data}}\n  I got interpolated.\n]\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// @todo: Partial Inheritance
// @todo: Post-Partial Behavior

mstest!(
    surrounding_whitespace,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{=@ @=}} |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |  |
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    outlying_whitespace_inline,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | {{=@ @=}}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    | \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_tag,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n{{=@ @=}}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_standalone_tag,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n  {{=@ @=}}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_line_endings,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n{{= @ @ =}}\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_previous_line,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    {{=@ @=}}\n=
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    =
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_following_line,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    =\n  {{=@ @=}}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    =\n\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    pair_with_padding,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |{{= @   @ =}}|
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ||
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
