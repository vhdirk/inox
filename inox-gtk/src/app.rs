
#![allow(new_without_default)]
use std;
use std::env;
use std::cell::RefCell;
use std::rc::Rc;

use std::sync::Arc;
use std::path::PathBuf;

use log::*;
use glib::subclass::{self, prelude::*};
use glib::translate::*;
use glib::{Receiver, Sender};
use glib::{glib_wrapper, glib_object_wrapper, glib_object_subclass, glib_object_impl};
use gio::{self, prelude::*, ApplicationFlags, ApplicationExt};
use gio::subclass::prelude::ApplicationImpl;
use gtk::prelude::*;
use gtk::subclass::application::GtkApplicationImpl;

use crate::constants;
use crate::main_window::MainWindow;



use inox_core::settings::Settings;



// use crate::api::{Station, StationRequest};
// use crate::audio::{GCastDevice, PlaybackState, Player, Song};
// use crate::config;
// use crate::database::Library;
// use crate::discover::StoreFront;
// use crate::settings::{settings_manager, Key};
// use crate::ui::{Notification, InoxApplicationWindow, View};
// use crate::utils::{Order, Sorting};

#[derive(Debug)]
pub enum Action {
    SelectTag(Option<String>),
    Search(String),
    Query(Arc<notmuch::Query<'static>>),
    // Reload,
    // ViewShowLibrary,
    // ViewShowPlayer,
    // ViewRaise,
    // ViewShowNotification(Rc<Notification>),
    // PlaybackConnectGCastDevice(GCastDevice),
    // PlaybackDisconnectGCastDevice,
    // PlaybackSetStation(Box<Station>),
    // PlaybackStart,
    // PlaybackStop,
    // PlaybackSetVolume(f64),
    // PlaybackSaveSong(Song),
    // LibraryAddStations(Vec<Station>),
    // LibraryRemoveStations(Vec<Station>),
    // SearchFor(StationRequest), // TODO: is this neccessary?,
    // SettingsKeyChanged(Key),
}

pub struct InoxApplicationPrivate {
    sender: Sender<Action>,
    receiver: RefCell<Option<Receiver<Action>>>,

    window: RefCell<Option<MainWindow>>,
    database: RefCell<Option<Arc<notmuch::Database>>>,
    // pub player: Player,
    // pub library: Library,
    // pub storefront: StoreFront,

