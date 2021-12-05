use crate::core::Action;
use crate::widgets::MainWindow;
use gdk::{self, prelude::*};
use gio::{self, prelude::*};
use glib::{subclass::prelude::*, Receiver, Sender, WeakRef};
use gtk::{self, prelude::*, subclass::prelude::*};
use inox_core::settings::Settings;
use log::*;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct InoxApplication {
    pub sender: Sender<Action>,
    pub receiver: RefCell<Option<Receiver<Action>>>,

    pub window: OnceCell<WeakRef<MainWindow>>,
    pub database: RefCell<Option<notmuch::Database>>,
    // pub player: Player,
    // pub library: Library,
    // pub storefront: StoreFront,
    pub settings: RefCell<Option<Rc<Settings>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for InoxApplication {
    const NAME: &'static str = "InoxApplication";
    type ParentType = gtk::Application;
    type Type = super::InoxApplication;

    fn new() -> Self {
        let (sender, recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let window = OnceCell::new();
        // let player = Player::new(sender.clone());
        // let library = Library::new(sender.clone());
        // let storefront = StoreFront::new(sender.clone());

        Self {
            sender,
            receiver: RefCell::new(Some(recv)),
            window,
            database: RefCell::new(None),
            settings: RefCell::new(None),
        }
    }
}

// Implement GLib.Object for InoxApplication
impl ObjectImpl for InoxApplication {}

// Implement Gtk.Application for InoxApplication
impl GtkApplicationImpl for InoxApplication {}

// Implement Gio.Application for InoxApplication
impl ApplicationImpl for InoxApplication {
    fn startup(&self, app: &Self::Type) {
        // Load Inox GTK CSS
        let css_provider = gtk::CssProvider::new();
        gtk::StyleContext::add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        self.load_css(&css_provider, "resource:///com/github/vhdirk/Inox/inox.css");

        self.parent_startup(app);

        // let app = app.downcast_ref::<super::InoxApplication>().unwrap();
        // let imp = InoxApplication::from_instance(app);
        // let window = MainWindow::new(imp.sender.clone(), app.clone());
        // imp.window
        //     .set(window)
        //     .expect("Failed to initialize application window");
    }

    fn activate(&self, app: &Self::Type) {
        debug!("gio::Application -> activate()");
        let mut imp = InoxApplication::from_instance(app);

        // If the window already exists,
        // present it instead creating a new one again.
        if let Some(weak_window) = self.window.get() {
            let window = weak_window.upgrade().unwrap();
            window.present();
            info!("Application window presented.");
            return;
        }

        // No window available -> we have to create one
        let window = app.create_window();
        let _ = self.window.set(window.downgrade());
        info!("Created application window.");

        let db = app.init_database();
        imp.database.borrow_mut().replace(db.clone());

        // Setup action channel
        let receiver = self.receiver.borrow_mut().take().unwrap();
        let capp = app.clone();
        receiver.attach(None, move |action| capp.process_action(action));

        // Setup settings signal (we get notified when a key gets changed)
        // self.settings.connect_changed(clone!(@strong self.sender as sender => move |_, key_str| {
        //     let key: Key = Key::from_str(key_str).unwrap();
        //     send!(sender, Action::SettingsKeyChanged(key));
        // }));

        // List all setting keys
        // settings_manager::list_keys();

        // Small workaround to update every view to the correct sorting/order.
        // send!(self.sender, Action::SettingsKeyChanged(Key::ViewSorting));
    }
}

impl InoxApplication {
    pub fn load_css(&self, css_provider: &gtk::CssProvider, resource_uri: &str) {
        css_provider.connect_parsing_error(move |provider, section, error| {
            let start = section.start_location();
            let end = section.end_location();
            warn!(
                "Error parsing {:?}:{:?}-{:?}: {:?}",
                section.file(),
                start,
                end,
                error.message()
            );
        });
        let file = gio::File::for_uri(resource_uri);
        css_provider.load_from_file(&file);
    }
}
