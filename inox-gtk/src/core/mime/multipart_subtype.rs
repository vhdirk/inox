use gmime;
use gmime::traits::ContentTypeExt;

#[derive(PartialEq, Clone, Copy)]
pub enum MultipartSubtype {
    /**
     * Used as a placeholder for no or unknown multipart subtype.
     *
     * Technically an unknown or unspecified subtype should be treated as {@link MIXED}, but there
     * are situations in code where this is useful.
     */
    Unspecified,
    /**
     * A multipart structure of mixed media.
     *
     * "Any 'multipart' subtypes that an implementation does not recognize must be treated as
     * being of subtype 'mixed'."
     *
     * See [[https://tools.ietf.org/html/rfc2046#section-5.1.3]]
     */
    Mixed,
    /**
     * A multipart structure of alternative media.
     *
     * See [[https://tools.ietf.org/html/rfc2046#section-5.1.4]]
     */
    Alternative,
    /**
     * A multipart structure of related media.
     *
     * See [[http://tools.ietf.org/html/rfc2387]]
     */
    Related,
}

impl MultipartSubtype {
    pub fn from_content_type(content_type: Option<gmime::ContentType>) -> Option<Self> {
        if content_type.is_none() {
            return None;
        }

        let content_type = content_type.as_ref().unwrap();
        if let Some(mtype) = content_type.media_type() {
            if mtype != "multipart" {
                return None;
            }
        }

        if let Some(mstype) = content_type.media_subtype() {
            match mstype.as_str() {
                "mixed" => Some(MultipartSubtype::Mixed),
                "alternative" => Some(MultipartSubtype::Alternative),
                "related" => Some(MultipartSubtype::Related),
                _ => None,
            }
        } else {
            None
        }
    }
}
