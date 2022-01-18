use crate::models::Contact;
use gmime::traits::ContentDispositionExt;
use gmime::InternetAddressExt;
use gmime::InternetAddressListExt;

use chrono::Utc;
use gmime::traits::ContentTypeExt;
use gmime::traits::StreamFilterExt;
use log::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use substring::Substring;

use chrono::{DateTime, NaiveDateTime};

use glib::prelude::*;
use gmime;
use gmime::traits::{
    DataWrapperExt, MessageExt, MultipartExt, ObjectExt, ParserExt, PartExt, StreamCatExt,
    StreamMemExt,
};
use gmime::MessageExtManual;
use notmuch;

use crate::mime::MultipartSubtype;
use crate::util::{EmptyOrWhitespace, ReduceWhiteSpace};

const MAX_PREVIEW_BYTES: usize = 128;
const UTF8_CHARSET: &str = "UTF-8";

// TODO: get from settings
const TAG_UNREAD: &str = "unread";
const TAG_ATTACHMENT: &str = "attachment";

#[derive(Clone, Debug, PartialEq)]
pub enum TextFormat {
    Plain,
    Html,
}

#[derive(Clone, Debug)]
pub struct MessageHelper {
    notmuch_message: notmuch::Message,
    gmime_message: gmime::Message,
}

impl From<gmime::InternetAddress> for Contact {
    fn from(addr: gmime::InternetAddress) -> Contact {
        Contact {
            email_address: addr.name().unwrap().into(), // TODO //addr.address().to_string(),
            name: addr.name().map(|n| n.into()),
        }
    }
}

impl MessageHelper {
    pub fn new(message: &notmuch::Message) -> Result<Self, glib::Error> {
        // create a stream to read from the file descriptor
        // TODO: proper error handling
        let stream = gmime::StreamCat::new();

        for fname in message.filenames() {
            let substream = gmime::StreamFile::open(&fname.to_string_lossy(), "r")?;
            stream.add_source(&substream);
        }

        // create a new parser object to parse the stream
        let parser = gmime::Parser::with_stream(&stream);

        // parse the message from the stream
        // TODO: what if empty?
        let gmime_message = parser.construct_message(None).unwrap();

        Ok(Self {
            notmuch_message: message.clone(),
            gmime_message,
        })
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.notmuch_message
            .tags()
            .collect::<Vec<String>>()
            .contains(&tag.to_string())
    }

    pub fn is_unread(&self) -> bool {
        self.has_tag(TAG_UNREAD)
    }

    pub fn has_attachment(&self) -> bool {
        self.has_tag(TAG_ATTACHMENT)
    }

    pub fn internet_address_list_to_contacts(
        &self,
        addr_list: &Option<gmime::InternetAddressList>,
    ) -> Vec<Contact> {
        if addr_list.is_none() {
            return vec![];
        }

        let addr_list = addr_list.as_ref().unwrap();

        let mut contacts = vec![];

        let count = addr_list.length();
        for i in 0..count {
            let addr = addr_list.address(i).unwrap();
            contacts.push(addr.into());
        }

        contacts
    }

    pub fn from_contacts(&self) -> Vec<Contact> {
        let from = self.gmime_message.from();
        self.internet_address_list_to_contacts(&from)
    }

    pub fn to_contacts(&self) -> Vec<Contact> {
        let to = self.gmime_message.to();
        self.internet_address_list_to_contacts(&to)
    }

    pub fn cc_contacts(&self) -> Vec<Contact> {
        let cc = self.gmime_message.cc();
        self.internet_address_list_to_contacts(&cc)
    }

    pub fn bcc_contacts(&self) -> Vec<Contact> {
        let bcc = self.gmime_message.bcc();
        self.internet_address_list_to_contacts(&bcc)
    }

    pub fn reply_to_contacts(&self) -> Vec<Contact> {
        let reply_to = self.gmime_message.reply_to();
        self.internet_address_list_to_contacts(&reply_to)
    }

    pub fn subject(&self) -> Option<String> {
        self.gmime_message.subject().map(|s| s.to_string())
    }

