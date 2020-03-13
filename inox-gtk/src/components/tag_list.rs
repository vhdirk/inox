use log::*;

use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use glib::Sender;

use crate::app::Action;

fn append_text_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

pub struct TagList {
    pub widget: gtk::TreeView,

    model: gtk::ListStore,
    sender: Sender<Action>,
}

impl TagList {
    pub fn new(sender: Sender<Action>) -> Self {
        let model = gtk::ListStore::new(&[String::static_type()]);
        let widget = gtk::TreeView::new_with_model(&model);
        widget.set_headers_visible(false);
        append_text_column(&widget, 0);

        Self {
            widget,
            model,
            sender,
        }
    }

    pub fn setup_signals(&self) {
        let sender = self.sender.clone();
        self.widget.get_selection().connect_changed(move |selection| {
            let (model, iter) = selection.get_selected().unwrap();
            let store = model.downcast_ref::<gtk::ListStore>().unwrap();

            let tag = if store.iter_is_valid(&iter) {
                store.get_value(&iter, 0).get().unwrap()
            } else {
                None
            };
            sender.send(Action::SelectTag(tag));
        });
    }

    pub fn set_tags(self: &Self, tags: &Vec<String>) {
        self.model.clear();
        for tag in tags {
            info!("tag {:?}", tag);
            self.add_tag(tag);
        }
    }

    pub fn add_tag(self: &Self, tag: &String) {
        let it = self.model.append();
        self.model.set_value(&it, 0, &tag.to_value());
    }
}
