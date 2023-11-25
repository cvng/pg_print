/// Options for the stream.
#[derive(Default)]
pub struct Options {
    /// An integer value that, if given, is used to print lists on a single line, when they
    /// do not exceed the given margin on the right.
    pub compact_lists_margin: Option<usize>,
    /// An integer value that, if given, is used as the threshold beyond that a string
    /// literal gets splitted in successive chunks of that length.
    pub split_string_literals_threshold: Option<usize>,
}

/// Indented SQL parse tree writer.
pub struct IndentedStream {
    compact_lists_margin: Option<usize>,
    split_string_literals_threshold: Option<usize>,
    current_indent: usize,
    indentation_stack: Vec<usize>,
}

impl IndentedStream {
    pub fn new(options: Options) -> Self {
        Self {
            compact_lists_margin: options.compact_lists_margin,
            split_string_literals_threshold: options.split_string_literals_threshold,
            current_indent: 0,
            indentation_stack: Vec::new(),
        }
    }

    pub fn call(&self, sql: &str) -> String {
        sql.to_string()
    }
}

impl Default for IndentedStream {
    fn default() -> Self {
        Self::new(Options::default())
    }
}