    settings: RefCell<Option<Rc<Settings>>>,
}

impl ObjectSubclass for InoxApplicationPrivate {
    const NAME: &'static str = "InoxApplication";
    type ParentType = gtk::Application;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        let (sender, recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let receiver = RefCell::new(Some(recv));

        let window = RefCell::new(None);
        // let player = Player::new(sender.clone());
        // let library = Library::new(sender.clone());
        // let storefront = StoreFront::new(sender.clone());

        Self {
            sender,
            receiver,
            window,
            database: RefCell::new(None),
            settings: RefCell::new(None)
        }
    }
}

// Implement GLib.Object for InoxApplication
impl ObjectImpl for InoxApplicationPrivate {
    glib_object_impl!();
}

// Implement Gtk.Application for InoxApplication
impl GtkApplicationImpl for InoxApplicationPrivate {}

// Implement Gio.Application for InoxApplication
impl ApplicationImpl for InoxApplicationPrivate {
    fn activate(&self, _app: &gio::Application) {
        debug!("gio::Application -> activate()");

        // If the window already exists,
        // present it instead creating a new one again.
        if let Some(ref window) = *self.window.borrow() {
            window.present();
            info!("Application window presented.");
            return;
        }

        // No window available -> we have to create one
        let app = ObjectSubclass::get_instance(self).downcast::<InoxApplication>().unwrap();
        let window = app.create_window();
        window.present();
        self.window.replace(Some(window));
        info!("Created application window.");


        let db = app.init_database();
        self.database.replace(Some(db.clone()));

        self.window.borrow().as_ref().unwrap().reload(db);


        // Setup action channel
        let receiver = self.receiver.borrow_mut().take().unwrap();
        receiver.attach(None, move |action| app.process_action(action));

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

// Wrap InoxApplicationPrivate into a usable gtk-rs object
glib_wrapper! {
    pub struct InoxApplication(
        Object<subclass::simple::InstanceStruct<InoxApplicationPrivate>,
        subclass::simple::ClassStruct<InoxApplicationPrivate>,
        InoxApplicationClass>)
        @extends gio::Application, gtk::Application;

    match fn {
        get_type => || InoxApplicationPrivate::get_type().to_glib(),
    }
}

impl InoxApplication {
    pub fn run(settings: Rc<Settings>) {
        info!("{} ({})", constants::APPLICATION_NAME, constants::APPLICATION_ID);
        // info!("Version: {} ({})", config::VERSION, config::PROFILE);
        // info!("Isahc version: {}", isahc::version());

        // Create new GObject and downcast it into InoxApplication
        let app = glib::Object::new(InoxApplication::static_type(),
            &[("application-id", &Some(constants::APPLICATION_ID)),
              ("flags", &ApplicationFlags::empty())])
            .unwrap()
            .downcast::<InoxApplication>()
            .unwrap();

        app.set_resource_base_path(Some("/com/github/vhdirk/Inox"));
        let self_ = InoxApplicationPrivate::from_instance(&app);

        self_.settings.replace(Some(settings));

        // Start running gtk::Application
        let args: Vec<String> = env::args().collect();
        ApplicationExtManual::run(&app, &args);
    }

    fn create_window(&self) -> MainWindow {
        let self_ = InoxApplicationPrivate::from_instance(self);
        let window = MainWindow::new(self_.sender.clone(), self.clone());

        // // Load custom styling
        // let p = gtk::CssProvider::new();
        // gtk::CssProvider::load_from_resource(&p, "/de/haeckerfelix/Shortwave/gtk/style.css");
        // gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &p, 500);

        // // Set initial view
        // window.set_view(View::Library);

        // // Setup help overlay
        // let builder = gtk::Builder::new_from_resource("/de/haeckerfelix/Shortwave/gtk/shortcuts.ui");
        // get_widget!(builder, gtk::ShortcutsWindow, shortcuts);
        // window.set_help_overlay(Some(&shortcuts));

        self_.sender.send(Action::Search("*".to_string())).unwrap();

        window
    }

    fn init_database(&self) -> Arc<notmuch::Database>{
        let self_ = InoxApplicationPrivate::from_instance(self);

        let db_path = PathBuf::from(&self_.settings.borrow().as_ref().unwrap().notmuch_config.database.path.clone());
        let database = Arc::new(notmuch::Database::open(&db_path, notmuch::DatabaseMode::ReadOnly).unwrap());

        database
    }

    fn process_action(&self, action: Action) -> glib::Continue {
        let self_ = InoxApplicationPrivate::from_instance(self);

        debug!("processing action {:?}", action);

        match action {
            Action::SelectTag(tag) => {
                let search = match tag {
                    Some(val) => format!("tag: {}", val),
                    None => "".to_string()
                };
                self_.sender.send(Action::Search(search.to_owned())).unwrap()
            },
            Action::Search(search) => self_.sender.send(Action::Query(Arc::new(notmuch::Query::create(self_.database.borrow().as_ref().unwrap().clone(), &search).unwrap()))).unwrap(),
            Action::Query(query) => self.perform_search(query)
            // Action::ViewShowDiscover => self_.window.borrow().as_ref().unwrap().set_view(View::Discover),
            // Action::ViewShowLibrary => self_.window.borrow().as_ref().unwrap().set_view(View::Library),
            // Action::ViewShowPlayer => self_.window.borrow().as_ref().unwrap().set_view(View::Player),
            // Action::ViewRaise => self_.window.borrow().as_ref().unwrap().present_with_time((glib::get_monotonic_time() / 1000) as u32),
            // Action::ViewShowNotification(notification) => self_.window.borrow().as_ref().unwrap().show_notification(notification),
            // Action::PlaybackConnectGCastDevice(device) => self_.player.connect_to_gcast_device(device),
            // Action::PlaybackDisconnectGCastDevice => self_.player.disconnect_from_gcast_device(),
            // Action::PlaybackSetStation(station) => {
            //     self_.player.set_station(*station);
            //     self_.window.borrow().as_ref().unwrap().show_player_widget(self_.player.widget.clone());
            // }
            // Action::PlaybackStart => self_.player.set_playback(PlaybackState::Playing),
            // Action::PlaybackStop => self_.player.set_playback(PlaybackState::Stopped),
            // Action::PlaybackSetVolume(volume) => self_.player.set_volume(volume),
            // Action::PlaybackSaveSong(song) => self_.player.save_song(song),
            // Action::LibraryAddStations(stations) => self_.library.add_stations(stations),
            // Action::LibraryRemoveStations(stations) => self_.library.remove_stations(stations),
            // Action::SearchFor(data) => self_.storefront.search_for(data),
            // Action::SettingsKeyChanged(key) => {
            //     debug!("Settings key changed: {:?}", &key);
            //     match key {
            //         Key::ViewSorting | Key::ViewOrder => {
            //             let sorting: Sorting = Sorting::from_str(&settings_manager::get_string(Key::ViewSorting)).unwrap();
            //             let order: Order = Order::from_str(&settings_manager::get_string(Key::ViewOrder)).unwrap();
            //             self_.library.set_sorting(sorting, order);
            //         }
            //         _ => (),
            //     }
            //}
        }
        glib::Continue(true)
    }

    fn perform_search(&self, query: Arc<notmuch::Query<'static>>) {
        let self_ = InoxApplicationPrivate::from_instance(self);
        self_.window.borrow().as_ref().unwrap().set_query(query);
    }

}







// #[derive(Debug, Clone)]
// pub enum Action {
//     RefreshAllViews,
//     RefreshEpisodesView,
//     RefreshEpisodesViewBGR,
//     RefreshShowsView,
//     // ReplaceWidget(Arc<Show>),
//     RefreshWidgetIfSame(i32),
//     // ShowWidgetAnimated,
//     // ShowShowsAnimated,
//     HeaderBarShowTile(String),
//     HeaderBarNormal,
//     HeaderBarShowUpdateIndicator,
//     HeaderBarHideUpdateIndicator,
//     // MarkAllPlayerNotification(Arc<Show>),
//     // RemoveShow(Arc<Show>),
//     // ErrorNotification(String),
//     // InitEpisode(i32),
// }

// #[derive(Clone)]
// pub struct InoxApp {
//     pub instance: gtk::Application,
//     pub builder: gtk::Builder,
//     window: RefCell<Option<Component<MainWindow>>>,
//     // overlay: gtk::Overlay,
//     pub settings: Rc<Settings>,
//     pub dbmanager: Rc<DBManager>,

//     // gio_settings: gio::Settings,
//     // content: Rc<Content>,
//     // headerbar: Rc<Header>,
//     // player: Rc<player::PlayerWidget>,
//     // sender: Sender<Action>,
//     // receiver: Receiver<Action>,
// }

// impl InoxApp {
//     pub fn new(application: &gtk::Application,
//                       settings: Rc<Settings>) -> Rc<Self> {
//         // let settings = gio::Settings::new("com.github.vhdirk.Inox");

//         // let (sender, receiver) = unbounded();

//         let builder = new_builder().unwrap();
//         let dbmanager = Rc::new(DBManager::new(&settings));


//         //let weak_s = settings.downgrade();
//         // let weak_app = application.downgrade();
//         // window.connect_delete_event(move |window, _| {
//         //     let app = match weak_app.upgrade() {
//         //         Some(a) => a,
//         //         None => return Inhibit(false),
//         //     };

//         //     // let settings = match weak_s.upgrade() {
//         //     //     Some(s) => s,
//         //     //     None => return Inhibit(false),
//         //     // };

//         //     info!("Saving window position");
//         //     //WindowGeometry::from_window(&window).write(&settings);

//         //     info!("Application is exiting");
//         //     app.quit();
//         //     Inhibit(false)
//         // });


//         // let window = gtk::ApplicationWindow::new(application);
//         // window.set_title(constants::APPLICATION_NAME);
//         // window.set_wmclass(constants::APPLICATION_CLASS, constants::APPLICATION_NAME);
//         // window.set_role(constants::APPLICATION_CLASS);
//         // window.set_default_size(800, 600);

//         // window.connect_delete_event(clone!(application, settings => move |window, _| {
//         //     // WindowGeometry::from_window(&window).write(&settings);
//         //     application.quit();
//         //     Inhibit(false)
//         // }));

//         // Create a content instance
//         // let content = Content::new(&sender).expect("Content Initialization failed.");

//         // // Create the headerbar
//         // let header = Header::new(&content, &sender);
//         // // Add the Headerbar to the window.
//         // window.set_titlebar(&header.container);

//         // // Add the content main stack to the overlay.
//         // let overlay = gtk::Overlay::new();
//         // overlay.add(&content.get_stack());

//         // let wrap = gtk::Box::new(gtk::Orientation::Vertical, 0);
//         // // Add the overlay to the main Box
//         // wrap.add(&overlay);

//         // let player = player::PlayerWidget::new(&sender);
//         // // Add the player to the main Box
//         // wrap.add(&player.action_bar);

//         // let window

//         //window.add(&wrap);

//         let app = InoxApp {
//             instance: application.clone(),
//             settings,
//             window: RefCell::new(None),
//             builder,
//             dbmanager
//             // overlay,
//             // headerbar: header,
//             // content,
//             // player,
//             // sender,
//             // receiver,
//         };

//         Rc::new(app)
//     }

//     fn init(app: &Rc<Self>) {
//         // let cleanup_date = settings::get_cleanup_date(&app.settings);
//         // Garbage collect watched episodes from the disk
//         // utils::cleanup(cleanup_date);

//         let window = relm_init::<MainWindow>(app.clone()).ok();
//         app.window.replace(window);


//         app.setup_gactions();
//         app.setup_timed_callbacks();

//         app.instance.connect_activate(clone!(app => move |_| app.activate()));


//         // Retrieve the previous window position and size.
//         // WindowGeometry::from_settings(&app.settings).apply(&app.window);

//         // Setup the Action channel
//         //gtk::timeout_add(25, clone!(app => move || app.setup_action_channel()));
//     }

//     pub fn activate(&self) {
//         // TODO: broadcast activate signal
//         let window: gtk::Window = self.builder.get_object("main_window")
//                                               .expect("Couldn't find main_window in ui file.");
//         window.show();
//         window.present();
//     }


//     fn setup_timed_callbacks(&self) {
//         // self.setup_dark_theme();
//         // self.setup_refresh_on_startup();
//         // self.setup_auto_refresh();
//     }

//     // fn setup_dark_theme(&self) {
//     //     let gtk_settings = gtk::Settings::get_default().unwrap();
//     //     let enabled = self.settings.get_boolean("dark-theme");
//     //
//     //     gtk_settings.set_property_gtk_application_prefer_dark_theme(enabled);
//     // }

//     // fn setup_refresh_on_startup(&self) {
//     //     // Update the feeds right after the Application is initialized.
//     //     let sender = self.sender.clone();
//     //     if self.settings.get_boolean("refresh-on-startup") {
//     //         info!("Refresh on startup.");
//     //         // The ui loads async, after initialization
//     //         // so we need to delay this a bit so it won't block
//     //         // requests that will come from loading the gui on startup.
//     //         gtk::timeout_add(1500, move || {
//     //             let s: Option<Vec<_>> = None;
//     //             utils::refresh(s, sender.clone());
//     //             glib::Continue(false)
//     //         });
//     //     }
//     // }

//     // fn setup_auto_refresh(&self) {
//     //     let refresh_interval = settings::get_refresh_interval(&self.settings).num_seconds() as u32;
//     //     info!("Auto-refresh every {:?} seconds.", refresh_interval);
//     //
//     //     let sender = self.sender.clone();
//     //     gtk::timeout_add_seconds(refresh_interval, move || {
//     //         let s: Option<Vec<_>> = None;
//     //         utils::refresh(s, sender.clone());
//     //
//     //         glib::Continue(true)
//     //     });
//     // }

//     /// Define the `GAction`s.
//     ///
//     /// Used in menus and the keyboard shortcuts dialog.
//     #[cfg_attr(rustfmt, rustfmt_skip)]
//     fn setup_gactions(&self) {
//         //let sender = &self.sender;
//         let _win = &self.window;
//         let _instance = &self.instance;
//         // let header = &self.headerbar;

//         // Create the `refresh` action.
//         //
//         // This will trigger a refresh of all the shows in the database.
//         // action!(win, "refresh", clone!(sender => move |_, _| {
//         //     gtk::idle_add(clone!(sender => move || {
//         //         let s: Option<Vec<_>> = None;
//         //         utils::refresh(s, sender.clone());
//         //         glib::Continue(false)
//         //     }));
//         // }));
//         // self.instance.set_accels_for_action("win.refresh", &["<primary>r"]);

//         // Create the `OPML` import action
//         // action!(win, "import", clone!(sender, win => move |_, _| {
//         //     utils::on_import_clicked(&win, &sender)
//         // }));

//         // Create the action that shows a `gtk::AboutDialog`
//         // action!(win, "about", clone!(win => move |_, _| about_dialog(&win)));

//         // Create the quit action
//         //action!(win, "quit", clone!(instance => move |_, _| instance.quit()));

//         self.instance.set_accels_for_action("win.quit", &["<primary>q"]);

//         // Create the menu action
//         // action!(win, "menu",clone!(header => move |_, _| header.open_menu()));
//         // Bind the hamburger menu button to `F10`
//         self.instance.set_accels_for_action("win.menu", &["F10"]);
//     }

//     fn setup_action_channel(&self) -> glib::Continue {
//         // if let Some(action) = self.receiver.try_recv() {
//         //     trace!("Incoming channel action: {:?}", action);
//         //     match action {
//         //         // Action::RefreshAllViews => self.content.update(),
//         //         // Action::RefreshShowsView => self.content.update_shows_view(),
//         //         // Action::RefreshWidgetIfSame(id) => self.content.update_widget_if_same(id),
//         //         // Action::RefreshEpisodesView => self.content.update_home(),
//         //         // Action::RefreshEpisodesViewBGR => self.content.update_home_if_background(),
//         //         // Action::ReplaceWidget(pd) => {
//         //         //     let shows = self.content.get_shows();
//         //         //     let mut pop = shows.borrow().populated();
//         //         //     pop.borrow_mut()
//         //         //         .replace_widget(pd.clone())
//         //         //         .map_err(|err| error!("Failed to update ShowWidget: {}", err))
//         //         //         .map_err(|_| error!("Failed ot update ShowWidget {}", pd.title()))
//         //         //         .ok();
//         //         // }
//         //         // Action::ShowWidgetAnimated => {
//         //         //     let shows = self.content.get_shows();
//         //         //     let mut pop = shows.borrow().populated();
//         //         //     pop.borrow_mut().switch_visible(
//         //         //         PopulatedState::Widget,
//         //         //         gtk::StackTransitionType::SlideLeft,
//         //         //     );
//         //         // }
//         //         // Action::ShowShowsAnimated => {
//         //         //     let shows = self.content.get_shows();
//         //         //     let mut pop = shows.borrow().populated();
//         //         //     pop.borrow_mut()
//         //         //         .switch_visible(PopulatedState::View, gtk::StackTransitionType::SlideRight);
//         //         // }
//         //         // Action::HeaderBarShowTile(title) => self.headerbar.switch_to_back(&title),
//         //         // Action::HeaderBarNormal => self.headerbar.switch_to_normal(),
//         //         // Action::HeaderBarShowUpdateIndicator => self.headerbar.show_update_notification(),
//         //         // Action::HeaderBarHideUpdateIndicator => self.headerbar.hide_update_notification(),
//         //         // Action::MarkAllPlayerNotification(pd) => {
//         //         //     let notif = mark_all_notif(pd, &self.sender);
//         //         //     notif.show(&self.overlay);
//         //         // }
//         //         // Action::RemoveShow(pd) => {
//         //         //     let notif = remove_show_notif(pd, self.sender.clone());
//         //         //     notif.show(&self.overlay);
//         //         // }
//         //         // Action::ErrorNotification(err) => {
//         //         //     error!("An error notification was triggered: {}", err);
//         //         //     let callback = || glib::Continue(false);
//         //         //     let notif = InAppNotification::new(&err, callback, || {}, UndoState::Hidden);
//         //         //     notif.show(&self.overlay);
//         //         // }
//         //         // Action::InitEpisode(rowid) => self.player.initialize_episode(rowid).unwrap(),
//         //         _ => ()
//         //     }
//         // }

//         glib::Continue(true)
//     }

//     pub fn run(settings: Rc<Settings>) {
//         let application = gtk::Application::new(Some(constants::APPLICATION_ID), ApplicationFlags::empty())
//             .expect("Application Initialization failed...");

//         application.set_resource_base_path(Some("/com/github/vhdirk/Inox"));

//         let weak_app = application.downgrade();
//         application.connect_startup(move |_| {
//             info!("GApplication::startup");
//             weak_app.upgrade().map(|application| {
//                 let mut app = Self::new(&application, settings.clone());
//                 Self::init(&mut app);
//                 info!("Init complete");
//             });
//         });

//         // Weird magic I copy-pasted that sets the Application Name in the Shell.
//         glib::set_application_name(constants::APPLICATION_NAME);
//         glib::set_prgname(Some(constants::APPLICATION_NAME));

//         // We need out own Inox icon
//         gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);

//         // Run GTK application with command line args
//         let args: Vec<String> = std::env::args().collect();
//         ApplicationExtManual::run(&application, &args);
//     }
// }


