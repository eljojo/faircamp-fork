use pulldown_cmark::{Event, html, Parser, Tag};

/// We render some incoming markdown (such as artist/catalog text)
/// both to html as well as to plaintext stripped of any and all
/// html (which we need for the RSS feed). This is a convenience
/// struct to encapsulate the result in both formats wherever we
/// need to store it.
#[derive(Clone, Debug)]
pub struct HtmlAndStripped {
    pub html: String,
    pub stripped: String
}

pub fn to_html(markdown_text: &str) -> String {
    let parser = Parser::new(markdown_text);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

pub fn to_html_and_stripped(markdown_text: &str) -> HtmlAndStripped {
    HtmlAndStripped {
        html: to_html(markdown_text),
        stripped: to_stripped(markdown_text)
    }
}

pub fn to_stripped(markdown_text: &str) -> String {
    let parser = Parser::new(markdown_text);
    
    StrippedRenderer::new(parser).render()
}

struct StrippedRenderer<'a> {
    cursor: Cursor,
    ordered_list_item_number: Option<u64>,
    output: String,
    parser: Parser<'a, 'a>
}

enum Cursor {
    BeginOfFile,
    BeginOfLine,
    EndOfGap,
    EndOfLine
}

impl<'a> StrippedRenderer<'a> {
    fn ensure_gap(&mut self) {
        match self.cursor {
            Cursor::BeginOfFile => {}
            Cursor::BeginOfLine => {
                self.output.push('\n');
                self.cursor = Cursor::EndOfGap;
            }
            Cursor::EndOfGap => {}
            Cursor::EndOfLine => {
                self.output.push_str("\n\n");
                self.cursor = Cursor::EndOfGap;
            }
        }

    }

    fn ensure_linebreak(&mut self) {
        if let Cursor::EndOfLine = self.cursor {
            self.output.push('\n');
            self.cursor = Cursor::BeginOfLine;
        }
    }

    fn new(parser: Parser<'a, 'a>) -> StrippedRenderer<'a> {
        StrippedRenderer {
            cursor: Cursor::BeginOfFile,
            parser,
            ordered_list_item_number: None,
            output: String::new()
        }
    }

    fn render(mut self) -> String {
        while let Some(event) = self.parser.next() {
            match event {
                Event::Code(text) |
                Event::Text(text) => {
                    self.output.push_str(&text);

                    if text.ends_with('\n') {
                        self.cursor = Cursor::BeginOfLine;
                    } else {
                        self.cursor = Cursor::EndOfLine;
                    }
                }
                Event::End(tag) => self.render_tag_end(tag),
                Event::HardBreak => {
                    self.output.push('\n');
                    self.cursor = Cursor::BeginOfLine;
                }
                Event::Html(text) => {
                    // This sometimes consumes non-tag content, this we render then
                    if !text.starts_with('<') {
                        self.ensure_gap();
                        self.output.push_str(text.trim_start());

                        if text.ends_with('\n') {
                            self.cursor = Cursor::BeginOfLine;
                        } else {
                            self.cursor = Cursor::EndOfLine;
                        }
                    }
                },
                Event::Rule => {
                    self.ensure_gap();
                    self.output.push_str("----------------");
                    self.cursor = Cursor::EndOfLine
                }
                Event::SoftBreak => self.ensure_linebreak(),
                Event::Start(tag) => self.render_tag_begin(tag),
                // All these below are not enabled/supported in faircamp
                Event::FootnoteReference(_) |
                Event::TaskListMarker(_) => ()
            }
        }

        self.output
    }

    /// We pass through here after encountering an Event::Start(Tag::Image(...)).
    /// Nominally we expect an Event::Text(...) containing the image caption,
    /// followed by an Event::End(Tag::Image(...)), after which we return.
    fn render_image(&mut self) {
        while let Some(event) = self.parser.next() {
            match event {
                Event::End(Tag::Image(_type, destination, _title)) => {
                    self.output.push_str(&format!(" ({destination})"));
                    self.cursor = Cursor::EndOfLine;
                    return
                }
                Event::Text(text) => self.output.push_str(&text),
                _ => ()
            }
        }
    }

    fn render_tag_begin(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::BlockQuote |
            Tag::CodeBlock(_) |
            Tag::Heading(_, _, _) |
            Tag::Paragraph => self.ensure_gap(),
            Tag::List(ordered_list_item_number) => {
                self.ensure_linebreak();
                self.ordered_list_item_number = ordered_list_item_number;
            }
            Tag::Item => {
                self.ensure_linebreak();
                if let Some(number) = self.ordered_list_item_number {
                    self.output.push_str(&format!("{number}. "));
                    self.ordered_list_item_number = Some(number + 1);
                } else {
                    self.output.push_str("- ");
                }
            }
            Tag::Emphasis => {}
            Tag::Strong => {}
            Tag::Link(_type, _destination, _title) => {}
            Tag::Image(_type, _destination, _title) => self.render_image(),
            // All these below are not enabled/supported in faircamp
            Tag::FootnoteDefinition(_) |
            Tag::Strikethrough |
            Tag::Table(_) |
            Tag::TableHead |
            Tag::TableRow |
            Tag::TableCell => {}
        }
    }

    fn render_tag_end(&mut self, tag: Tag) {
        match tag {
            Tag::BlockQuote |
            Tag::CodeBlock(_) |
            Tag::Heading(_, _, _) |
            Tag::Item |
            Tag::Paragraph |
            Tag::Emphasis => {}
            Tag::Link(_type, destination, _title) => {
                self.output.push_str(&format!(" ({destination})"));
                self.cursor = Cursor::EndOfLine;
            }
            Tag::List(_) => {
                self.ordered_list_item_number = None;
                self.cursor = Cursor::EndOfLine;
            }
            Tag::Strong => {}
            // Never encountered here (consumed in render_image())
            Tag::Image(_type, _destination, _title) => {}
            // All these below are not enabled/supported in faircamp
            Tag::FootnoteDefinition(_) |
            Tag::Strikethrough |
            Tag::Table(_) |
            Tag::TableCell |
            Tag::TableHead |
            Tag::TableRow => ()
        }
    }
}
