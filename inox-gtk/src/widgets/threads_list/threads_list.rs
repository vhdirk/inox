use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};

use notmuch;

use crate::core::Action;
// use crate::widgets::thread_list_cell_renderer::CellRendererThread;
use crate::core::Thread;

use super::threads_list_imp as imp;

const COLUMN_ID: u8 = 0;
const COLUMN_THREAD: u8 = 1;
const COLUMN_AUTHORS: u8 = 2;

// Wrap imp::ThreadsList into a usable gtk-rs object
glib::wrapper! {
    pub struct ThreadsList(ObjectSubclass<imp::ThreadsList>)
        @extends gtk::Widget;
}

// ThreadsList implementation itself
impl ThreadsList {
    pub fn new(sender: Sender<Action>) -> Self {
        let thread_list: Self = glib::Object::new(&[]).expect("Failed to create ThreadsList");
        let imp = imp::ThreadsList::from_instance(&thread_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ThreadsList");
        thread_list.set_vexpand(true);
        thread_list.set_vexpand_set(true);

        thread_list.setup_callbacks();

        thread_list
    }

    // ANCHOR: model
    pub fn setup_model(&self) {}
    // ANCHOR_END: model

    // ANCHOR: setup_callbacks
    pub fn setup_callbacks(&self) {
        // Get state
        let imp = imp::ThreadsList::from_instance(self);

        let sender = imp.sender.get().unwrap().clone();

        imp.column_view.model().unwrap().connect_selection_changed(
            move |model, position, n_items| {
                dbg!("Selection changed {:?} {:?}", position, n_items);

                let selection = model.selection_in_range(position, n_items);
                let (mut selection_iter, _) = gtk::BitsetIter::init_first(&selection).unwrap();

                let mut threads = vec![];

                while selection_iter.is_valid() {
                    let selection_val = selection_iter.value();
                    let threadw = model
                        .item(selection_val)
                        .unwrap()
                        .downcast::<Thread>()
                        .unwrap();
                    let thread = threadw.data().clone();
                    threads.push(thread);
                    selection_iter.next();
                }

                match threads.len() {
                    0 => {
                        sender
                            .send(Action::SelectThread(None))
                            .expect("Failed to send thread selected action");
                    }
                    1 => {
                        dbg!(
                            "Selected thread {:?} {:?}",
                            threads[0].clone(),
                            threads[0].clone().subject()
                        );

                        sender
                            .send(Action::SelectThread(Some(threads[0].clone())))
                            .expect("Failed to send thread selected action");
                    }
                    _ => {
                        sender
                            .send(Action::SelectThreads(threads))
                            .expect("Failed to send thread selected action");
                    }
                };
            },
        );
    }
    // ANCHOR_END: setup_callbacks

    // ANCHOR: setup_factory

    pub fn setup_columns(&self) {
        let imp = imp::ThreadsList::from_instance(self);

        imp.column_view.append_column(&self.setup_authors_column());
        imp.column_view.append_column(&self.setup_subject_column());
        imp.column_view
            .append_column(&self.setup_attachment_column());
    }

    fn setup_authors_column(&self) -> gtk::ColumnViewColumn {
        let imp = imp::ThreadsList::from_instance(self);

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let label = gtk::Label::new(None);
            entry.set_child(Some(&label));
            label.set_xalign(0.0);
            label.show();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let label = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Label>()
                .expect("The child has to be a `Label`.");

            label.set_label(&thread.data().authors().join(", "));
        });

        gtk::ColumnViewColumn::builder()
            .title("authors")
            .factory(&factory)
            .build()
    }

    fn setup_subject_column(&self) -> gtk::ColumnViewColumn {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let label = gtk::Label::new(None);
            entry.set_child(Some(&label));
            label.set_xalign(0.0);
            label.show();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let label = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Label>()
                .expect("The child has to be a `Label`.");

            label.set_label(&thread.data().subject());
        });

        gtk::ColumnViewColumn::builder()
            .title("subject")
            .factory(&factory)
            .build()
    }

    fn setup_attachment_column(&self) -> gtk::ColumnViewColumn {
        let icon_name = "mail-attachment-symbolic";
        let icon = gio::ThemedIcon::new(icon_name);

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let img = gtk::ImageBuilder::new().gicon(&icon).build();
            entry.set_child(Some(&img));
            img.hide();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let img = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Image>()
                .expect("The child has to be a `Image`.");

            if thread.has_attachment() {
                img.show()
            } else {
                img.hide()
            }
        });

        // Tell factory how to unbind `ThreadRow` from `Thread`
        // factory.connect_unbind(move |_, entry| {
        //     let label = entry
        //         .child()
        //         .expect("The child has to exist.")
        //         .downcast::<gtk::Label>()
        //         .expect("The child has to be a `Label`.");

        //     label.unbind();
        // });

        gtk::ColumnViewColumn::builder()
            .title("attachment")
            .factory(&factory)
            .build()
    }

    pub fn set_threads(&self, threads: notmuch::Threads) {
        let imp = imp::ThreadsList::from_instance(self);
        let model = imp::create_liststore();

        for thread in threads {
            model.append(&Thread::new(thread));
        }

        imp.selection_model.set_model(Some(&model));
    }
}
