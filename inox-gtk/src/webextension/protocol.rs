use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebViewMessage {
    CommandStackChanged,  // TODO: what's this?
    ContentLoaded,
    DocumentModified,// TODO: what's this?
    PreferredHeightChanged(i64),
    RemoteResourceLoadBlocked,// TODO: what's this?
    SelectionChanged,// TODO: what's this?
}
