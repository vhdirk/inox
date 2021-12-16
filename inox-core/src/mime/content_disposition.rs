pub enum ContentDisposition {
    /**
     * Filename parameter name.
     *
     * See [[https://tools.ietf.org/html/rfc2183#section-2.3]]
     */
    Filename = "filename",

    /**
     * Creation-Date parameter name.
     *
     * See [[https://tools.ietf.org/html/rfc2183#section-2.4]]
     */
    CreationDate = "creation-date",

    /**
     * Modification-Date parameter name.
     *
     * See [[https://tools.ietf.org/html/rfc2183#section-2.5]]
     */
    ModificationDate = "modification-date",

    /**
     * Read-Date parameter name.
     *
     * See [[https://tools.ietf.org/html/rfc2183#section-2.6]]
     */
    ReadDate = "read-date",

    /**
     * Size parameter name.
     *
     * See [[https://tools.ietf.org/html/rfc2183#section-2.7]]
     */
    Size = "size"
}


impl ContentDisposition {
        pub fn from_gmime(content_disposition: &gmime::ContentDisposition) {


        bool is_unknown;
        disposition_type = DispositionType.deserialize(content_disposition.get_disposition(),
            out is_unknown);
        is_unknown_disposition_type = is_unknown;
        original_disposition_type_string = content_disposition.get_disposition();
        params = new ContentParameters.from_gmime(content_disposition.get_parameters());
    }
    }
}