    pub fn date(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.notmuch_message.date(), 0),
            Utc,
        )
    }

    /**
     * Generates a preview from the email's message body.
     *
     * If there is no body, the empty string will be returned.
     */
    pub fn preview(&self) -> Option<String> {
        // try {
        let mut body = self.plain_body(false);
        let mut text_format = TextFormat::Plain;
        if body.is_none() {
            body = self.html_body();
            text_format = TextFormat::Html;
        }

        if body.is_none() {
            return None;
        }

        let ptext = self.to_preview_text(&body.unwrap(), text_format);

        Some(if ptext.len() > MAX_PREVIEW_BYTES {
            format!("{}{}", ptext.substring(0, MAX_PREVIEW_BYTES), "…")
        } else {
            ptext
        })
    }

    // TODO: should return error when no plain body present
    pub fn plain_body(&self, convert_to_html: bool) -> Option<String> {
        self.construct_body_from_mime_parts(
            &self.gmime_message.mime_part().unwrap(),
            MultipartSubtype::Unspecified,
            "plain",
            convert_to_html,
        )
    }

    // TODO: should return error when no html body present
    pub fn html_body(&self) -> Option<String> {
        self.construct_body_from_mime_parts(
            &self.gmime_message.mime_part().unwrap(),
            MultipartSubtype::Unspecified,
            "html",
            false,
        )
    }

    fn has_body_parts(&self, node: &gmime::Object, text_subtype: &str) -> bool {
        if let Some(multipart) = node.downcast_ref::<gmime::Multipart>() {
            let count = multipart.count();
            for i in 0..count {
                let is_matching_part =
                    self.has_body_parts(&multipart.part(i).unwrap(), text_subtype);

                if is_matching_part {
                    return true;
                }
            }
        } else if let Some(part) = node.downcast_ref::<gmime::Part>() {
            let disposition = part.content_disposition().and_then(|d| d.disposition());
            if let Some(content_type) = part.content_type() {
                return (disposition.is_none() || disposition.unwrap() != "attachment")
                    && content_type.is_type("text", text_subtype);
            }
        }
        false
    }

    pub fn has_html_body(&self) -> bool {
        return self.has_body_parts(&self.gmime_message.mime_part().unwrap(), "html");
    }

    fn construct_body_from_mime_parts(
        &self,
        node: &gmime::Object,
        container_subtype: MultipartSubtype,
        text_subtype: &str,
        to_html: bool, /*,
                       replacer: Option<InlinePartReplacerCB>*/
    ) -> Option<String> {
        let part = node.clone().downcast::<gmime::Part>().ok();
        let content_type = part
            .as_ref()
            .and_then(|p| p.content_type())
            .or_else(|| node.content_type());

        let multipart = node.clone().downcast::<gmime::Multipart>().ok();

        //  If this is a multipart, call ourselves recursively on the children
        if let Some(mp) = multipart {
            let self_subtype = MultipartSubtype::from_content_type(content_type)
                .unwrap_or(MultipartSubtype::Mixed);

            let mut body_parts = vec![];
            let count = mp.count();
            for i in 0..count {
                let child = mp.part(i);

                let child_body = self.construct_body_from_mime_parts(
                    &child.unwrap(),
                    self_subtype,
                    text_subtype,
                    to_html,
                );
                if let Some(body) = child_body {
                    body_parts.push(body);
                }
            }
            if !body_parts.is_empty() {
                return Some(body_parts.join(""));
            }
            return None;
        }

        if part.is_some() {
            let p = part.as_ref().unwrap();
            let disposition = p.disposition().map(|d| d.to_ascii_lowercase());

            // Process inline leaf parts
            if disposition.is_some() && disposition.unwrap() == "attachment" {
                None
            } else {
                // Assemble body from matching text parts, else use inline
                // part replacer *only* for inline parts and if in a mixed
                // multipart where each element is to be presented to the
                // user as structure dictates; For alternative and
                // related, the inline part is referred to elsewhere in
                // the document and it's the callers responsibility to
                // locate them

                if content_type.as_ref().unwrap().is_type("text", text_subtype) {
                    let content = p.content().unwrap();
                    Some(self.write_to_buffer(&content, &content_type.unwrap(), to_html))
                // } else if disposition.unwrap().eq(&"inline".to_owned())
                //     && container_subtype == MultipartSubtype::Mixed
                // {
                //     None
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn write_to_buffer(
        &self,
        data: &gmime::DataWrapper,
        content_type: &gmime::ContentType,
        to_html: bool,
    ) -> String {
        let stream = gmime::StreamMem::new();
        stream.set_owner(false);
        self.write_to_stream(data, content_type, &stream, to_html);

        String::from_utf8(stream.byte_array().unwrap().as_ref().to_vec()).unwrap()
    }

    fn is_utf_8(&self, charset: &str) -> bool {
        matches!(
            charset.to_uppercase().as_str(),
            "ASCII" | "US-ASCII" | "US_ASCII" | "UTF-8" | "UTF8" | "UTF_8"
        )
    }

    fn write_to_stream(
        &self,
        data: &gmime::DataWrapper,
        content_type: &gmime::ContentType,
        stream: &gmime::StreamMem,
        to_html: bool,
    ) {
        //  internal void write_to_stream(GMime.Stream destination,
        //                                   EncodingConversion conversion,
        //                                   BodyFormatting format = BodyFormatting.NONE)
        // throws Error {
        // GMime.DataWrapper? wrapper = (this.source_part != null)
        //     ? this.source_part.get_content() : null;
        // if (wrapper == null) {
        //     throw new Error.INVALID(
        //         "Could not get the content wrapper for content-type %s",
        //         content_type.to_string()
        //     );
        // }

        debug!("Write to stream: {:?}", content_type.mime_type());
        if content_type.is_type("text", "*") {
            let filter = gmime::StreamFilter::new(stream);

            // Do charset conversion if needed
            // Fallback charset per RFC 2045, Section 5.2
            let charset = content_type
                .parameter("charset")
                .unwrap_or_else(|| glib::GString::from("US-ASCII"));

            if !self.is_utf_8(&charset) {
                let filter_charset = gmime::FilterCharset::new(&charset, UTF8_CHARSET);

                //         if (filter_charset == null) {
                //             // Source charset not supported, so assume
                //             // US-ASCII
                //             filter_charset = new GMime.FilterCharset(
                //                 "US-ASCII", Geary.RFC822.UTF8_CHARSET
                //             );
                //         }

                filter.add(&filter_charset);
            }

            let flowed =
                if let Some(format) = content_type.parameter("format").map(|s| s.to_lowercase()) {
                    format == "flowed"
                } else {
                    false
                };
            let delsp =
                if let Some(format) = content_type.parameter("DelSp").map(|s| s.to_lowercase()) {
                    format == "yes"
                } else {
                    false
                };

            // Remove the CR's in any CRLF sequence since they are
            // effectively a wire encoding, unless the format requires
            // them or the content encoding is Base64 (being a binary
            // format)
            //     if ((this.source_part == null ||
            //          this.source_part.encoding != BASE64) &&
            //         !(content_type.media_subtype in CR_PRESERVING_TEXT_TYPES)) {
            //         filter.add(new GMime.FilterDos2Unix(false));
            //     }

            // if (flowed) {
            //     filter.add(
            //         new Geary.RFC822.FilterFlowed(
            //             format == BodyFormatting.HTML, delsp
            //         )
            //     );
            // }

            if to_html {
                //         if (!flowed) {
                //             filter.add(new Geary.RFC822.FilterPlain());
                //         }
                let filter_html = gmime::FilterHTML::new(
                    (gmime::ffi::GMIME_FILTER_HTML_CONVERT_URLS
                        | gmime::ffi::GMIME_FILTER_HTML_CONVERT_ADDRESSES)
                        as u32,
                    0,
                );
                filter.add(&filter_html);
                //         filter.add(new Geary.RFC822.FilterBlockquotes());
            }

            data.write_to_stream(&filter);
        //     if (wrapper.write_to_stream(filter) < 0)
        //         throw new Error.FAILED("Unable to write textual RFC822 part to filter stream");
        //     if (filter.flush() != 0)
        //         throw new Error.FAILED("Unable to flush textual RFC822 part to destination stream");
        //     if (destination.flush() != 0)
        //         throw new Error.FAILED("Unable to flush textual RFC822 part to destination");
        } else {
            // Keep as binary
            data.write_to_stream(stream);

            //     if (wrapper.write_to_stream(destination) < 0)
            //         throw new Error.FAILED("Unable to write binary RFC822 part to destination stream");
            //     if (destination.flush() != 0)
            //         throw new Error.FAILED("Unable to flush binary RFC822 part to destination");
        }
    }

    /**
     * Obtains the best preview text from a plain or HTML string.
     *
     * The given string `text` should have UNIX encoded line endings (LF),
     * rather than RFC822 (CRLF). The string returned will will have had
     * its whitespace squashed.
     */
    pub fn to_preview_text(&self, text: &str, text_format: TextFormat) -> String {
        let preview = if text_format == TextFormat::Plain {
            // TODO: pretty sure we can do all of this in a single fancy regex
            let all_lines = text.split("\n");
            let mut buf = vec![];
            let mut in_inline_pgp_header = false;
            for line in all_lines {
                if in_inline_pgp_header {
                    if line.is_empty() {
                        in_inline_pgp_header = false;
                    }
                    continue;
                }

                if line.starts_with("-----BEGIN PGP SIGNED MESSAGE-----") {
                    in_inline_pgp_header = true;
                    continue;
                }

                if line.starts_with(">") {
                    continue;
                }

                if line.starts_with("--") {
                    continue;
                }
                if line.starts_with("====") {
                    continue;
                }

                if line.starts_with("~~~~") {
                    continue;
                }

                if line.is_empty_or_whitespace() {
                    buf.push("\n");
                    continue;
                }

                buf.push(" ");
                buf.push(line);
            }

            buf.join("")
        } else {
            use crate::util::html_to_text;

            html_to_text(text, Some(false), None)
        };

        preview.reduce_whitespace().to_string()
    }

    pub fn from_names(&self) -> Vec<String> {
        let msg = &self.gmime_message;
        let from = msg.from();

        if from.is_none() {
            return vec![];
        }

        let from = from.unwrap();
        let num_from = from.length();

        let mut originators = vec![];
        for i in 0..num_from {
            // TODO: link email addresses to addressbook
            let from_address = from.address(i);
            if from_address.is_none() {
                continue;
            }

            let from_name = from_address.unwrap().name();

            // TODO: if there is no name, take email address instead?
            if from_name.is_none() {
                continue;
            }

            originators.push(from_name.unwrap().to_string());
        }

        originators
    }
}
