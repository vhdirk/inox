use md5;
use once_cell::sync::Lazy;
use regex;
use regex::Regex;
use std::borrow::Cow;
use std::ops::AddAssign;
use std::str::FromStr;
use tl;

/** The end-of-string character, NUL. */
const EOS: &str = "\0";

/** A regex that matches one or more whitespace or non-printing chars. */
static WS_OR_NP: Lazy<Regex> = Lazy::new(|| Regex::new("[[:space:][:cntrl:]]+").unwrap());

pub trait EmptyOrWhitespace {
    fn is_empty_or_whitespace(&self) -> bool;
}

impl EmptyOrWhitespace for &str {
    fn is_empty_or_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

impl EmptyOrWhitespace for String {
    fn is_empty_or_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

// impl EmptyOrWhitespace for glib::GString {
//     fn is_empty_or_whitespace(&self) -> bool {
//         self.trim().is_empty()
//     }
// }

pub trait ReduceWhiteSpace {
    /**
     * Removes redundant white space and non-printing characters.
     *
     * @return the input string /str/, modified so that any non-printing
     * characters are converted to spaces, all consecutive spaces are
     * coalesced into a single space, and stripped of leading and trailing
     * white space.
     */
    fn reduce_whitespace(&self) -> String;
}

impl ReduceWhiteSpace for String {
    fn reduce_whitespace(&self) -> String {
        WS_OR_NP.replace_all(self, " ").trim().to_owned()
    }
}

/**
 * Does a very approximate conversion from HTML to text.
 *
 * This does more than stripping tags -- it inserts line breaks where
 * appropriate, decodes entities, etc. Note the full string is parsed
 * by libxml's HTML parser to create a DOM-like tree representation,
 * which is then walked, so this function can be somewhat
 * computationally expensive.
 */
pub fn html_to_text<'s>(html: &'s str,
                    include_blockquotes: Option<bool>,
                    encoding: Option<&str>) -> String {
    let include_blockquotes = include_blockquotes.unwrap_or(true);
    let encoding = encoding.unwrap_or("UTF8");

    let dom = tl::parse(html, tl::ParserOptions::default());

    recurse_html_nodes_for_text(dom.parser(), dom.children().iter().cloned(), include_blockquotes)
}

fn recurse_html_nodes_for_text<I: Iterator<Item=tl::NodeHandle>>(parser: &tl::Parser<'_>,
                                   nodes: I,
                                   include_blockquotes: bool) -> String {
    let mut text = vec![];

    for node in nodes {
        match node.get(parser) {
            Some(tl::Node::Tag(tag)) => {
                let name = tag.name();
                if include_blockquotes || name.as_utf8_str() != "blockquote" {

                    if matches!(name.as_utf8_str().as_ref(), "img") {
                        if let Some(Some(alt_text)) = tag.attributes().raw.get(&tl::Bytes::from("alt")) {
                            text.push(alt_text.as_utf8_str().to_string());
                        }
                    }

                    if !matches!(name.as_utf8_str().as_ref(), "base" | "link" | "meta" | "head" | "script" | "style" | "template") {
                        text.push(recurse_html_nodes_for_text(parser, tag.children(), include_blockquotes));
                    }

                    if matches!(name.as_utf8_str().as_ref(), "dt" | "dd" | "img" | "td" | "th") {
                        text.push(" ".to_string());
                    }

                    if matches!(name.as_utf8_str().as_ref(), "address" | "blockquote" | "br" | "caption" | "center" | "div" | "dt" | "embed" | "form" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "hr" | "iframe" | "li" | "map" | "menu" | "noscript" | "object" | "p" | "pre" | "tr") {
                        text.push("\n".to_string());
                    }

                }
            }
            Some(tl::Node::Raw(bytes)) => {
                text.push(bytes.as_utf8_str().to_string())
            }
            _ => {}

        }
    }
    text.join("")
}
