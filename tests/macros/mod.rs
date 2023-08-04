/// Generates a test that compares the parsed Mustache input against the
/// expected text output. Use the following separator to frame the input and
/// expected output: `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~`.
///
/// # Examples
///
/// ```
/// mstest!(
///     simple,
///     Context::Map(
///         HashMap::from(
///             [(String::from("greeting", Context::String("hello".into())))]
///         )
///     ),
///     "
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     {{greeting}} world!
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     Hello world!
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     "
/// );
/// ```
#[macro_export]
macro_rules! mstest {
    ($name:ident, $context:expr, $test:expr) => {
        #[test]
        fn $name() {
            use indoc::indoc;
            use stache::{Context, Template};

            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let input = &components[1][1..]; // skip leading newline
            let template = Template::compile(input).unwrap();
            let rendered = template.render($context);
            let expected = &components[2][1..]; // skip leading newline
            if rendered != expected {
                panic!(
                    "\nFailed to parse template.\n{separator}Template\n{input}{separator}Expected\n{expected}{separator}Actual\n{rendered}"
                );
            }
        }
    };
}

/// Generates a test that compares the parsed Mustache input against the
/// expected text output. Use the following separator to frame the input and
/// expected output: `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~`. This variant
/// allows you to provide a map of partials as well.
///
/// # Examples
///
/// ```
/// mstest_with_partials!(
///     simple,
///     Context::Map(
///         HashMap::from(
///             [(String::from("greeting", Context::String("hello".into())))]
///         )
///     ),
///     HashMap::from(
///         [(String::from("text"), String::from("world"))]
///     ),
///     "
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     {{greeting}} {{>text}}!
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     Hello world!
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     "
/// );
/// ```
#[macro_export]
macro_rules! mstest_with_partials {
    ($name:ident, $context:expr, $partials:expr, $test:expr) => {
        #[test]
        fn $name() {
            use indoc::indoc;
            use stache::{Context, Template};

            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let input = &components[1][1..]; // skip leading newline
            let template = Template::compile(input).unwrap();
            let rendered = template.render_with_partials($context, $partials);
            let expected = &components[2][1..]; // skip leading newline
            if rendered != expected {
                panic!(
                    "\nFailed to parse template.\n{separator}Template\n{input}{separator}Expected\n{expected}{separator}Actual\n{rendered}"
                );
            }
        }
    };
}
