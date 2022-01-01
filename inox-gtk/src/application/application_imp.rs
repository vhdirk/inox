use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

use futures::Future;
use log::*;
use once_cell::sync::{Lazy, OnceCell};

use gdk::{self, prelude::*};
use gio::{self, prelude::*};
use glib::subclass::Signal;
use glib::{clone, subclass::prelude::*, Receiver, Sender, WeakRef};
use gtk::{self, prelude::*, subclass::prelude::*};

use inox_core::models::Query;
use inox_core::protocol::mail_service::*;
use inox_core::settings::Settings;

use crate::core::glib_rpc_client;
use crate::core::Action;
use crate::widgets::MainWindow;

pub struct RpcConnection {
    pub socket_connection: gio::SocketConnection,
    pub mail_client: MailServiceClient,
}

pub struct InoxApplication {
    pub sender: Sender<Action>,
    pub receiver: RefCell<Option<Receiver<Action>>>,

    pub window: OnceCell<WeakRef<MainWindow>>,
    pub socket_client: gio::SocketClient,
    pub rpc: RefCell<Option<RpcConnection>>,
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

        Self {
            sender,
            receiver: RefCell::new(Some(recv)),
            window,
            settings: RefCell::new(None),
            socket_client: gio::SocketClient::new(),
            rpc: RefCell::new(None),
        }
    }
}

// Implement GLib.Object for InoxApplication
impl ObjectImpl for InoxApplication {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "core-connected",
                // Types of the values which will be sent to the signal handler
                &[bool::static_type().into()],
                // Type of the value the signal handler sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}

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
        self.load_css(
            &css_provider,
            "resource:///com/github/vhdirk/Inox/gtk/inox.css",
        );

        self.parent_startup(app);
    }

    fn activate(&self, app: &Self::Type) {
        debug!("gio::Application -> activate()");
        let mut this = InoxApplication::from_instance(app);

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

        app.setup_connection();

        // Setup action channel
        let receiver = self.receiver.borrow_mut().take().unwrap();
        let capp = app.clone();
        receiver.attach(None, move |action| capp.process_action(action));

        let capp = app.clone();
        app.connect_local("core-connected", false, move |args| {
            let this = InoxApplication::from_instance(&capp);
            let connected = args[1]
                .get::<bool>()
                .expect("The value needs to be of type `bool`.");

            if connected {
                this.sender.send(Action::Search("*".to_string())).unwrap();
            }
            None
        });

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

    pub async fn connect_core(&self) -> Result<(), glib::Error> {
        let address = gio::UnixSocketAddress::new(
            &self
                .settings
                .borrow()
                .as_ref()
                .unwrap()
                .inox_config
                .socket_path,
        );
        let res = self.socket_client.connect_future(&address).await;

        if let Ok(connection) = res {
            let channel = glib_rpc_client::connect(connection.clone()).unwrap();
            let mail_client: MailServiceClient = channel.clone().into();

            let rpc = RpcConnection {
                socket_connection: connection,
                mail_client,
            };

            self.rpc.replace(Some(rpc));
            Ok(())
        } else {
            Err(res.err().unwrap())
        }
    }

    pub fn perform_search(&self, query: Query) {
        let inst = self.instance();

        let ctx = glib::MainContext::default();
        ctx.with_thread_default(clone!(@weak inst => move || {
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@weak inst => async move {
                let this = Self::from_instance(&inst);

                let query_client = this.rpc.borrow().as_ref().unwrap().mail_client.clone();
                let conversations = query_client.query_search_conversations(query).await;

                this.window
            .get()
            .unwrap()
            .upgrade()
            .unwrap()
            .set_conversations(&conversations.unwrap().conversations);
            }));
        }));

        // self.rpc.borrow().as_ref().unwrap().query_client.conversations(query);
    }

    pub fn open_conversation(&self, conversation_id: Option<String>) {
        // let conversation = conversation_id.map(|id| {
        //     self.open_database(notmuch::DatabaseMode::ReadOnly)
        //         .unwrap()
        //         .find_thread_by_id(&id)
        //         .unwrap()
        //         .unwrap()
        // });

        // imp.window
        //     .get()
        //     .unwrap()
        //     .upgrade()
        //     .unwrap()
        //     .open_conversation(conversation);
    }
}
