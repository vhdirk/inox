use crate::core::mime::MultipartSubtype;
use chrono::Utc;
use glib::subclass::boxed::BoxedType;
use glib::GBoxed;
use gmime::traits::ContentTypeExt;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime};

use glib;
use glib::prelude::*;
use gmime;
use gmime::traits::{
    DataWrapperExt, MessageExt, MultipartExt, ObjectExt, ParserExt, PartExt, StreamCatExt,
    StreamMemExt,
};
use gmime::MessageExtManual;
use notmuch;

#[derive(Clone, Debug, GBoxed)]
#[gboxed(type_name = "inox_Message")]
pub struct Message {
    notmuch_message: notmuch::Message,
    gmime_message: gmime::Message,
}

impl Deref for Message {
    type Target = gmime::Message;

    fn deref(&self) -> &Self::Target {
        &self.gmime_message
    }
}

impl Message {
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

    /**
     * Generates a preview from the email's message body.
     *
     * If there is no body, the empty string will be returned.
     */
    pub fn preview(&self) -> String {

        // try {
        let mut body = self.get_plain_body(false);

        if body.is_none() {
            body = self.get_html_body();
        }

        dbg!("message body: {:?}", &body);
        // } catch (Error e) {
        //     try {
        //         format = TextFormat.HTML;
        //         preview = get_html_body(null);
        //     } catch (Error error) {
        //         debug("Could not generate message preview: %s\n and: %s",
        //               e.message, error.message);
        //     }
        // }

        if body.is_none() {
            "".to_string()
        } else {
            body.unwrap()
        }
        // return (preview != null)
        //     ? Geary.RFC822.Utils.to_preview_text(preview, format)
        //     : "";
    }

    pub fn get_plain_body(&self, convert_to_html: bool) -> Option<String> {
        Message::construct_body_from_mime_parts(
            &self.gmime_message.mime_part().unwrap(),
            MultipartSubtype::Unspecified,
            "plain",
            convert_to_html,
        )
    }

    pub fn get_html_body(&self) -> Option<String> {
        Message::construct_body_from_mime_parts(
            &self.gmime_message.mime_part().unwrap(),
            MultipartSubtype::Unspecified,
            "plain",
            false,
        )
    }

    pub fn date(&self) -> DateTime<Utc> {
        let date = self.notmuch_message.date();
        // TODO: verify this is UTC!
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date, 0), Utc)
    }

    fn has_body_parts(node: &gmime::Object, text_subtype: &str) -> bool {
        // Part part = new Part(node);
        // bool is_matching_part = false;

        // if (node is GMime.Multipart) {
        //     GMime.Multipart multipart = (GMime.Multipart) node;
        //     int count = multipart.get_count();
        //     for (int i = 0; i < count && !is_matching_part; i++) {
        //         is_matching_part = has_body_parts(
        //             multipart.get_part(i), text_subtype
        //         );
        //     }
        // } else if (node is GMime.Part) {
        //     Mime.DispositionType disposition = Mime.DispositionType.UNSPECIFIED;
        //     if (part.content_disposition != null) {
        //         disposition = part.content_disposition.disposition_type;
        //     }

        //     is_matching_part = (
        //         disposition != Mime.DispositionType.ATTACHMENT &&
        //         part.content_type.is_type("text", text_subtype)
        //     );
        // }
        // return is_matching_part;
        false
    }

    fn construct_body_from_mime_parts(
        node: &gmime::Object,
        container_subtype: MultipartSubtype,
        text_subtype: &str,
        to_html: bool, /*,
                       replacer: Option<InlinePartReplacerCB>*/
    ) -> Option<String> {
        // let part = Part::new(&node);
        //    throws Error {
        //     Part part = new Part(node);
        let part = node.clone().downcast::<gmime::Part>().ok();
        let content_type = part
            .as_ref()
            .and_then(|p| p.content_type())
            .or(node.content_type());

        let multipart = node.clone().downcast::<gmime::Multipart>().ok();

        //  If this is a multipart, call ourselves recursively on the children
        if let Some(mp) = multipart {
            let self_subtype = MultipartSubtype::from_content_type(content_type)
                .unwrap_or(MultipartSubtype::Mixed);

            let mut body_parts = vec![];
            let count = mp.count();
            for i in 0..count {
                let child = mp.part(i);

                let child_body = Message::construct_body_from_mime_parts(
                    &child.unwrap(),
                    self_subtype.clone(),
                    text_subtype,
                    to_html,
                );
                if let Some(body) = child_body {
                    body_parts.push(body);
                }
            }
            if body_parts.len() > 0 {
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

                if content_type.unwrap().is_type("text", text_subtype) {
                    let content = p.content().unwrap();
                    Some(Message::write_to_buffer(&content, to_html))
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

    fn write_to_buffer(data: &gmime::DataWrapper, html: bool) -> String {
        let stream = gmime::StreamMem::new();
        stream.set_owner(false);
        data.write_to_stream(&stream);

        String::from_utf8(stream.byte_array().unwrap().as_ref().to_vec()).unwrap()
    }
}
