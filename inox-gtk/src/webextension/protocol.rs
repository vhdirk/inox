use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebViewMessage {

    PreferredHeight(i64),
}