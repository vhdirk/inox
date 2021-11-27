mod about_dialog;
pub mod threads_list;
pub mod thread_view;
pub mod placeholder_pane;
pub mod message_view;
pub mod messages_view;
pub mod message_row;
pub mod main_window;
pub mod web_view;
pub use self::about_dialog::about_dialog;

pub use thread_view::ThreadView;
pub use messages_view::MessagesView;
pub use message_view::MessageView;

pub use message_row::{BaseRow, MessageRow, LoadingRow};
pub use main_window::MainWindow;
