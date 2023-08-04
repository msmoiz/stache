use std::collections::HashMap;

mod macros;

mstest!(
    comment_stripped,
    Context::Map(HashMap::new()),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12345{{! Comment Block! }}67890
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    1234567890
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    multiline_comment,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12345{{!\n  This is a\n  multi-line comment...\n}}67890\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    1234567890\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_comment,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n{{! Comment Block! }}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_standalone_comment,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n  {{! Indented Comment Block! }}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    win_standalone_line_ending,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |\r\n{{! Standalone Comment }}\r\n|
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
    {{! I'm Still Standalone }}\n!
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    !
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    standalone_no_following_line,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    !\n  {{! I'm Still Standalone }}
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    !\n\
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    multiline_standalone,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n{{!\nSomething's going on here...\n}}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_multiline_standalone,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\n  {{!\n    Something's going on here...\n  }}\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Begin.\nEnd.\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    indented_inline,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12 {{! 34 }}\n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12 \n
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    surrounding_whitespace,
    Context::Map(HashMap::from([])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12345 {{! Comment Block! }} 67890
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    12345  67890
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

mstest!(
    variable_name_collision,
    Context::Map(HashMap::from([
        (String::from("! comment"), Context::Integer(1)),
        (String::from("! comment "), Context::Integer(2)),
        (String::from("!comment"), Context::Integer(3)),
        (String::from("comment"), Context::Integer(4))
    ])),
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    comments never show: >{{! comment }}<
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    comments never show: ><
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
