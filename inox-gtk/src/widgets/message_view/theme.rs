use log::*;

#[derive(Clone, Debug, Default)]
pub struct MessageViewTheme {
    pub html: String,
    pub css: String,
    pub part_css: Option<String>,
}

impl MessageViewTheme {
    pub fn load() -> Self {
        // load html and css (from scss)

        // TODO: theme versioning?
        debug!("theme: loading..");

        let htmlfile = gio::resources_lookup_data(
            &"/com/github/vhdirk/Inox/html/thread_view.html",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
        let html = std::str::from_utf8(&*htmlfile)
            .map(ToOwned::to_owned)
            .unwrap();

        let cssfile = gio::resources_lookup_data(
            &"/com/github/vhdirk/Inox/html/thread_view.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
        let css = std::str::from_utf8(&*cssfile)
            .map(ToOwned::to_owned)
            .unwrap();

        Self {
            html,
            css,
            part_css: None,
        }
    }
}
