use glib;
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

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for gdk::RGBA {
    fn to_hex(&self) -> String {
        let hex = format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            (self.red() * 255.0 / 65535.0) as u8,
            (self.green() * 255.0 / 65535.0) as u8,
            (self.blue() * 255.0 / 65535.0) as u8,
            (self.alpha() * 255.0) as u8
        );
        hex.to_string()
    }
}

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

impl EmptyOrWhitespace for glib::GString {
    fn is_empty_or_whitespace(&self) -> bool {
        self.trim().is_empty()
    }
}

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


pub fn get_tag_color_rgba(tag: &str, canvascolor: &gdk::RGBA) -> (gdk::RGBA, gdk::RGBA) {
    //TODO: get from settings
    let tags_upper_color = gdk::RGBA::from_str(&"#e5e5e5".to_string()).unwrap();
    let tags_lower_color = gdk::RGBA::from_str(&"#333333".to_string()).unwrap();
    let tags_alpha = 0.5;

    // # ifndef DISABLE_PLUGINS
    //
    // Gdk::RGBA canvas;
    // canvas.set_red_u   ( cv[0] * 65535 / 255 );
    // canvas.set_green_u ( cv[1] * 65535 / 255 );
    // canvas.set_blue_u  ( cv[2] * 65535 / 255 );
    //
    // auto clrs = astroid->plugin_manager->astroid_extension->get_tag_colors (t, rgba_to_hex (canvas));
    //
    // if (!clrs.first.empty () || !clrs.second.empty ()) {
    //   return std::make_pair (Gdk::RGBA (clrs.first), Gdk::RGBA (clrs.second));
    // }
    // # endif

    let tc = md5::compute(tag);

    /*
     * normalize the background tag color to be between upper and
     * lower, then choose light or dark font color depending on
     * luminocity of background color.
     */

    let bg = gdk::RGBA::new(
        f32::from(tc[0]) * (tags_upper_color.red() - tags_lower_color.red())
            + tags_lower_color.red(),
        f32::from(tc[1]) * (tags_upper_color.green() - tags_lower_color.green())
            + tags_lower_color.green(),
        f32::from(tc[2]) * (tags_upper_color.blue() - tags_lower_color.blue())
            + tags_lower_color.blue(),
        0.0,
    );

    let bc = gdk::RGBA::new(
        bg.red() * (65535.0) / (255.0),
        bg.green() * (65535.0) / (255.0),
        bg.blue() * (65535.0) / (255.0),
        tags_alpha * (65535.0),
    );

    let lum = ((bg.red() * tags_alpha + (1.0 - tags_alpha) * canvascolor.red()) * 0.2126
        + (bg.green() * tags_alpha + (1.0 - tags_alpha) * canvascolor.green()) * 0.7152
        + (bg.blue() * tags_alpha + (1.0 - tags_alpha) * canvascolor.blue()) * 0.0722)
        / 255.0;
    /* float avg = (bg[0] + bg[1] + bg[2]) / (3 * 255.0); */

    let fc = if lum > 0.5 {
        gdk::RGBA::from_str("#000000").unwrap()
    } else {
        gdk::RGBA::from_str("#f2f2f2").unwrap()
    };

    (fc, bc)
}

pub fn get_tag_color(tag: &str, canvascolor: &gdk::RGBA) -> (String, String) {
    let clrs = get_tag_color_rgba(&tag, &canvascolor);

    (clrs.0.to_hex(), clrs.1.to_hex())
}

pub fn concat_tags_color(
    tags: &[String],
    use_pango: bool,
    maxlen: i32,
    canvascolor: &gdk::RGBA,
) -> String {
    let mut tag_string = "".to_string();
    let mut first = true;
    let mut broken = false;
    let mut len = 0;

    let tags_alpha = 0.5;

    for t in tags {
        let mut tag = t.clone();

        if !first {
            if use_pango {
                tag_string.add_assign("<span size=\"xx-small\"> </span>");
            } else {
                tag_string.add_assign(" ");
            }
        } else {
            first = false;
        }

        let mut colors = get_tag_color(&tag, &canvascolor);

        if maxlen > 0 {
            broken = true;
            if len >= maxlen {
                break;
            }
            broken = false;

            if (len + tag.len() as i32 + 2) > maxlen {
                let cur_tag_len = tag.len() as i32;
                tag.truncate((len + cur_tag_len + 2 - maxlen) as usize);
                tag.add_assign("..");
            }

            len += tag.len() as i32 + 2;
        }

        if use_pango {
            tag_string.add_assign(&format!(
                "<span bgcolor=\"{}\" color=\"{}\"> {} </span>",
                colors.1,
                colors.0,
                glib::markup_escape_text(tag.as_str())
            ));
        } else {
            colors.1.truncate(7);
            let mut bg = gdk::RGBA::from_str(&colors.1).unwrap();

            tag_string.add_assign(&format!("<span style=\"background-color: rgba({}, {}, {}, {}); color: {} !important; white-space: pre;\"> {} </span>",
                                (bg.red() * 255.0) as u8,
                                (bg.green() * 255.0) as u8,
                                (bg.blue() * 255.0) as u8,
                                tags_alpha,
                                colors.0,
                                glib::markup_escape_text(tag.as_str())));
        }
    }
    if broken {
        tag_string.add_assign("..");
    }
    tag_string
}
