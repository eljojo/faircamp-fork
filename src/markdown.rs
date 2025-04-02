// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::indoc;
use pulldown_cmark::{
    Event,
    LinkType,
    Parser,
    Tag,
    TagEnd
};
use pulldown_cmark::html;

use crate::SiteUrl;

/// We render some incoming markdown (such as artist/catalog text)
/// both to html as well as to plaintext stripped of any and all
/// html (which we currently don't use though). This is a convenience
/// struct to encapsulate the result in both formats wherever we
/// need to store it.
#[derive(Clone, Debug)]
pub struct HtmlAndStripped {
    pub html: String,
    pub stripped: String
}

pub fn to_html(base_url: &Option<SiteUrl>, markdown_text: &str) -> String {
    let parser = Parser::new(markdown_text);
    
    let parser = parser.map(|event| match &event {
        Event::Rule => {
            let divider = indoc!(r#"
                <div class="divider">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            "#);

            Event::Html(divider.into())
        }
        Event::Start(Tag::Link { dest_url, link_type, title, .. }) => {
            'transformation: {
                if *link_type == LinkType::Autolink || *link_type == LinkType::Inline {
                    if let Some(base_url) = base_url {
                        // Links that do not purely target an anchor
                        // (e.g. #t=3m to jump to the third minute) and are
                        // not internal (do not point to the same site) are
                        // hinted to open in a new tab.
                        if !(
                            dest_url.starts_with('#') ||
                            dest_url.starts_with(base_url.without_trailing_slash())
                        ) {
                            let html = format!(r#"<a href="{dest_url}" target="_blank">{title}"#);
                            break 'transformation Event::InlineHtml(html.into());
                        }
                    }
                }

                event
            }
        }
        _ => event
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

pub fn to_html_and_stripped(base_url: &Option<SiteUrl>, markdown_text: &str) -> HtmlAndStripped {
    HtmlAndStripped {
        html: to_html(base_url, markdown_text),
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
    link_end_dest_url: Option<String>,
    output: String,
    parser: Parser<'a>
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

    fn new(parser: Parser<'a>) -> StrippedRenderer<'a> {
        StrippedRenderer {
            cursor: Cursor::BeginOfFile,
            parser,
            link_end_dest_url: None,
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
                }
                Event::InlineHtml(_) => {}
                Event::Rule => {
                    self.ensure_gap();
                    self.output.push_str("----------------");
                    self.cursor = Cursor::EndOfLine
                }
                Event::SoftBreak => self.ensure_linebreak(),
                Event::Start(tag) => self.render_tag_begin(tag),
                // All these below are not enabled/supported in faircamp
                Event::DisplayMath(_) |
                Event::FootnoteReference(_) |
                Event::InlineMath(_) |
                Event::TaskListMarker(_) => ()
            }
        }

        self.output
    }

    /// We pass through here after encountering an Event::Start(Tag::Image(...)).
    /// Nominally we expect an Event::Text(...) containing the image caption,
    /// followed by an Event::End(Tag::Image(...)), after which we return.
    fn render_image(&mut self, destination: &str) {
        while let Some(event) = self.parser.next() {
            match event {
                Event::End(TagEnd::Image) => {
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
            Tag::BlockQuote(_) |
            Tag::CodeBlock(_) |
            Tag::DefinitionList |
            Tag::DefinitionListDefinition |
            Tag::DefinitionListTitle |
            Tag::Heading { .. } |
            Tag::Paragraph => self.ensure_gap(),
            Tag::Emphasis => {}
            Tag::HtmlBlock => {}
            Tag::Image { dest_url, .. } => self.render_image(&dest_url),
            Tag::Item => {
                self.ensure_linebreak();
                if let Some(number) = self.ordered_list_item_number {
                    self.output.push_str(&format!("{number}. "));
                    self.ordered_list_item_number = Some(number + 1);
                } else {
                    self.output.push_str("- ");
                }
            }
            Tag::Link { dest_url, .. } => {
                self.link_end_dest_url = Some(dest_url.to_string());
            }
            Tag::List(ordered_list_item_number) => {
                self.ensure_linebreak();
                self.ordered_list_item_number = ordered_list_item_number;
            }
            Tag::Strong => {}
            Tag::Subscript => {}
            Tag::Superscript => {}
            // All these below are not enabled/supported in faircamp
            Tag::FootnoteDefinition(_) |
            Tag::MetadataBlock(_) |
            Tag::Strikethrough |
            Tag::Table(_) |
            Tag::TableHead |
            Tag::TableRow |
            Tag::TableCell => {}
        }
    }

    fn render_tag_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::BlockQuote(_) |
            TagEnd::CodeBlock |
            TagEnd::DefinitionList |
            TagEnd::DefinitionListDefinition |
            TagEnd::DefinitionListTitle |
            TagEnd::Heading(_) |
            TagEnd::Item |
            TagEnd::Paragraph |
            TagEnd::Emphasis => {}
            TagEnd::HtmlBlock |
            TagEnd::Link => {
                if let Some(dest_url) = self.link_end_dest_url.take() {
                    self.output.push_str(&format!(" ({dest_url})"));
                }
                self.cursor = Cursor::EndOfLine;
            }
            TagEnd::List(_) => {
                self.ordered_list_item_number = None;
                self.cursor = Cursor::EndOfLine;
            }
            TagEnd::Strong => {}
            TagEnd::Subscript => {}
            TagEnd::Superscript => {}
            // Never encountered here (consumed in render_image())
            TagEnd::Image => {}
            // All these below are not enabled/supported in faircamp
            TagEnd::FootnoteDefinition |
            TagEnd::MetadataBlock(_) |
            TagEnd::Strikethrough |
            TagEnd::Table |
            TagEnd::TableCell |
            TagEnd::TableHead |
            TagEnd::TableRow => ()
        }
    }
}
