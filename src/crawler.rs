
struct HtmlParser {
    input: String,
    max_depth: usize,
    must_contain: Option<String>
}

impl HtmlParser {
    /// Create a new parser.
    /// ```
    /// use html_parser::HtmlParser;
    /// let parser = HtmlParser::new("<html><body><p>Hello</p></body></html>", 1, None);
    /// ```
    fn new(input: String, max_depth: usize, must_contain: Option<String>) -> HtmlParser {
        HtmlParser {
            input,
            max_depth,
            must_contain
        }
    }
}
