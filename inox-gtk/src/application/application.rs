#![allow(new_without_default)]
use gio::ApplicationFlags;
use once_cell::unsync::OnceCell;
use std;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use std::path::PathBuf;

use gio::Application;
use glib::clone::Upgrade;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Receiver, Sender};
use gtk::prelude::*;
use gtk::subclass::application::GtkApplicationImpl;
use gtk::subclass::prelude::*;
use log::*;

use crate::constants;
use crate::widgets::MainWindow;
use crate::core::Thread;
use crate::core::Action;
use inox_core::models::query::{Query, Sort};
use inox_core::settings::Settings;

use super::application_imp as imp;

// use crate::api::{Station, StationRequest};
// use crate::audio::{GCastDevice, PlaybackState, Player, Song};
// use crate::config;
// use crate::database::Library;
// use crate::discover::StoreFront;
// use crate::settings::{settings_manager, Key};
// use crate::ui::{Notification, InoxApplicationWindow, View};
// use crate::utils::{Order, Sorting};

glib::wrapper! {
    pub struct InoxApplication(ObjectSubclass<imp::InoxApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl InoxApplication {
    pub fn run(settings: Rc<Settings>) {
        info!(
            "{} ({})",
            constants::APPLICATION_NAME,
            constants::APPLICATION_ID
        );

        // Create new GObject and downcast it into InoxApplication
        let app: Self = glib::Object::new(&[
            ("application-id", &Some(constants::APPLICATION_ID)),
            ("flags", &ApplicationFlags::empty()),
            ("resource-base-path", &Some(constants::RESOURCE_BASE_PATH)),
        ])
        .unwrap();

        let imp = imp::InoxApplication::from_instance(&app);

        imp.settings.replace(Some(settings));

        // Start running gtk::Application
        let args: Vec<String> = env::args().collect();
        app.run_with_args(&args);
    }

    pub fn create_window(&self) -> MainWindow {
        let imp = imp::InoxApplication::from_instance(self);
        let window = MainWindow::new(imp.sender.clone(), self.clone());

        // // Load custom styling
        // let p = gtk::CssProvider::new();
        // gtk::CssProvider::load_from_resource(&p, "/de/haeckerfelix/Shortwave/gtk/style.css");
        // gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &p, 500);

        /// Set initial view
        // window.set_view(View::Library);

        // // Setup help overlay
        // let builder = gtk::Builder::new_from_resource("/de/haeckerfelix/Shortwave/gtk/shortcuts.ui");
        // get_widget!(builder, gtk::ShortcutsWindow, shortcuts);
        // window.set_help_overlay(Some(&shortcuts));
        imp.sender.send(Action::Search("*".to_string())).unwrap();
        window.present();

        window
    }

    pub fn process_action(&self, action: Action) -> glib::Continue {
        let imp = imp::InoxApplication::from_instance(self);

        debug!("processing action {:?}", action);

        match action {
            Action::SelectTag(tag) => {
                let search = match tag {
                    Some(val) => format!("tag:\"{}\"", val),
                    None => "".to_string(),
                };
                imp.sender.send(Action::Search(search.to_owned())).unwrap()
            }
            Action::Search(search) => imp
                .sender
                .send(Action::Query(
                    Query {
                        query: search,
                        sort: Sort::NewestFirst,
                        tags_exclude: vec![],
                        omit_excluded: false,
                    }
                ))
                .unwrap(),
            Action::Query(query) => self.perform_search(&query),
            Action::SelectThread(thread_id) => self.open_thread(thread_id),
            Action::SelectThreads(thread_ids) => self.open_threads(thread_ids),

            // Action::ViewShowDiscover => imp.window.borrow().as_ref().unwrap().set_view(View::Discover),
            // Action::ViewShowLibrary => imp.window.borrow().as_ref().unwrap().set_view(View::Library),
            // Action::ViewShowPlayer => imp.window.borrow().as_ref().unwrap().set_view(View::Player),
            // Action::ViewRaise => imp.window.borrow().as_ref().unwrap().present_with_time((glib::get_monotonic_time() / 1000) as u32),
            // Action::ViewShowNotification(notification) => imp.window.borrow().as_ref().unwrap().show_notification(notification),
            // Action::PlaybackConnectGCastDevice(device) => imp.player.connect_to_gcast_device(device),
            // Action::PlaybackDisconnectGCastDevice => imp.player.disconnect_from_gcast_device(),
            // Action::PlaybackSetStation(station) => {
            //     imp.player.set_station(*station);
            //     imp.window.borrow().as_ref().unwrap().show_player_widget(imp.player.widget.clone());
            // }
            // Action::PlaybackStart => imp.player.set_playback(PlaybackState::Playing),
            // Action::PlaybackStop => imp.player.set_playback(PlaybackState::Stopped),
            // Action::PlaybackSetVolume(volume) => imp.player.set_volume(volume),
            // Action::PlaybackSaveSong(song) => imp.player.save_song(song),
            // Action::LibraryAddStations(stations) => imp.library.add_stations(stations),
            // Action::LibraryRemoveStations(stations) => imp.library.remove_stations(stations),
            // Action::SearchFor(data) => imp.storefront.search_for(data),
            // Action::SettingsKeyChanged(key) => {
            //     debug!("Settings key changed: {:?}", &key);
            //     match key {
            //         Key::ViewSorting | Key::ViewOrder => {
            //             let sorting: Sorting = Sorting::from_str(&settings_manager::get_string(Key::ViewSorting)).unwrap();
            //             let order: Order = Order::from_str(&settings_manager::get_string(Key::ViewOrder)).unwrap();
            //             imp.library.set_sorting(sorting, order);
            //         }
            //         _ => (),
            //     }
            //}
        }
        glib::Continue(true)
    }

    fn perform_search(&self, query: &Query) {
        let imp = imp::InoxApplication::from_instance(self);
        imp.window
            .get()
            .unwrap()
            .upgrade()
            .unwrap()
            .set_conversations(query);
    }

    fn open_thread(&self, thread_id: Option<String>) {
        let imp = imp::InoxApplication::from_instance(self);

        let thread = thread_id.map(|id| {
            self.open_database(notmuch::DatabaseMode::ReadOnly)
                .unwrap().find_thread_by_id(&id).unwrap().unwrap()
        });

        imp.window
            .get()
            .unwrap()
            .upgrade()
            .unwrap()
            .open_thread(thread);
    }

    fn open_threads(&self, thread_ids: Vec<String>) {
        // let imp = imp::InoxApplication::from_instance(self);
        // imp.window
        //     .get()
        //     .unwrap()
        //     .upgrade()
        //     .unwrap()
        //     .open_thread(thread);
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
